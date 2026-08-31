use plychart::ChartInteraction;
use proptest::prelude::*;

proptest! {
    #[test]
    fn price_to_y_linear_never_nan(
        price in -1e10f64..1e10,
        min_p in -1e10f64..1e10,
        max_p in -1e10f64..1e10,
    ) {
        let i = ChartInteraction::new();
        let y = i.price_to_y_linear(price, min_p, max_p);
        prop_assert!(y.is_finite(), "price_to_y_linear returned {} for price={}, min={}, max={}", y, price, min_p, max_p);
    }

    #[test]
    fn price_to_y_log_never_nan(
        price in 0.0001f64..1e10,
        min_p in 0.0001f64..1e10,
        max_p in 0.0001f64..1e10,
    ) {
        let i = ChartInteraction::new();
        let y = i.price_to_y_log(price, min_p, max_p);
        prop_assert!(y.is_finite(), "price_to_y_log returned {} for price={}, min={}, max={}", y, price, min_p, max_p);
    }

    #[test]
    fn price_to_y_linear_monotonic(
        p1 in -1e10f64..1e10,
        p2 in -1e10f64..1e10,
        min_p in -1e10f64..1e10,
        max_p in -1e10f64..1e10,
    ) {
        let i = ChartInteraction::new();
        let y1 = i.price_to_y_linear(p1, min_p, max_p);
        let y2 = i.price_to_y_linear(p2, min_p, max_p);
        if p1 > p2 {
            prop_assert!(y1 <= y2 + 1e-6, "monotonicity violated: p1={} > p2={} but y1={} > y2={}", p1, p2, y1, y2);
        }
    }

    #[test]
    fn index_to_x_within_bounds(
        index in 0usize..10000,
        visible_count in 1usize..10000,
    ) {
        prop_assume!(index < visible_count);
        let i = ChartInteraction::new();
        let x = i.index_to_x(index, visible_count);
        let area = i.price_area();
        prop_assert!(
            x >= area.x && x <= area.x + area.w,
            "index_to_x({}) = {} out of bounds [{}, {}] (visible_count={})",
            index, x, area.x, area.x + area.w, visible_count
        );
    }

    #[test]
    fn visible_range_start_le_end(
        start in 0usize..10000,
        count in 1usize..10000,
        total in 1usize..10000,
    ) {
        let mut i = ChartInteraction::new();
        i.viewport.start = start;
        i.viewport.count = count;
        let (s, e) = i.visible_range(total);
        prop_assert!(s <= e, "visible_range({}) returned start={} > end={}", total, s, e);
    }

    #[test]
    fn visible_range_end_le_total(
        start in 0usize..10000,
        count in 1usize..10000,
        total in 1usize..10000,
    ) {
        let mut i = ChartInteraction::new();
        i.viewport.start = start;
        i.viewport.count = count;
        let (_s, e) = i.visible_range(total);
        prop_assert!(e <= total, "visible_range({}) returned end={} > total={}", total, e, total);
    }

    #[test]
    fn on_wheel_preserves_count_bounds(
        start in 0usize..10000,
        count in 10usize..10000,
        total in 10usize..10000,
        delta_y in -100.0f64..100.0,
    ) {
        let mut i = ChartInteraction::new();
        i.viewport.start = start.min(total.saturating_sub(1));
        i.viewport.count = count.min(total);
        i.on_wheel(delta_y, total);
        prop_assert!(i.viewport.count >= 10, "on_wheel set count={} < 10", i.viewport.count);
        prop_assert!(i.viewport.count <= total, "on_wheel set count={} > total={}", i.viewport.count, total);
    }

    #[test]
    fn on_wheel_preserves_start_bounds(
        start in 0usize..10000,
        count in 10usize..10000,
        total in 10usize..10000,
        delta_y in -100.0f64..100.0,
    ) {
        let mut i = ChartInteraction::new();
        i.viewport.start = start.min(total.saturating_sub(1));
        i.viewport.count = count.min(total);
        i.on_wheel(delta_y, total);
        prop_assert!(i.viewport.start + i.viewport.count <= total,
            "on_wheel: start={} + count={} > total={}", i.viewport.start, i.viewport.count, total);
    }

    #[test]
    fn on_mouse_drag_preserves_start_bounds(
        drag_start in 0usize..10000,
        count in 10usize..10000,
        total in 10usize..10000,
        x in -5000.0f64..5000.0,
    ) {
        let mut i = ChartInteraction::new();
        i.viewport.start = drag_start.min(total.saturating_sub(1));
        i.viewport.count = count.min(total);
        i.dragging = true;
        i.drag_start_x = 400.0;
        i.drag_start_viewport_start = drag_start;
        i.on_mouse_drag(x, total);
        prop_assert!(i.viewport.start + i.viewport.count <= total,
            "on_mouse_drag: start={} + count={} > total={}", i.viewport.start, i.viewport.count, total);
    }

    #[test]
    fn candle_index_at_mouse_returns_valid_index(
        visible_count in 1usize..10000,
        mouse_x in 60.0f64..784.0,
    ) {
        let mut i = ChartInteraction::new();
        i.mouse = plychart::interaction::MousePosition { x: mouse_x, y: 100.0, in_chart: true };
        if let Some(idx) = i.candle_index_at_mouse(visible_count) {
            prop_assert!(idx < visible_count, "candle_index_at_mouse({}) returned {} >= visible_count", visible_count, idx);
        }
    }

    #[test]
    fn on_pinch_preserves_count_bounds(
        start in 0usize..10000,
        count in 10usize..10000,
        total in 10usize..10000,
        scale in 0.01f64..100.0,
    ) {
        let mut i = ChartInteraction::new();
        i.viewport.start = start.min(total.saturating_sub(1));
        i.viewport.count = count.min(total);
        i.on_pinch(scale, total);
        prop_assert!(i.viewport.count >= 10, "on_pinch set count={} < 10", i.viewport.count);
        prop_assert!(i.viewport.count <= total, "on_pinch set count={} > total={}", i.viewport.count, total);
        prop_assert!(i.viewport.start + i.viewport.count <= total,
            "on_pinch: start={} + count={} > total={}", i.viewport.start, i.viewport.count, total);
    }

    #[test]
    fn price_to_y_linear_at_min_equals_bottom(
        min_p in -1e10f64..1e10,
        delta in 0.0001f64..1e6,
    ) {
        let i = ChartInteraction::new();
        let area = i.price_area();
        let y = i.price_to_y_linear(min_p, min_p, min_p + delta);
        let expected_bottom = area.y + area.h;
        prop_assert!(
            (y - expected_bottom).abs() < 1.0,
            "price=min_p should map to bottom: y={} expected~{}", y, expected_bottom
        );
    }
}
