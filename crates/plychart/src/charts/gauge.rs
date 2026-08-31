//! Gauge chart renderer.

#[cfg(target_arch = "wasm32")]
pub fn draw(
    ctx: &web_sys::CanvasRenderingContext2d,
    value: f64,
    max: f64,
    color: &str,
    area: &crate::types::ChartArea,
) {
    let cx = area.x + area.w / 2.0;
    let cy = area.y + area.h * 0.8;
    let radius = (area.w / 2.0 - 20.0).min(area.h * 0.7);

    // Background arc
    ctx.set_stroke_style(&"rgba(255,255,255,0.1)".into());
    ctx.set_line_width(8.0);
    ctx.begin_path();
    ctx.arc(cx, cy, radius, std::f64::consts::PI, 0.0)
        .unwrap_or_default();
    ctx.stroke();

    // Value arc
    let ratio = (value / max).clamp(0.0, 1.0);
    ctx.set_stroke_style(&color.into());
    ctx.set_line_width(8.0);
    ctx.begin_path();
    ctx.arc(
        cx,
        cy,
        radius,
        std::f64::consts::PI,
        std::f64::consts::PI + ratio * std::f64::consts::PI,
    )
    .unwrap_or_default();
    ctx.stroke();

    // Value text
    ctx.set_fill_style(&"#ffffff".into());
    ctx.set_font("bold 16px monospace");
    let _ = ctx.fill_text(&format!("{:.1}", value), cx - 20.0, cy);
}
