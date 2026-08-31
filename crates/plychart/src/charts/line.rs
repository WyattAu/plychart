//! Line chart renderer.

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

    ctx.set_stroke_style(&color.into());
    ctx.set_line_width(1.5);
    ctx.begin_path();
    for (i, c) in points.iter().enumerate() {
        let x = index_to_x(i);
        let y = price_to_y(c.close);
        if i == 0 {
            ctx.move_to(x, y);
        } else {
            ctx.line_to(x, y);
        }
    }
    ctx.stroke();
}
