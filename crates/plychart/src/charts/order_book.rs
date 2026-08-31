//! Order book depth visualization.
#[cfg(target_arch = "wasm32")]
pub fn draw(
    ctx: &web_sys::CanvasRenderingContext2d,
    _bids: &[(f64, f64)],
    _asks: &[(f64, f64)],
    _area: &crate::types::ChartArea,
    _theme: &plycore::ChartTheme,
) {
    // Order book depth — horizontal bars for bid/ask levels
}
