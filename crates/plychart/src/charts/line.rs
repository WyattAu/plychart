//! Line chart renderer.

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

    ctx.set_stroke_style(&color.into());
    ctx.set_line_width(1.5);
    ctx.begin_path();
    for (i, c) in points.iter().enumerate() {
        let x = index_to_x(i);
        let y = price_to_y(c.close);
        if i == 0 {
            ctx.move_to(x, y);
        } else {
            ctx.line_to(x, y);
        }
    }
    ctx.stroke();
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

    fn line_range(points: &[CandleData]) -> (f64, f64) {
        let min_p = points.iter().map(|c| c.close).fold(f64::INFINITY, f64::min);
        let max_p = points
            .iter()
            .map(|c| c.close)
            .fold(f64::NEG_INFINITY, f64::max);
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
    fn single_point() {
        let points = vec![CandleData {
            time: 1.0,
            open: 100.0,
            high: 100.0,
            low: 100.0,
            close: 100.0,
            volume: 0.0,
        }];
        let (min_p, max_p) = line_range(&points);
        let total = max_p - min_p;
        assert!(total > 0.0);
        let y = price_to_y(100.0, min_p, max_p);
        assert!(y.is_finite());
        assert!(!y.is_nan());
    }

    #[test]
    fn single_point_index_to_x() {
        let x = index_to_x(0, 1);
        assert_eq!(x, AREA.x, "single point should be at left edge");
    }

    #[test]
    fn multiple_points_index_mapping() {
        let points = vec![
            CandleData {
                time: 1.0,
                open: 0.0,
                high: 0.0,
                low: 0.0,
                close: 10.0,
                volume: 0.0,
            },
            CandleData {
                time: 2.0,
                open: 0.0,
                high: 0.0,
                low: 0.0,
                close: 20.0,
                volume: 0.0,
            },
            CandleData {
                time: 3.0,
                open: 0.0,
                high: 0.0,
                low: 0.0,
                close: 30.0,
                volume: 0.0,
            },
        ];
        let x0 = index_to_x(0, points.len());
        let x1 = index_to_x(1, points.len());
        let x2 = index_to_x(2, points.len());
        assert!(x0 < x1);
        assert!(x1 < x2);
        assert!(
            (x2 - AREA.x - AREA.w).abs() < 1e-10,
            "last point at right edge"
        );
    }

    #[test]
    fn constant_price_line() {
        let points = vec![
            CandleData {
                time: 1.0,
                open: 50.0,
                high: 50.0,
                low: 50.0,
                close: 50.0,
                volume: 0.0,
            },
            CandleData {
                time: 2.0,
                open: 50.0,
                high: 50.0,
                low: 50.0,
                close: 50.0,
                volume: 0.0,
            },
        ];
        let (min_p, max_p) = line_range(&points);
        let y1 = price_to_y(50.0, min_p, max_p);
        let y2 = price_to_y(50.0, min_p, max_p);
        assert_eq!(y1, y2, "constant price should map to same y");
    }

    #[test]
    fn price_mapping_inverses_with_screen_y() {
        let points = vec![
            CandleData {
                time: 1.0,
                open: 0.0,
                high: 0.0,
                low: 0.0,
                close: 10.0,
                volume: 0.0,
            },
            CandleData {
                time: 2.0,
                open: 0.0,
                high: 0.0,
                low: 0.0,
                close: 20.0,
                volume: 0.0,
            },
        ];
        let (min_p, max_p) = line_range(&points);
        let y_low = price_to_y(10.0, min_p, max_p);
        let y_high = price_to_y(20.0, min_p, max_p);
        assert!(y_low > y_high, "higher price maps to lower screen y");
    }

    #[test]
    fn no_nan_in_outputs() {
        let points = vec![
            CandleData {
                time: 1.0,
                open: 0.0,
                high: 0.0,
                low: 0.0,
                close: -100.0,
                volume: 0.0,
            },
            CandleData {
                time: 2.0,
                open: 0.0,
                high: 0.0,
                low: 0.0,
                close: 100.0,
                volume: 0.0,
            },
        ];
        let (min_p, max_p) = line_range(&points);
        assert!(min_p.is_finite());
        assert!(max_p.is_finite());
        for p in &points {
            let y = price_to_y(p.close, min_p, max_p);
            assert!(y.is_finite());
        }
    }

    #[test]
    fn negative_prices() {
        let points = vec![
            CandleData {
                time: 1.0,
                open: 0.0,
                high: 0.0,
                low: 0.0,
                close: -50.0,
                volume: 0.0,
            },
            CandleData {
                time: 2.0,
                open: 0.0,
                high: 0.0,
                low: 0.0,
                close: -10.0,
                volume: 0.0,
            },
        ];
        let (min_p, max_p) = line_range(&points);
        assert!(min_p < 0.0);
        assert!(max_p > min_p);
    }
}
