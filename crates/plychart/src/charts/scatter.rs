//! Scatter plot renderer.

#[cfg(target_arch = "wasm32")]
pub fn draw(
    ctx: &web_sys::CanvasRenderingContext2d,
    points: &[(f64, f64)],
    area: &crate::types::ChartArea,
    color: &str,
) {
    if points.is_empty() {
        return;
    }
    let min_x = points.iter().map(|p| p.0).fold(f64::INFINITY, f64::min);
    let max_x = points.iter().map(|p| p.0).fold(f64::NEG_INFINITY, f64::max);
    let min_y = points.iter().map(|p| p.1).fold(f64::INFINITY, f64::min);
    let max_y = points.iter().map(|p| p.1).fold(f64::NEG_INFINITY, f64::max);
    let range_x = (max_x - min_x).max(1.0);
    let range_y = (max_y - min_y).max(1.0);

    ctx.set_fill_style(&color.into());
    for (x_val, y_val) in points {
        let x = area.x + (x_val - min_x) / range_x * area.w;
        let y = area.y + area.h - (y_val - min_y) / range_y * area.h;
        ctx.begin_path();
        ctx.arc(x, y, 3.0, 0.0, std::f64::consts::TAU).unwrap_or_default();
        ctx.fill();
    }
}
