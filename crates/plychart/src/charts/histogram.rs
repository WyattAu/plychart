//! Histogram chart renderer — bins data into buckets and renders as bars.

#[cfg(target_arch = "wasm32")]
pub fn draw(
    ctx: &web_sys::CanvasRenderingContext2d,
    values: &[f64],
    bin_count: usize,
    area: &plycore::ChartArea,
    theme: &plycore::ChartTheme,
) {
    if values.is_empty() || bin_count == 0 {
        return;
    }

    let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    if (max - min).abs() < f64::EPSILON {
        // All values identical — draw single bar
        ctx.set_fill_style(&theme.accent.into());
        ctx.fill_rect(
            area.x,
            area.y,
            area.w,
            area.h,
        )
        .unwrap_or_default();
        return;
    }

    let bin_width = (max - min) / bin_count as f64;
    let mut bins = vec![0usize; bin_count];

    for &v in values {
        let mut idx = ((v - min) / bin_width) as usize;
        if idx >= bin_count {
            idx = bin_count - 1;
        }
        bins[idx] += 1;
    }

    let max_bin = *bins.iter().max().unwrap_or(&1).max(&1);
    let padding = 4.0;
    let bar_area_w = area.w - padding * 2.0;
    let bar_area_h = area.h - padding * 2.0;
    let bar_w = bar_area_w / bin_count as f64;
    let gap = (bar_w * 0.15).max(1.0);

    for (i, &count) in bins.iter().enumerate() {
        let bar_h = (count as f64 / max_bin as f64) * bar_area_h;
        let x = area.x + padding + i as f64 * bar_w;
        let y = area.y + padding + bar_area_h - bar_h;

        // Color: accent for normal, down for tail (last 20% of bins)
        let is_tail = i as f64 / bin_count as f64 > 0.8;
        let color = if is_tail { theme.down } else { theme.accent };

        ctx.set_fill_style(&color.into());
        ctx.fill_rect(x + gap / 2.0, y, bar_w - gap, bar_h)
            .unwrap_or_default();
    }

    // Axis labels
    ctx.set_fill_style(&theme.text_muted.into());
    ctx.set_font("9px monospace");
    ctx.set_text_align("left");
    ctx.fill_text(&format!("{min:.2}"), area.x + 2.0, area.y + area.h - 2.0)
        .unwrap_or_default();
    ctx.set_text_align("right");
    ctx.fill_text(&format!("{max:.2}"), area.x + area.w - 2.0, area.y + area.h - 2.0)
        .unwrap_or_default();
}

#[cfg(test)]
mod tests {
    #[test]
    fn placeholder() {
        // Histogram rendering requires WASM; test binning logic here
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let bin_count = 5;
        let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let bin_width = (max - min) / bin_count as f64;
        let mut bins = vec![0usize; bin_count];
        for &v in &values {
            let mut idx = ((v - min) / bin_width) as usize;
            if idx >= bin_count {
                idx = bin_count - 1;
            }
            bins[idx] += 1;
        }
        assert_eq!(bins, vec![2, 2, 2, 2, 2]);
    }
}
