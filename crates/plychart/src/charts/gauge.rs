//! Gauge chart renderer.

#[cfg(target_arch = "wasm32")]
pub fn draw(
    ctx: &web_sys::CanvasRenderingContext2d,
    value: f64,
    max: f64,
    color: &str,
    area: &crate::types::ChartArea,
) {
    let cx = area.x + area.w / 2.0;
    let cy = area.y + area.h * 0.8;
    let radius = (area.w / 2.0 - 20.0).min(area.h * 0.7);

    // Background arc
    ctx.set_stroke_style(&"rgba(255,255,255,0.1)".into());
    ctx.set_line_width(8.0);
    ctx.begin_path();
    ctx.arc(cx, cy, radius, std::f64::consts::PI, 0.0)
        .unwrap_or_default();
    ctx.stroke();

    // Value arc
    let ratio = (value / max).clamp(0.0, 1.0);
    ctx.set_stroke_style(&color.into());
    ctx.set_line_width(8.0);
    ctx.begin_path();
    ctx.arc(
        cx,
        cy,
        radius,
        std::f64::consts::PI,
        std::f64::consts::PI + ratio * std::f64::consts::PI,
    )
    .unwrap_or_default();
    ctx.stroke();

    // Value text
    ctx.set_fill_style(&"#ffffff".into());
    ctx.set_font("bold 16px monospace");
    let _ = ctx.fill_text(&format!("{:.1}", value), cx - 20.0, cy);
}

#[cfg(test)]
mod tests {
    use plycore::ChartArea;

    const AREA: ChartArea = ChartArea {
        x: 50.0,
        y: 50.0,
        w: 200.0,
        h: 150.0,
    };

    fn gauge_geometry(value: f64, max: f64, area: ChartArea) -> (f64, f64, f64, f64) {
        let cx = area.x + area.w / 2.0;
        let cy = area.y + area.h * 0.8;
        let radius = (area.w / 2.0 - 20.0).min(area.h * 0.7);
        let ratio = (value / max).clamp(0.0, 1.0);
        (cx, cy, radius, ratio)
    }

    #[test]
    fn zero_value() {
        let (cx, cy, radius, ratio) = gauge_geometry(0.0, 100.0, AREA);
        assert!(cx.is_finite());
        assert!(cy.is_finite());
        assert!(radius.is_finite());
        assert_eq!(ratio, 0.0);
    }

    #[test]
    fn max_value() {
        let (_, _, _, ratio) = gauge_geometry(100.0, 100.0, AREA);
        assert_eq!(ratio, 1.0);
    }

    #[test]
    fn normal_value() {
        let (_, _, _, ratio) = gauge_geometry(50.0, 100.0, AREA);
        assert!((ratio - 0.5).abs() < 1e-10);
    }

    #[test]
    fn value_exceeds_max_clamps() {
        let (_, _, _, ratio) = gauge_geometry(150.0, 100.0, AREA);
        assert_eq!(ratio, 1.0, "ratio clamps to 1.0");
    }

    #[test]
    fn negative_value_clamps_to_zero() {
        let (_, _, _, ratio) = gauge_geometry(-10.0, 100.0, AREA);
        assert_eq!(ratio, 0.0, "ratio clamps to 0.0");
    }

    #[test]
    fn radius_computed_correctly() {
        let area = ChartArea {
            x: 0.0,
            y: 0.0,
            w: 200.0,
            h: 100.0,
        };
        let (_, _, radius, _) = gauge_geometry(50.0, 100.0, area);
        let expected = (200.0_f64 / 2.0 - 20.0).min(100.0 * 0.7);
        assert!((radius - expected).abs() < 1e-10);
    }

    #[test]
    fn center_x_at_area_center() {
        let (cx, _, _, _) = gauge_geometry(0.0, 100.0, AREA);
        assert!((cx - (AREA.x + AREA.w / 2.0)).abs() < 1e-10);
    }

    #[test]
    fn center_y_at_80_percent_height() {
        let (_, cy, _, _) = gauge_geometry(0.0, 100.0, AREA);
        assert!((cy - (AREA.y + AREA.h * 0.8)).abs() < 1e-10);
    }

    #[test]
    fn arc_end_angle_range() {
        let pi = std::f64::consts::PI;
        for ratio in [0.0, 0.25, 0.5, 0.75, 1.0] {
            let end_angle = pi + ratio * pi;
            assert!(end_angle >= pi);
            assert!(end_angle <= 2.0 * pi);
        }
    }

    #[test]
    fn small_area_no_nan() {
        let tiny = ChartArea {
            x: 0.0,
            y: 0.0,
            w: 5.0,
            h: 5.0,
        };
        let (cx, cy, radius, ratio) = gauge_geometry(50.0, 100.0, tiny);
        assert!(cx.is_finite());
        assert!(cy.is_finite());
        assert!(radius.is_finite());
        assert!(ratio.is_finite());
    }
}
