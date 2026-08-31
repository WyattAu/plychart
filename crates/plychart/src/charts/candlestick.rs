//! Candlestick chart renderer — wick + body with up/down colors.

#[cfg(target_arch = "wasm32")]
pub fn draw(
    ctx: &web_sys::CanvasRenderingContext2d,
    candles: &[plycore::CandleData],
    area: &crate::types::ChartArea,
    theme: &plycore::ChartTheme,
    candle_width: f64,
) {
    if candles.is_empty() {
        return;
    }

    let min_p = candles.iter().map(|c| c.low).fold(f64::INFINITY, f64::min);
    let max_p = candles
        .iter()
        .map(|c| c.high)
        .fold(f64::NEG_INFINITY, f64::max);
    let range = (max_p - min_p).max(0.0001);
    let pad = range * 0.05;
    let min_p = min_p - pad;
    let max_p = max_p + pad;
    let total_range = max_p - min_p;

    let price_to_y =
        |price: f64| -> f64 { area.y + area.h * (1.0 - (price - min_p) / total_range) };
    let index_to_x =
        |i: usize| -> f64 { area.x + (i as f64 + 0.5) * (area.w / candles.len() as f64) };

    for (i, c) in candles.iter().enumerate() {
        let x = index_to_x(i);
        let is_up = c.close >= c.open;
        let color = if is_up { theme.up } else { theme.down };

        // Wick
        ctx.set_stroke_style(&color.into());
        ctx.set_line_width(1.0);
        ctx.begin_path();
        ctx.move_to(x, price_to_y(c.high));
        ctx.line_to(x, price_to_y(c.low));
        ctx.stroke();

        // Body
        let body_top = price_to_y(c.open.max(c.close));
        let body_bot = price_to_y(c.open.min(c.close));
        let body_h = (body_bot - body_top).max(1.0);
        ctx.set_fill_style(&color.into());
        ctx.fill_rect(x - candle_width / 2.0, body_top, candle_width, body_h);
    }
}
