//! Correlation matrix visualization.
#[cfg(target_arch = "wasm32")]
pub fn draw(
    ctx: &web_sys::CanvasRenderingContext2d,
    _matrix: &[Vec<f64>],
    _labels: &[&str],
    _area: &crate::types::ChartArea,
) {
    // Correlation heatmap — color-coded matrix
}
