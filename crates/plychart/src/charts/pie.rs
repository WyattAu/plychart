//! Pie / Donut chart renderer.

#[cfg(target_arch = "wasm32")]
pub fn draw(
    ctx: &web_sys::CanvasRenderingContext2d,
    items: &[(String, f64)],
    area: &plycore::ChartArea,
    theme: &plycore::ChartTheme,
) {
    use wasm_bindgen::JsCast;

    if items.is_empty() {
        return;
    }

    let total: f64 = items.iter().map(|(_, v)| *v).sum();
    if total <= 0.0 {
        return;
    }

    let cx = area.x + area.w / 2.0;
    let cy = area.y + area.h / 2.0;
    let radius = area.w.min(area.h) / 2.0 * 0.85;
    let inner_radius = radius * 0.55; // donut hole

    // 8-color palette
    let palette = [
        theme.accent,
        theme.up,
        theme.down,
        "#a78bfa",
        "#fb923c",
        "#38bdf8",
        "#f472b6",
        "#34d399",
    ];

    let mut start_angle = -std::f64::consts::FRAC_PI_2;

    for (i, (_label, value)) in items.iter().enumerate() {
        let sweep = (value / total) * std::f64::consts::TAU;
        let end_angle = start_angle + sweep;
        let color = palette[i % palette.len()];

        // Draw slice
        ctx.set_fill_style(&color.into());
        ctx.begin_path();
        ctx.move_to(cx, cy);
        ctx.arc(cx, cy, radius, start_angle, end_angle)
            .unwrap_or_default();
        ctx.close_path();
        ctx.fill();

        start_angle = end_angle;
    }

    // Draw inner circle (donut hole)
    ctx.set_fill_style(&theme.bg.into());
    ctx.begin_path();
    ctx.arc(cx, cy, inner_radius, 0.0, std::f64::consts::TAU)
        .unwrap_or_default();
    ctx.fill();

    // Center label: total
    ctx.set_fill_style(&theme.text.into());
    ctx.set_font("bold 14px monospace");
    ctx.set_text_align("center");
    ctx.set_text_baseline("middle");
    let total_str = format_total(total);
    ctx.fill_text(&total_str, cx, cy - 8.0).unwrap_or_default();

    ctx.set_fill_style(&theme.text_muted.into());
    ctx.set_font("10px monospace");
    ctx.fill_text("total", cx, cy + 10.0).unwrap_or_default();

    // Legend at bottom
    let legend_y = area.y + area.h - 12.0;
    let legend_x = area.x + 8.0;
    let mut lx = legend_x;

    for (i, (label, value)) in items.iter().enumerate() {
        let color = palette[i % palette.len()];
        let pct = (value / total) * 100.0;

        ctx.set_fill_style(&color.into());
        ctx.fill_rect(lx, legend_y - 6.0, 8.0, 8.0);

        ctx.set_fill_style(&theme.text_muted.into());
        ctx.set_font("9px monospace");
        ctx.set_text_align("left");
        let text = format!("{label} {pct:.0}%");
        ctx.fill_text(&text, lx + 12.0, legend_y + 1.0)
            .unwrap_or_default();

        lx += ctx.measure_text(&text).map(|m| m.width()).unwrap_or(0.0) + 20.0;
    }
}

#[cfg(any(target_arch = "wasm32", test))]
fn format_total(v: f64) -> String {
    if v >= 1_000_000.0 {
        format!("{:.1}M", v / 1_000_000.0)
    } else if v >= 1_000.0 {
        format!("{:.1}K", v / 1_000.0)
    } else {
        format!("{v:.0}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_total_thousands() {
        assert_eq!(format_total(1500.0), "1.5K");
    }

    #[test]
    fn format_total_millions() {
        assert_eq!(format_total(2_500_000.0), "2.5M");
    }

    #[test]
    fn format_total_small() {
        assert_eq!(format_total(42.0), "42");
    }
}
