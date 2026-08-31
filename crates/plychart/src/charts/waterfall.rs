#[cfg(target_arch = "wasm32")]
pub fn draw(
    ctx: &web_sys::CanvasRenderingContext2d,
    bars: &[crate::charts::BarData],
    area: &crate::types::ChartArea,
    theme: &plycore::ChartTheme,
) {
    if bars.is_empty() {
        return;
    }

    let mut cumulative: f64 = 0.0;
    let mut min_val: f64 = 0.0;
    let mut max_val: f64 = 0.0;
    for b in bars {
        cumulative += b.value;
        min_val = f64::min(min_val, cumulative);
        max_val = f64::max(max_val, cumulative);
    }
    let range = f64::max(max_val - min_val, 1.0);
    let pad = range * 0.1;
    let min_val = min_val - pad;
    let max_val = max_val + pad;
    let total_range = max_val - min_val;

    let val_to_y = |v: f64| -> f64 { area.y + area.h * (1.0 - (v - min_val) / total_range) };
    let bar_w = f64::max(area.w / bars.len() as f64 * 0.6, 1.0);

    let mut running: f64 = 0.0;
    let mut prev_y = val_to_y(0.0);

    for (i, b) in bars.iter().enumerate() {
        let x = area.x + (i as f64 + 0.5) * (area.w / bars.len() as f64);
        let start = running;
        running += b.value;
        let y_start = val_to_y(f64::max(start, running));
        let y_end = val_to_y(f64::min(start, running));
        let h = f64::max(y_end - y_start, 1.0);

        let color = if b.value >= 0.0 { theme.up } else { theme.down };
        ctx.set_fill_style(&color.into());
        ctx.fill_rect(x - bar_w / 2.0, y_start, bar_w, h);

        ctx.set_stroke_style(&theme.text_muted.into());
        ctx.set_line_width(0.5);
        ctx.begin_path();
        ctx.move_to(x - bar_w / 2.0, prev_y);
        ctx.line_to(
            x - bar_w / 2.0,
            if b.value >= 0.0 { y_end } else { y_start },
        );
        ctx.stroke();

        ctx.set_stroke_style(&theme.text_muted.into());
        ctx.set_line_width(0.5);
        ctx.begin_path();
        ctx.move_to(
            x + bar_w / 2.0,
            if b.value >= 0.0 { y_end } else { y_start },
        );
        ctx.line_to(
            x + bar_w / 2.0 + (area.w / bars.len() as f64 - bar_w) / 2.0,
            if b.value >= 0.0 { y_end } else { y_start },
        );
        ctx.stroke();

        prev_y = if b.value >= 0.0 { y_start } else { y_end };

        ctx.set_fill_style(&theme.text.into());
        ctx.set_font("10px sans-serif");
        ctx.fill_text(&b.label, x - bar_w / 2.0, area.y + area.h + 12.0)
            .ok();
    }
}
