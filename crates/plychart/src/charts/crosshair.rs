//! Crosshair overlay and OHLC readout.

/// Draw dashed crosshair lines at mouse position.
#[cfg(target_arch = "wasm32")]
pub fn draw_crosshair(
    ctx: &web_sys::CanvasRenderingContext2d,
    mx: f64,
    my: f64,
    area: &plycore::ChartArea,
    theme: &plycore::ChartTheme,
) {
    ctx.set_stroke_style(&theme.crosshair.into());
    ctx.set_line_width(0.5);
    ctx.set_line_dash(&js_sys::Array::of2(&2.0.into(), &2.0.into()));

    ctx.begin_path();
    ctx.move_to(area.x, my);
    ctx.line_to(area.x + area.w, my);
    ctx.stroke();

    ctx.begin_path();
    ctx.move_to(mx, area.y);
    ctx.line_to(mx, area.y + area.h);
    ctx.stroke();

    ctx.set_line_dash(&js_sys::Array::new());
}

/// Draw OHLC readout text at top-left for nearest candle.
#[cfg(target_arch = "wasm32")]
pub fn draw_ohlc_readout(
    ctx: &web_sys::CanvasRenderingContext2d,
    candles: &[plycore::CandleData],
    mx: f64,
    index_to_x: &dyn Fn(usize) -> f64,
    theme: &plycore::ChartTheme,
) {
    if candles.is_empty() {
        return;
    }

    let mut nearest_idx = 0;
    let mut nearest_dist = f64::INFINITY;
    for i in 0..candles.len() {
        let dist = (index_to_x(i) - mx).abs();
        if dist < nearest_dist {
            nearest_dist = dist;
            nearest_idx = i;
        }
    }

    let c = &candles[nearest_idx];
    let text = format!(
        "O:{:.2} H:{:.2} L:{:.2} C:{:.2}",
        c.open, c.high, c.low, c.close
    );
    ctx.set_font("10px 'JetBrains Mono', monospace");
    ctx.set_fill_style(&theme.text.into());
    let _ = ctx.fill_text(&text, 10.0, 16.0);
}
