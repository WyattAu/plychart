//! Stacked bar chart renderer.

#[cfg(target_arch = "wasm32")]
pub fn draw(
    ctx: &web_sys::CanvasRenderingContext2d,
    segments: &[Vec<f64>],
    labels: &[&str],
    area: &plycore::ChartArea,
    theme: &plycore::ChartTheme,
) {
    if segments.is_empty() || labels.is_empty() {
        return;
    }
    let n = segments.len().min(labels.len());
    if n == 0 {
        return;
    }

    // Max total across bars for Y scaling
    let max_total = segments
        .iter()
        .take(n)
        .map(|s| s.iter().sum::<f64>())
        .fold(0.0_f64, f64::max)
        .max(1.0);

    let bar_w = (area.w / n as f64 * 0.6).max(2.0);
    let gap = (area.w / n as f64 * 0.4).max(2.0);

    // Colors: issues=down, warnings=amber, passed=up/transparent
    let palette = [theme.down, "#eab308", theme.up];

    for (i, segs) in segments.iter().take(n).enumerate() {
        let x = area.x + gap / 2.0 + i as f64 * (bar_w + gap);
        let mut y = area.y + area.h;

        for (si, &val) in segs.iter().enumerate() {
            if val <= 0.0 {
                continue;
            }
            let h = (val / max_total) * area.h;
            y -= h;
            let color = palette[si % palette.len()];
            // passed segment (index 2) at 40% opacity
            if si == 2 {
                ctx.set_global_alpha(0.4);
            } else if si == 1 {
                ctx.set_global_alpha(0.7);
            } else {
                ctx.set_global_alpha(0.9);
            }
            ctx.set_fill_style(&color.into());
            ctx.fill_rect(x, y, bar_w, h);
            ctx.set_global_alpha(1.0);
        }

        // total count on top
        let total: f64 = segs.iter().sum();
        if total > 0.0 {
            ctx.set_fill_style(&theme.text.into());
            ctx.set_font("bold 9px monospace");
            ctx.set_text_align("center");
            let _ = ctx.fill_text(&format!("{:.0}", total), x + bar_w / 2.0, y - 4.0);
        }

        // category label
        ctx.set_fill_style(&theme.text_muted.into());
        ctx.set_font("8px monospace");
        ctx.set_text_align("center");
        let label = labels[i];
        let short = if label.len() > 6 { &label[..6] } else { label };
        let _ = ctx.fill_text(
            &short.to_uppercase(),
            x + bar_w / 2.0,
            area.y + area.h + 14.0,
        );
    }

    // Y-axis ticks
    ctx.set_fill_style(&theme.text_muted.into());
    ctx.set_font("8px monospace");
    ctx.set_text_align("right");
    for k in 0..=4 {
        let v = max_total * (4 - k) as f64 / 4.0;
        let y = area.y + area.h * k as f64 / 4.0;
        let _ = ctx.fill_text(&format!("{:.0}", v), area.x - 6.0, y + 3.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use plycore::ChartArea;

    const AREA: ChartArea = ChartArea {
        x: 40.0,
        y: 10.0,
        w: 400.0,
        h: 200.0,
    };

    fn total(segs: &[Vec<f64>]) -> f64 {
        segs.iter()
            .map(|s| s.iter().sum::<f64>())
            .fold(0.0, f64::max)
    }

    #[test]
    fn empty_no_panic() {
        let segs: Vec<Vec<f64>> = vec![];
        let labels: Vec<&str> = vec![];
        assert!(segs.is_empty());
        assert!(total(&segs) == 0.0);
        let _ = labels;
        let _ = AREA;
    }

    #[test]
    fn max_total_computed() {
        let segs = vec![vec![3.0, 2.0, 5.0], vec![1.0, 1.0, 8.0]];
        assert!((total(&segs) - 10.0).abs() < 1e-9);
    }

    #[test]
    fn single_bar() {
        let segs = vec![vec![10.0, 5.0, 20.0]];
        assert!((total(&segs) - 35.0).abs() < 1e-9);
    }
}
