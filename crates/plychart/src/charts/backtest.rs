//! Backtest equity curve + drawdown split pane.

#[cfg(target_arch = "wasm32")]
pub fn draw(
    ctx: &web_sys::CanvasRenderingContext2d,
    equity: &[f64],
    drawdown: &[f64],
    area: &crate::types::ChartArea,
    theme: &plycore::ChartTheme,
) {
    if equity.is_empty() {
        return;
    }

    let split_y = area.y + area.h * 0.65;
    let dd_h = area.h - (split_y - area.y) - 30.0;

    // Equity curve
    let min_eq = equity.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_eq = equity.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let eq_range = (max_eq - min_eq).max(1.0);

    ctx.set_stroke_style(&theme.accent.into());
    ctx.set_line_width(1.5);
    ctx.begin_path();
    for (i, &val) in equity.iter().enumerate() {
        let x = (i as f64 / (equity.len() - 1).max(1) as f64) * area.w + area.x;
        let y = area.y + 30.0 + (1.0 - (val - min_eq) / eq_range) * (split_y - area.y - 40.0);
        if i == 0 { ctx.move_to(x, y); } else { ctx.line_to(x, y); }
    }
    ctx.stroke();

    // Drawdown
    if !drawdown.is_empty() {
        let min_dd = drawdown.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_dd = drawdown.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let dd_range = (max_dd - min_dd).abs().max(1.0);

        ctx.set_stroke_style(&theme.down.into());
        ctx.set_line_width(1.5);
        ctx.begin_path();
        for (i, &val) in drawdown.iter().enumerate() {
            let x = (i as f64 / (drawdown.len() - 1).max(1) as f64) * area.w + area.x;
            let y = split_y + 20.0 + (1.0 - (val - min_dd) / dd_range) * dd_h;
            if i == 0 { ctx.move_to(x, y); } else { ctx.line_to(x, y); }
        }
        ctx.stroke();
    }
}
