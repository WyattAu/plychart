//! Heatmap chart renderer.

#[cfg(target_arch = "wasm32")]
pub fn draw(
    ctx: &web_sys::CanvasRenderingContext2d,
    matrix: &[Vec<f64>],
    area: &crate::types::ChartArea,
) {
    if matrix.is_empty() || matrix[0].is_empty() {
        return;
    }
    let rows = matrix.len();
    let cols = matrix[0].len();
    let cell_w = area.w / cols as f64;
    let cell_h = area.h / rows as f64;

    let min_val = matrix
        .iter()
        .flatten()
        .cloned()
        .fold(f64::INFINITY, f64::min);
    let max_val = matrix
        .iter()
        .flatten()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);
    let range = (max_val - min_val).max(1.0);

    for (r, row) in matrix.iter().enumerate() {
        for (c, val) in row.iter().enumerate() {
            let norm = ((val - min_val) / range) as f32;
            let g = ((norm * 255.0) as u8).max(1);
            let color = format!("rgb({},{},{})", 30 + g / 3, 30 + g, 30 + g / 2);
            ctx.set_fill_style(&color.into());
            ctx.fill_rect(
                area.x + c as f64 * cell_w,
                area.y + r as f64 * cell_h,
                cell_w - 1.0,
                cell_h - 1.0,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use plycore::ChartArea;

    const AREA: ChartArea = ChartArea {
        x: 0.0,
        y: 0.0,
        w: 400.0,
        h: 300.0,
    };

    fn heatmap_metrics(matrix: &[Vec<f64>]) -> Option<(usize, usize, f64, f64, f64)> {
        if matrix.is_empty() || matrix[0].is_empty() {
            return None;
        }
        let rows = matrix.len();
        let cols = matrix[0].len();
        let cell_w = AREA.w / cols as f64;
        let cell_h = AREA.h / rows as f64;
        let min_val = matrix
            .iter()
            .flatten()
            .cloned()
            .fold(f64::INFINITY, f64::min);
        let max_val = matrix
            .iter()
            .flatten()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);
        let range = (max_val - min_val).max(1.0);
        Some((rows, cols, cell_w, cell_h, range))
    }

    fn normalize(val: f64, min_val: f64, range: f64) -> f32 {
        ((val - min_val) / range) as f32
    }

    #[test]
    fn empty_matrix_no_panic() {
        let matrix: Vec<Vec<f64>> = vec![];
        assert!(heatmap_metrics(&matrix).is_none());
    }

    #[test]
    fn empty_inner_vec_no_panic() {
        let matrix: Vec<Vec<f64>> = vec![vec![], vec![]];
        assert!(heatmap_metrics(&matrix).is_none());
    }

    #[test]
    fn single_cell() {
        let matrix = vec![vec![42.0]];
        let m = heatmap_metrics(&matrix).unwrap();
        assert_eq!(m.0, 1);
        assert_eq!(m.1, 1);
        assert!(m.2 > 0.0);
        assert!(m.3 > 0.0);
    }

    #[test]
    fn multiple_cells_dimensions() {
        let matrix = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
        let m = heatmap_metrics(&matrix).unwrap();
        assert_eq!(m.0, 2, "rows");
        assert_eq!(m.1, 3, "cols");
        assert!((m.2 - AREA.w / 3.0).abs() < 1e-10);
        assert!((m.3 - AREA.h / 2.0).abs() < 1e-10);
    }

    #[test]
    fn normalization_range() {
        let matrix = vec![vec![10.0, 20.0, 30.0]];
        let m = heatmap_metrics(&matrix).unwrap();
        let range = m.4;
        let min_val = 10.0;
        let n10 = normalize(10.0, min_val, range);
        let n30 = normalize(30.0, min_val, range);
        assert!((n10 - 0.0).abs() < 1e-5);
        assert!((n30 - 1.0).abs() < 1e-5);
    }

    #[test]
    fn constant_values_range_floors() {
        let matrix = vec![vec![5.0, 5.0], vec![5.0, 5.0]];
        let m = heatmap_metrics(&matrix).unwrap();
        assert_eq!(m.4, 1.0, "range floors at 1.0");
    }

    #[test]
    fn color_computation_no_nan() {
        let norm = 0.5f32;
        let g = ((norm * 255.0) as u8).max(1);
        assert!(g > 0);
        let color = format!("rgb({},{},{})", 30 + g / 3, 30 + g, 30 + g / 2);
        assert!(color.starts_with("rgb("));
    }

    #[test]
    fn negative_values() {
        let matrix = vec![vec![-100.0, 0.0, 100.0]];
        let m = heatmap_metrics(&matrix).unwrap();
        assert!(m.4 > 0.0);
        let n = normalize(0.0, -100.0, m.4);
        assert!(n > 0.0 && n < 1.0);
    }

    #[test]
    fn large_matrix() {
        let matrix: Vec<Vec<f64>> = (0..100)
            .map(|r| (0..100).map(|c| (r * 100 + c) as f64).collect())
            .collect();
        let m = heatmap_metrics(&matrix).unwrap();
        assert_eq!(m.0, 100);
        assert_eq!(m.1, 100);
        assert!(m.2 > 0.0);
        assert!(m.3 > 0.0);
    }

    #[test]
    fn cell_dimensions_fits_area() {
        let matrix = vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]];
        let m = heatmap_metrics(&matrix).unwrap();
        let total_w = m.1 as f64 * m.2;
        let total_h = m.0 as f64 * m.3;
        assert!((total_w - AREA.w).abs() < 1e-10);
        assert!((total_h - AREA.h).abs() < 1e-10);
    }
}
