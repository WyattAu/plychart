//! Bar chart renderer.

#[cfg(target_arch = "wasm32")]
pub fn draw(
    ctx: &web_sys::CanvasRenderingContext2d,
    bars: &[plycore::CandleData],
    area: &crate::types::ChartArea,
    theme: &plycore::ChartTheme,
) {
    if bars.is_empty() {
        return;
    }

    let min_p = bars.iter().map(|c| c.close).fold(f64::INFINITY, f64::min).min(0.0);
    let max_p = bars.iter().map(|c| c.close).fold(f64::NEG_INFINITY, f64::max);
    let range = (max_p - min_p).max(1.0);
    let bar_w = (area.w / bars.len() as f64 * 0.7).max(1.0);

    for (i, c) in bars.iter().enumerate() {
        let x = area.x + (i as f64 + 0.5) * (area.w / bars.len() as f64);
        let is_up = c.close >= c.open;
        let color = if is_up { theme.up } else { theme.down };
        let h = (c.close.abs() / range) * area.h;
        let y = if c.close >= 0.0 {
            area.y + area.h - h
        } else {
            area.y + area.h
        };
        ctx.set_fill_style(&color.into());
        ctx.fill_rect(x - bar_w / 2.0, y, bar_w, h);
    }
}
