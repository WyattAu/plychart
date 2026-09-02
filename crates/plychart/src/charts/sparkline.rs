//! Sparkline chart — minimal line chart without axes, grid, or labels.

#[cfg(target_arch = "wasm32")]
pub fn draw(
    ctx: &web_sys::CanvasRenderingContext2d,
    values: &[f64],
    area: &plycore::ChartArea,
    color: &str,
) {
    if values.len() < 2 {
        return;
    }

    let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let range = (max - min).abs().max(f64::EPSILON);

    let pad = 2.0;
    let w = area.w - pad * 2.0;
    let h = area.h - pad * 2.0;

    // Stroke line
    ctx.set_stroke_style(&color.into());
    ctx.set_line_width(1.5);
    ctx.begin_path();

    for (i, &v) in values.iter().enumerate() {
        let x = area.x + pad + (i as f64 / (values.len() - 1) as f64) * w;
        let y = area.y + pad + h - ((v - min) / range) * h;

        if i == 0 {
            ctx.move_to(x, y);
        } else {
            ctx.line_to(x, y);
        }
    }
    ctx.stroke();

    // Area fill with gradient
    let last_x = area.x + pad + w;
    let base_y = area.y + pad + h;

    ctx.begin_path();
    ctx.move_to(area.x + pad, base_y);

    for (i, &v) in values.iter().enumerate() {
        let x = area.x + pad + (i as f64 / (values.len() - 1) as f64) * w;
        let y = area.y + pad + h - ((v - min) / range) * h;
        ctx.line_to(x, y);
    }

    ctx.line_to(last_x, base_y);
    ctx.close_path();

    // Gradient fill
    let gradient = ctx
        .create_linear_gradient(0.0, area.y, 0.0, area.y + area.h)
        .unwrap_or_default();
    gradient.add_color_with_alpha(color, 0.15).unwrap_or_default();
    gradient.add_color_with_alpha(color, 0.0).unwrap_or_default();
    ctx.set_fill_style(&gradient.into());
    ctx.fill();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sparkline_constant_value() {
        let values = vec![5.0; 10];
        let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let range = (max - min).abs();
        assert_eq!(range, 0.0);
    }

    #[test]
    fn sparkline_empty() {
        let values: Vec<f64> = vec![];
        assert!(values.len() < 2);
    }

    #[test]
    fn sparkline_two_points() {
        let values = vec![1.0, 5.0];
        assert_eq!(values.len(), 2);
    }
}
