//! Scatter plot renderer.

#[cfg(target_arch = "wasm32")]
pub fn draw(
    ctx: &web_sys::CanvasRenderingContext2d,
    points: &[(f64, f64)],
    area: &crate::types::ChartArea,
    color: &str,
) {
    if points.is_empty() {
        return;
    }
    let min_x = points.iter().map(|p| p.0).fold(f64::INFINITY, f64::min);
    let max_x = points.iter().map(|p| p.0).fold(f64::NEG_INFINITY, f64::max);
    let min_y = points.iter().map(|p| p.1).fold(f64::INFINITY, f64::min);
    let max_y = points.iter().map(|p| p.1).fold(f64::NEG_INFINITY, f64::max);
    let range_x = (max_x - min_x).max(1.0);
    let range_y = (max_y - min_y).max(1.0);

    ctx.set_fill_style(&color.into());
    for (x_val, y_val) in points {
        let x = area.x + (x_val - min_x) / range_x * area.w;
        let y = area.y + area.h - (y_val - min_y) / range_y * area.h;
        ctx.begin_path();
        ctx.arc(x, y, 3.0, 0.0, std::f64::consts::TAU)
            .unwrap_or_default();
        ctx.fill();
    }
}

/// Draw a multi-series scatter plot.
/// `series` is an array of `(points, color)` pairs.
#[cfg(target_arch = "wasm32")]
pub fn draw_multi(
    ctx: &web_sys::CanvasRenderingContext2d,
    series: &[(&[(f64, f64)], &str)],
    area: &crate::types::ChartArea,
) {
    if series.is_empty() {
        return;
    }

    // Compute global min/max across all series
    let mut min_x = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    for (points, _) in series {
        for &(x, y) in points.iter() {
            if x < min_x {
                min_x = x;
            }
            if x > max_x {
                max_x = x;
            }
            if y < min_y {
                min_y = y;
            }
            if y > max_y {
                max_y = y;
            }
        }
    }
    let range_x = (max_x - min_x).max(1.0);
    let range_y = (max_y - min_y).max(1.0);

    for (points, color) in series {
        ctx.set_fill_style(&(*color).into());
        for &(x_val, y_val) in points.iter() {
            let x = area.x + (x_val - min_x) / range_x * area.w;
            let y = area.y + area.h - (y_val - min_y) / range_y * area.h;
            ctx.begin_path();
            ctx.arc(x, y, 3.0, 0.0, std::f64::consts::TAU)
                .unwrap_or_default();
            ctx.fill();
        }
    }
}

#[cfg(test)]
mod tests {
    use plycore::ChartArea;

    const AREA: ChartArea = ChartArea {
        x: 0.0,
        y: 0.0,
        w: 800.0,
        h: 400.0,
    };

    fn scatter_coords(points: &[(f64, f64)]) -> Vec<(f64, f64)> {
        let min_x = points.iter().map(|p| p.0).fold(f64::INFINITY, f64::min);
        let max_x = points.iter().map(|p| p.0).fold(f64::NEG_INFINITY, f64::max);
        let min_y = points.iter().map(|p| p.1).fold(f64::INFINITY, f64::min);
        let max_y = points.iter().map(|p| p.1).fold(f64::NEG_INFINITY, f64::max);
        let range_x = (max_x - min_x).max(1.0);
        let range_y = (max_y - min_y).max(1.0);
        points
            .iter()
            .map(|&(xv, yv)| {
                let x = AREA.x + (xv - min_x) / range_x * AREA.w;
                let y = AREA.y + AREA.h - (yv - min_y) / range_y * AREA.h;
                (x, y)
            })
            .collect()
    }

    #[test]
    fn empty_data_no_panic() {
        let points: Vec<(f64, f64)> = vec![];
        assert!(points.is_empty());
    }

    #[test]
    fn single_point() {
        let points = vec![(5.0, 10.0)];
        let coords = scatter_coords(&points);
        assert_eq!(coords.len(), 1);
        let (x, y) = coords[0];
        assert!(x.is_finite());
        assert!(y.is_finite());
        assert!(!x.is_nan());
        assert!(!y.is_nan());
    }

    #[test]
    fn multiple_points_all_finite() {
        let points = vec![(1.0, 2.0), (3.0, 4.0), (5.0, 6.0), (-1.0, -2.0)];
        let coords = scatter_coords(&points);
        assert_eq!(coords.len(), 4);
        for (x, y) in &coords {
            assert!(x.is_finite());
            assert!(y.is_finite());
        }
    }

    #[test]
    fn points_within_area_bounds() {
        let points = vec![(0.0, 0.0), (100.0, 200.0), (50.0, 100.0)];
        let coords = scatter_coords(&points);
        for (x, y) in &coords {
            assert!(*x >= AREA.x - 1.0, "x {x} should be >= area left");
            assert!(*x <= AREA.x + AREA.w + 1.0, "x {x} should be <= area right");
            assert!(*y >= AREA.y - 1.0, "y {y} should be >= area top");
            assert!(
                *y <= AREA.y + AREA.h + 1.0,
                "y {y} should be <= area bottom"
            );
        }
    }

    #[test]
    fn identical_x_values_range_floors() {
        let points = vec![(5.0, 1.0), (5.0, 10.0)];
        let coords = scatter_coords(&points);
        let range_x = (5.0_f64 - 5.0).max(1.0);
        assert_eq!(range_x, 1.0, "range_x floors at 1.0");
        assert!(!coords[0].0.is_nan());
    }

    #[test]
    fn identical_y_values_range_floors() {
        let points = vec![(1.0, 7.0), (10.0, 7.0)];
        let coords = scatter_coords(&points);
        assert_eq!(coords.len(), 2);
        assert!(!coords[0].1.is_nan());
    }

    #[test]
    fn negative_coordinates() {
        let points = vec![(-100.0, -200.0), (0.0, 0.0), (100.0, 200.0)];
        let coords = scatter_coords(&points);
        assert_eq!(coords.len(), 3);
        for (x, y) in &coords {
            assert!(x.is_finite());
            assert!(y.is_finite());
        }
    }

    #[test]
    fn large_values() {
        let points = vec![(1e12, 1e15), (-1e12, -1e15)];
        let coords = scatter_coords(&points);
        for (x, y) in &coords {
            assert!(x.is_finite());
            assert!(y.is_finite());
        }
    }

    #[test]
    fn sorted_order_preserved() {
        let points = vec![(1.0, 10.0), (2.0, 20.0), (3.0, 30.0)];
        let coords = scatter_coords(&points);
        assert!(coords[0].0 < coords[1].0);
        assert!(coords[1].0 < coords[2].0);
        assert!(coords[0].1 > coords[1].1, "y is inverted on screen");
        assert!(coords[1].1 > coords[2].1);
    }
}
