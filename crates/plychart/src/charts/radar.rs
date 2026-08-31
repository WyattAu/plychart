//! Radar/spider chart renderer.
#[cfg(target_arch = "wasm32")]
pub fn draw(
    ctx: &web_sys::CanvasRenderingContext2d,
    _values: &[f64],
    _labels: &[&str],
    _color: &str,
    _area: &crate::types::ChartArea,
) {
    // Radar chart implementation — polygon with spokes
}
