//! Heatmap chart renderer.

#[cfg(target_arch = "wasm32")]
pub fn draw(
    ctx: &web_sys::CanvasRenderingContext2d,
    matrix: &[Vec<f64>],
    area: &crate::types::ChartArea,
) {
    if matrix.is_empty() || matrix[0].is_empty() {
        return;
    }
    let rows = matrix.len();
    let cols = matrix[0].len();
    let cell_w = area.w / cols as f64;
    let cell_h = area.h / rows as f64;

    let min_val = matrix.iter().flatten().cloned().fold(f64::INFINITY, f64::min);
    let max_val = matrix.iter().flatten().cloned().fold(f64::NEG_INFINITY, f64::max);
    let range = (max_val - min_val).max(1.0);

    for (r, row) in matrix.iter().enumerate() {
        for (c, val) in row.iter().enumerate() {
            let norm = ((val - min_val) / range) as f32;
            let g = ((norm * 255.0) as u8).max(1);
            let color = format!("rgb({},{},{})", 30 + g / 3, 30 + g, 30 + g / 2);
            ctx.set_fill_style(&color.into());
            ctx.fill_rect(
                area.x + c as f64 * cell_w,
                area.y + r as f64 * cell_h,
                cell_w - 1.0,
                cell_h - 1.0,
            );
        }
    }
}
