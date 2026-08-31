//! Waterfall chart renderer.
#[cfg(target_arch = "wasm32")]
pub fn draw(
    ctx: &web_sys::CanvasRenderingContext2d,
    _bars: &[crate::charts::BarData],
    _area: &crate::types::ChartArea,
    _theme: &plycore::ChartTheme,
) {
    // Waterfall chart — incremental bar chart
}
