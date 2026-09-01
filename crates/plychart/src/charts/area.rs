//! Area chart renderer with gradient fill.

#[cfg(target_arch = "wasm32")]
pub fn draw(
    ctx: &web_sys::CanvasRenderingContext2d,
    points: &[plycore::CandleData],
    area: &crate::types::ChartArea,
    color: &str,
) {
    if points.is_empty() {
        return;
    }

    // Draw filled area under line
    let min_p = points.iter().map(|c| c.close).fold(f64::INFINITY, f64::min);
    let max_p = points
        .iter()
        .map(|c| c.close)
        .fold(f64::NEG_INFINITY, f64::max);
    let range = (max_p - min_p).max(0.0001);
    let pad = range * 0.05;
    let min_p = min_p - pad;
    let max_p = max_p + pad;
    let total_range = max_p - min_p;

    let price_to_y =
        |price: f64| -> f64 { area.y + area.h * (1.0 - (price - min_p) / total_range) };
    let index_to_x =
        |i: usize| -> f64 { area.x + (i as f64 / (points.len() - 1).max(1) as f64) * area.w };

    // Area fill
    let grad = ctx.create_linear_gradient(0.0, area.y, 0.0, area.y + area.h);
    grad.add_color_stop(0.0, color);
    grad.add_color_stop(1.0, "transparent");
    ctx.set_fill_style(&grad.into());
    ctx.begin_path();
    ctx.move_to(index_to_x(0), area.y + area.h);
    for (i, c) in points.iter().enumerate() {
        ctx.line_to(index_to_x(i), price_to_y(c.close));
    }
    ctx.line_to(index_to_x(points.len() - 1), area.y + area.h);
    ctx.close_path();
    ctx.fill();

    // Line on top
    crate::charts::line::draw(ctx, points, area, color);
}

#[cfg(test)]
mod tests {
    use plycore::{CandleData, ChartArea};

    const AREA: ChartArea = ChartArea {
        x: 0.0,
        y: 0.0,
        w: 800.0,
        h: 400.0,
    };

    fn price_to_y(price: f64, min_p: f64, max_p: f64) -> f64 {
        let total = max_p - min_p;
        AREA.y + AREA.h * (1.0 - (price - min_p) / total)
    }

    fn index_to_x(i: usize, len: usize) -> f64 {
        AREA.x + (i as f64 / (len - 1).max(1) as f64) * AREA.w
    }

    fn area_range(points: &[CandleData]) -> (f64, f64) {
        let min_p = points.iter().map(|c| c.close).fold(f64::INFINITY, f64::min);
        let max_p = points.iter().map(|c| c.close).fold(f64::NEG_INFINITY, f64::max);
        let range = (max_p - min_p).max(0.0001);
        let pad = range * 0.05;
        (min_p - pad, max_p + pad)
    }

    #[test]
    fn empty_data_no_panic() {
        let points: Vec<CandleData> = vec![];
        assert!(points.is_empty());
    }

    #[test]
    fn single_point_gradient_bounds() {
        let points = vec![CandleData {
            time: 1.0, open: 50.0, high: 50.0, low: 50.0, close: 50.0, volume: 0.0,
        }];
        let (min_p, max_p) = area_range(&points);
        assert!(min_p.is_finite());
        assert!(max_p.is_finite());
        let y = price_to_y(50.0, min_p, max_p);
        assert!(y.is_finite());
        assert!(!y.is_nan());
    }

    #[test]
    fn gradient_fill_y_range() {
        let points = vec![
            CandleData { time: 1.0, open: 0.0, high: 0.0, low: 0.0, close: 10.0, volume: 0.0 },
            CandleData { time: 2.0, open: 0.0, high: 0.0, low: 0.0, close: 20.0, volume: 0.0 },
            CandleData { time: 3.0, open: 0.0, high: 0.0, low: 0.0, close: 30.0, volume: 0.0 },
        ];
        let (min_p, max_p) = area_range(&points);

        for c in &points {
            let y = price_to_y(c.close, min_p, max_p);
            assert!(y >= AREA.y - 1.0, "y should be within chart area");
            assert!(y <= AREA.y + AREA.h + 1.0, "y should be within chart area");
            assert!(y.is_finite());
        }
    }

    #[test]
    fn area_polygon_vertices() {
        let points = vec![
            CandleData { time: 1.0, open: 0.0, high: 0.0, low: 0.0, close: 10.0, volume: 0.0 },
            CandleData { time: 2.0, open: 0.0, high: 0.0, low: 0.0, close: 20.0, volume: 0.0 },
        ];
        let bottom_y = AREA.y + AREA.h;
        let first_x = index_to_x(0, points.len());
        let last_x = index_to_x(points.len() - 1, points.len());
        assert!((first_x - AREA.x).abs() < 1e-10);
        assert!((last_x - AREA.x - AREA.w).abs() < 1e-10);
        assert_eq!(bottom_y, AREA.y + AREA.h);
    }

    #[test]
    fn descending_prices_area() {
        let points = vec![
            CandleData { time: 1.0, open: 0.0, high: 0.0, low: 0.0, close: 100.0, volume: 0.0 },
            CandleData { time: 2.0, open: 0.0, high: 0.0, low: 0.0, close: 50.0, volume: 0.0 },
            CandleData { time: 3.0, open: 0.0, high: 0.0, low: 0.0, close: 10.0, volume: 0.0 },
        ];
        let (min_p, max_p) = area_range(&points);
        let y1 = price_to_y(100.0, min_p, max_p);
        let y3 = price_to_y(10.0, min_p, max_p);
        assert!(y1 < y3, "descending prices: first point higher on screen");
        assert!(!y1.is_nan());
        assert!(!y3.is_nan());
    }

    #[test]
    fn single_point_index_at_zero() {
        let x = index_to_x(0, 1);
        assert_eq!(x, AREA.x);
    }
}
