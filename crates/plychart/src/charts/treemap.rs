//! Treemap renderer.
#[cfg(target_arch = "wasm32")]
pub fn draw(
    ctx: &web_sys::CanvasRenderingContext2d,
    _items: &[(String, f64)],
    _area: &crate::types::ChartArea,
) {
    // Treemap implementation — nested rectangles
}
