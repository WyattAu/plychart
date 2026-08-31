//! Volume sub-pane rendering.

/// Draw volume bars for each candle.
#[cfg(target_arch = "wasm32")]
pub fn draw_volume(
    ctx: &web_sys::CanvasRenderingContext2d,
    candles: &[plycore::CandleData],
    index_to_x: &dyn Fn(usize) -> f64,
    area: &plycore::ChartArea,
    max_vol: f64,
    theme: &plycore::ChartTheme,
    bar_w: f64,
) {
    if max_vol <= 0.0 || candles.is_empty() {
        return;
    }

    for (i, c) in candles.iter().enumerate() {
        let x = index_to_x(i);
        let h = (c.volume / max_vol) * area.h;
        let is_up = c.close >= c.open;
        let color = if is_up { theme.up } else { theme.down };
        ctx.set_fill_style(&color.into());
        ctx.set_global_alpha(0.3);
        ctx.fill_rect(x - bar_w / 2.0, area.y + area.h - h, bar_w, h);
    }
    ctx.set_global_alpha(1.0);
}
