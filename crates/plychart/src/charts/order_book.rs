#[cfg(target_arch = "wasm32")]
pub fn draw(
    ctx: &web_sys::CanvasRenderingContext2d,
    bids: &[(f64, f64)],
    asks: &[(f64, f64)],
    area: &crate::types::ChartArea,
    theme: &plycore::ChartTheme,
) {
    if bids.is_empty() && asks.is_empty() {
        return;
    }

    let max_size: f64 = bids
        .iter()
        .chain(asks.iter())
        .map(|(_, s)| *s)
        .fold(f64::NEG_INFINITY, f64::max)
        .max(0.0001);

    let mid_x = area.x + area.w / 2.0;
    let total_levels = bids.len().max(asks.len()).max(1);
    let level_h = f64::min(area.h / total_levels as f64, 20.0);
    let bar_max_w = area.w / 2.0 * 0.9;

    ctx.set_font("10px monospace");

    for (i, &(price, size)) in bids.iter().enumerate() {
        let y = area.y + i as f64 * level_h;
        let w = (size / max_size) * bar_max_w;
        ctx.set_fill_style(&theme.up.into());
        ctx.fill_rect(mid_x - w, y, w, level_h - 1.0);
        ctx.set_fill_style(&theme.text.into());
        ctx.fill_text(
            &format!("{:.2}", price),
            mid_x - w - 48.0,
            y + level_h * 0.7,
        )
        .ok();
        ctx.fill_text(
            &format!("{:.4}", size),
            mid_x - w - 48.0 + 48.0,
            y + level_h * 0.7,
        )
        .ok();
    }

    for (i, &(price, size)) in asks.iter().enumerate() {
        let y = area.y + i as f64 * level_h;
        let w = (size / max_size) * bar_max_w;
        ctx.set_fill_style(&theme.down.into());
        ctx.fill_rect(mid_x, y, w, level_h - 1.0);
        ctx.set_fill_style(&theme.text.into());
        ctx.fill_text(&format!("{:.2}", price), mid_x + 4.0, y + level_h * 0.7)
            .ok();
        ctx.fill_text(&format!("{:.4}", size), mid_x + 52.0, y + level_h * 0.7)
            .ok();
    }

    ctx.set_stroke_style(&theme.grid.into());
    ctx.set_line_width(1.0);
    ctx.begin_path();
    ctx.move_to(mid_x, area.y);
    ctx.line_to(mid_x, area.y + area.h);
    ctx.stroke();
}
