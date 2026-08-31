#[cfg(target_arch = "wasm32")]
pub fn draw(
    ctx: &web_sys::CanvasRenderingContext2d,
    values: &[f64],
    labels: &[&str],
    color: &str,
    area: &crate::types::ChartArea,
) {
    if values.is_empty() {
        return;
    }

    let n = values.len();
    let cx = area.x + area.w / 2.0;
    let cy = area.y + area.h / 2.0;
    let radius = area.w.min(area.h) / 2.0 * 0.85;
    let max_val = values
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max)
        .max(1.0);

    for i in 0..n {
        let angle = (i as f64 / n as f64) * std::f64::consts::TAU - std::f64::consts::FRAC_PI_2;
        let ex = cx + angle.cos() * radius;
        let ey = cy + angle.sin() * radius;

        ctx.set_stroke_style(&"#333333".into());
        ctx.set_line_width(0.5);
        ctx.begin_path();
        ctx.move_to(cx, cy);
        ctx.line_to(ex, ey);
        ctx.stroke();

        let lx = cx + angle.cos() * (radius + 14.0);
        let ly = cy + angle.sin() * (radius + 14.0);
        if i < labels.len() {
            ctx.set_fill_style(&"#e0e0e0".into());
            ctx.fill_text(labels[i], lx, ly).ok();
        }
    }

    ctx.set_stroke_style(&color.into());
    ctx.set_line_width(1.5);
    ctx.set_fill_style(&format!("{}33", color).into());
    ctx.begin_path();
    for (i, v) in values.iter().enumerate() {
        let angle = (i as f64 / n as f64) * std::f64::consts::TAU - std::f64::consts::FRAC_PI_2;
        let r = (v / max_val) * radius;
        let px = cx + angle.cos() * r;
        let py = cy + angle.sin() * r;
        if i == 0 {
            ctx.move_to(px, py);
        } else {
            ctx.line_to(px, py);
        }
    }
    ctx.close_path();
    ctx.fill();
    ctx.stroke();
}
