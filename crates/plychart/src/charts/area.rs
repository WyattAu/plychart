//! Area chart renderer with gradient fill.

#[cfg(target_arch = "wasm32")]
pub fn draw(
    ctx: &web_sys::CanvasRenderingContext2d,
    points: &[plycore::CandleData],
    area: &crate::types::ChartArea,
    color: &str,
) {
    if points.is_empty() {
        return;
    }

    // Draw filled area under line
    let min_p = points.iter().map(|c| c.close).fold(f64::INFINITY, f64::min);
    let max_p = points
        .iter()
        .map(|c| c.close)
        .fold(f64::NEG_INFINITY, f64::max);
    let range = (max_p - min_p).max(0.0001);
    let pad = range * 0.05;
    let min_p = min_p - pad;
    let max_p = max_p + pad;
    let total_range = max_p - min_p;

    let price_to_y =
        |price: f64| -> f64 { area.y + area.h * (1.0 - (price - min_p) / total_range) };
    let index_to_x =
        |i: usize| -> f64 { area.x + (i as f64 / (points.len() - 1).max(1) as f64) * area.w };

    // Area fill
    let grad = ctx.create_linear_gradient(0.0, area.y, 0.0, area.y + area.h);
    grad.add_color_stop(0.0, color);
    grad.add_color_stop(1.0, "transparent");
    ctx.set_fill_style(&grad.into());
    ctx.begin_path();
    ctx.move_to(index_to_x(0), area.y + area.h);
    for (i, c) in points.iter().enumerate() {
        ctx.line_to(index_to_x(i), price_to_y(c.close));
    }
    ctx.line_to(index_to_x(points.len() - 1), area.y + area.h);
    ctx.close_path();
    ctx.fill();

    // Line on top
    crate::charts::line::draw(ctx, points, area, color);
}
