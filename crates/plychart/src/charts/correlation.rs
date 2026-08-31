#[cfg(target_arch = "wasm32")]
pub fn draw(
    ctx: &web_sys::CanvasRenderingContext2d,
    matrix: &[Vec<f64>],
    labels: &[&str],
    area: &crate::types::ChartArea,
) {
    if matrix.is_empty() || matrix[0].is_empty() {
        return;
    }

    let n = matrix.len();
    let label_w: f64 = 50.0;
    let draw_w = area.w - label_w;
    let draw_h = area.h - label_w;
    let cell_w = draw_w / n as f64;
    let cell_h = draw_h / n as f64;

    for (r, row) in matrix.iter().enumerate() {
        for (c, &val) in row.iter().enumerate() {
            let norm = ((val + 1.0) / 2.0).clamp(0.0, 1.0) as f32;
            let red = (norm * 255.0) as u8;
            let blue = ((1.0 - norm) * 255.0) as u8;
            let color = format!("rgb({},{},{})", red, 255 - red.abs_diff(blue) / 2, blue);
            ctx.set_fill_style(&color.into());
            ctx.fill_rect(
                area.x + label_w + c as f64 * cell_w,
                area.y + r as f64 * cell_h,
                cell_w - 1.0,
                cell_h - 1.0,
            );

            ctx.set_fill_style(&"#e0e0e0".into());
            ctx.set_font("9px sans-serif");
            ctx.fill_text(
                &format!("{:.2}", val),
                area.x + label_w + c as f64 * cell_w + 2.0,
                area.y + r as f64 * cell_h + cell_h * 0.65,
            )
            .ok();
        }
    }

    ctx.set_fill_style(&"#e0e0e0".into());
    ctx.set_font("9px sans-serif");
    for i in 0..n {
        if i < labels.len() {
            ctx.fill_text(
                labels[i],
                area.x,
                area.y + i as f64 * cell_h + cell_h * 0.65,
            )
            .ok();
            ctx.save();
            ctx.translate(
                area.x + label_w + i as f64 * cell_w + cell_w * 0.4,
                area.y + draw_h + 4.0,
            )
            .ok();
            ctx.rotate(-std::f64::consts::FRAC_PI_2).ok();
            ctx.fill_text(labels[i], 0.0, 0.0).ok();
            ctx.restore();
        }
    }
}
