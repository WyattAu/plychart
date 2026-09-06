//! Histogram chart renderer — bins data into buckets and renders as bars.

#[cfg(target_arch = "wasm32")]
pub fn draw(
    ctx: &web_sys::CanvasRenderingContext2d,
    values: &[f64],
    bin_count: usize,
    area: &plycore::ChartArea,
    theme: &plycore::ChartTheme,
) {
    draw_impl(ctx, values, bin_count, false, area, theme)
}

/// Log-scale variant: bar heights scale with log10(count), so rare tail
/// bins stay visible next to a dominant mode.
#[cfg(target_arch = "wasm32")]
pub fn draw_log(
    ctx: &web_sys::CanvasRenderingContext2d,
    values: &[f64],
    bin_count: usize,
    area: &plycore::ChartArea,
    theme: &plycore::ChartTheme,
) {
    draw_impl(ctx, values, bin_count, true, area, theme)
}

#[cfg(target_arch = "wasm32")]
fn draw_impl(
    ctx: &web_sys::CanvasRenderingContext2d,
    values: &[f64],
    bin_count: usize,
    log_scale: bool,
    area: &plycore::ChartArea,
    theme: &plycore::ChartTheme,
) {
    let Some((min, max, bins)) = compute_bins(values, bin_count) else {
        if !values.is_empty() && bin_count > 0 {
            // All values identical — draw single bar
            ctx.set_fill_style_str(theme.accent);
            ctx.fill_rect(area.x, area.y, area.w, area.h);
        }
        return;
    };

    let max_bin = *bins.iter().max().unwrap_or(&1).max(&1);
    let padding = 4.0;
    let bar_area_w = area.w - padding * 2.0;
    let bar_area_h = area.h - padding * 2.0;
    let bar_w = bar_area_w / bin_count as f64;
    let gap = (bar_w * 0.15).max(1.0);
    let max_log = (max_bin as f64).log10().max(1e-9);

    for (i, &count) in bins.iter().enumerate() {
        let frac = if log_scale && count > 0 {
            (count as f64).log10() / max_log
        } else {
            count as f64 / max_bin as f64
        };
        let bar_h = frac.max(0.02) * bar_area_h;
        let x = area.x + padding + i as f64 * bar_w;
        let y = area.y + padding + bar_area_h - bar_h;

        // Color: accent for normal, down for tail (last 20% of bins)
        let is_tail = i as f64 / bin_count as f64 > 0.8;
        let color = if is_tail { theme.down } else { theme.accent };

        ctx.set_fill_style_str(color);
        ctx.fill_rect(x + gap / 2.0, y, bar_w - gap, bar_h);
    }

    // Axis labels
    ctx.set_fill_style_str(theme.text_muted);
    ctx.set_font("9px monospace");
    ctx.set_text_align("left");
    ctx.fill_text(&format!("{min:.2}"), area.x + 2.0, area.y + area.h - 2.0)
        .unwrap_or_default();
    ctx.set_text_align("right");
    ctx.fill_text(
        &format!("{max:.2}"),
        area.x + area.w - 2.0,
        area.y + area.h - 2.0,
    )
    .unwrap_or_default();
}

/// Compute histogram bins from raw values.
///
/// Returns `(min, max, counts)` where `counts.len() == bin_count` and the
/// maximum value is clamped into the last bin. Returns `None` when `values`
/// is empty, `bin_count` is zero, or all values are identical (degenerate
/// range) — callers render those cases as a single bar or nothing.
#[must_use]
pub fn compute_bins(values: &[f64], bin_count: usize) -> Option<(f64, f64, Vec<usize>)> {
    if values.is_empty() || bin_count == 0 {
        return None;
    }

    let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    if (max - min).abs() < f64::EPSILON {
        return None;
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

    Some((min, max, bins))
}

#[cfg(test)]
mod tests {
    use super::compute_bins;

    #[test]
    fn compute_bins_uniform() {
        let values: Vec<f64> = (1..=10).map(f64::from).collect();
        let (min, max, bins) = compute_bins(&values, 5).expect("bins");
        assert_eq!((min, max), (1.0, 10.0));
        assert_eq!(bins, vec![2, 2, 2, 2, 2]);
    }

    #[test]
    fn compute_bins_clamps_max_into_last_bin() {
        // 1..=10 in 3 bins: width 3, so 10 would land on idx 3 — clamps to last.
        let values: Vec<f64> = (1..=10).map(f64::from).collect();
        let (_, _, bins) = compute_bins(&values, 3).expect("bins");
        assert_eq!(bins, vec![3, 3, 4]);
        assert_eq!(bins.iter().sum::<usize>(), 10);
    }

    #[test]
    fn compute_bins_empty_or_zero_is_none() {
        assert!(compute_bins(&[], 5).is_none());
        assert!(compute_bins(&[1.0, 2.0], 0).is_none());
    }

    #[test]
    fn compute_bins_identical_values_is_none() {
        assert!(compute_bins(&[2.5; 8], 4).is_none());
    }

    #[test]
    fn compute_bins_preserves_total_count() {
        let values: Vec<f64> = (0..1000).map(|i| (i as f64 * 0.7).sin() * 50.0).collect();
        let (_, _, bins) = compute_bins(&values, 20).expect("bins");
        assert_eq!(bins.len(), 20);
        assert_eq!(bins.iter().sum::<usize>(), 1000);
    }
}
