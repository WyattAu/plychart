//! Price and time grid rendering.

/// Draw horizontal price grid lines with labels.
#[cfg(target_arch = "wasm32")]
pub fn draw_price_grid(
    ctx: &web_sys::CanvasRenderingContext2d,
    area: &plycore::ChartArea,
    min_y: f64,
    max_y: f64,
    theme: &plycore::ChartTheme,
) {
    ctx.set_stroke_style(&theme.grid.into());
    ctx.set_line_width(0.5);
    ctx.set_fill_style(&theme.text_muted.into());
    ctx.set_font("9px 'JetBrains Mono', monospace");

    for i in 0..=4 {
        let price = min_y + (max_y - min_y) * (1.0 - i as f64 / 4.0);
        let y = area.y + area.h * (1.0 - i as f64 / 4.0);
        ctx.begin_path();
        ctx.move_to(area.x, y);
        ctx.line_to(area.x + area.w, y);
        ctx.stroke();
        let _ = ctx.fill_text(&format!("{:.2}", price), 4.0, y + 3.0);
    }
}

/// Draw vertical time grid lines with HH:MM labels.
#[cfg(target_arch = "wasm32")]
pub fn draw_time_grid(
    ctx: &web_sys::CanvasRenderingContext2d,
    area: &plycore::ChartArea,
    timestamps: &[f64],
    theme: &plycore::ChartTheme,
) {
    if timestamps.len() <= 1 {
        return;
    }

    ctx.set_stroke_style(&theme.grid.into());
    ctx.set_line_width(0.5);
    ctx.set_fill_style(&theme.text_muted.into());
    ctx.set_font("9px 'JetBrains Mono', monospace");

    let step = (timestamps.len() / 6).max(1);
    for i in (0..timestamps.len()).step_by(step) {
        let x = area.x + (i as f64 + 0.5) * (area.w / timestamps.len() as f64);
        ctx.begin_path();
        ctx.move_to(x, area.y);
        ctx.line_to(x, area.y + area.h);
        ctx.stroke();

        let total_secs = timestamps[i] as i64;
        let hours = (total_secs / 3600) % 24;
        let mins = (total_secs / 60) % 60;
        let _ = ctx.fill_text(
            &format!("{:02}:{:02}", hours, mins),
            x - 15.0,
            area.y + area.h + 14.0,
        );
    }
}
