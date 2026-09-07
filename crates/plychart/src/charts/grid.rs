//! Price and time grid rendering.

/// Draw horizontal price grid lines with labels.
#[cfg(target_arch = "wasm32")]
pub fn draw_price_grid(
    ctx: &web_sys::CanvasRenderingContext2d,
    area: &plycore::ChartArea,
    min_y: f64,
    max_y: f64,
    theme: &plycore::ChartTheme,
) {
    ctx.set_stroke_style_str(theme.grid);
    ctx.set_line_width(0.5);
    ctx.set_fill_style_str(theme.text_muted);
    ctx.set_font("9px 'JetBrains Mono', monospace");

    for i in 0..=4 {
        let price = min_y + (max_y - min_y) * (1.0 - i as f64 / 4.0);
        let y = area.y + area.h * (1.0 - i as f64 / 4.0);
        ctx.begin_path();
        ctx.move_to(area.x, y);
        ctx.line_to(area.x + area.w, y);
        ctx.stroke();
        let _ = ctx.fill_text(&format!("{:.2}", price), 4.0, y + 3.0);
    }
}

/// Draw vertical time grid lines with HH:MM labels.
#[cfg(target_arch = "wasm32")]
pub fn draw_time_grid(
    ctx: &web_sys::CanvasRenderingContext2d,
    area: &plycore::ChartArea,
    timestamps: &[f64],
    theme: &plycore::ChartTheme,
) {
    if timestamps.len() <= 1 {
        return;
    }

    ctx.set_stroke_style_str(theme.grid);
    ctx.set_line_width(0.5);
    ctx.set_fill_style_str(theme.text_muted);
    ctx.set_font("9px 'JetBrains Mono', monospace");

    let step = (timestamps.len() / 6).max(1);
    for i in (0..timestamps.len()).step_by(step) {
        let x = area.x + (i as f64 + 0.5) * (area.w / timestamps.len() as f64);
        ctx.begin_path();
        ctx.move_to(x, area.y);
        ctx.line_to(x, area.y + area.h);
        ctx.stroke();

        let total_secs = timestamps[i] as i64;
        let hours = (total_secs / 3600) % 24;
        let mins = (total_secs / 60) % 60;
        let _ = ctx.fill_text(
            &format!("{:02}:{:02}", hours, mins),
            x - 15.0,
            area.y + area.h + 14.0,
        );
    }
}

#[cfg(test)]
mod tests {
    use plycore::ChartArea;

    const AREA: ChartArea = ChartArea {
        x: 50.0,
        y: 20.0,
        w: 700.0,
        h: 350.0,
    };

    fn price_grid_lines(min_y: f64, max_y: f64, area: ChartArea) -> Vec<(f64, f64)> {
        (0..=4)
            .map(|i| {
                let price = min_y + (max_y - min_y) * (1.0 - i as f64 / 4.0);
                let y = area.y + area.h * (1.0 - i as f64 / 4.0);
                (price, y)
            })
            .collect()
    }

    fn time_grid_positions(timestamps: &[f64], area: ChartArea) -> Vec<(f64, i64, i64)> {
        if timestamps.len() <= 1 {
            return vec![];
        }
        let step = (timestamps.len() / 6).max(1);
        let mut result = vec![];
        for i in (0..timestamps.len()).step_by(step) {
            let x = area.x + (i as f64 + 0.5) * (area.w / timestamps.len() as f64);
            let total_secs = timestamps[i] as i64;
            let hours = (total_secs / 3600) % 24;
            let mins = (total_secs / 60) % 60;
            result.push((x, hours, mins));
        }
        result
    }

    #[test]
    fn price_grid_5_lines() {
        let lines = price_grid_lines(100.0, 200.0, AREA);
        assert_eq!(lines.len(), 5, "should have 5 grid lines");
    }

    #[test]
    fn price_grid_y_evenly_spaced() {
        let lines = price_grid_lines(0.0, 100.0, AREA);
        for w in lines.windows(2) {
            let diff = (w[1].1 - w[0].1 + AREA.h / 4.0).abs();
            assert!(diff < 1e-10, "y spacing should be uniform");
        }
    }

    #[test]
    fn price_grid_first_last_prices() {
        let lines = price_grid_lines(100.0, 200.0, AREA);
        assert!((lines[0].0 - 200.0).abs() < 1e-10, "first line at max_y");
        assert!((lines[4].0 - 100.0).abs() < 1e-10, "last line at min_y");
    }

    #[test]
    fn price_grid_all_y_finite() {
        let lines = price_grid_lines(-50.0, 500.0, AREA);
        for (price, y) in &lines {
            assert!(price.is_finite());
            assert!(y.is_finite());
        }
    }

    #[test]
    fn time_grid_empty_timestamps() {
        let positions = time_grid_positions(&[], AREA);
        assert!(positions.is_empty());
    }

    #[test]
    fn time_grid_single_timestamp() {
        let positions = time_grid_positions(&[1000.0], AREA);
        assert!(positions.is_empty(), "single timestamp returns early");
    }

    #[test]
    fn time_grid_multiple_timestamps() {
        let timestamps: Vec<f64> = (0..12).map(|i| (i * 3600) as f64).collect();
        let positions = time_grid_positions(&timestamps, AREA);
        assert!(!positions.is_empty(), "should produce grid positions");
        for (x, hours, mins) in &positions {
            assert!(x.is_finite());
            assert!(*hours >= 0 && *hours < 24);
            assert!(*mins >= 0 && *mins < 60);
        }
    }

    #[test]
    fn time_grid_x_within_area() {
        let timestamps: Vec<f64> = (0..24).map(|i| (i * 3600) as f64).collect();
        let positions = time_grid_positions(&timestamps, AREA);
        for (x, _, _) in &positions {
            assert!(*x >= AREA.x - 1.0);
            assert!(*x <= AREA.x + AREA.w + 1.0);
        }
    }

    #[test]
    fn time_grid_step_computation() {
        let len = 30;
        let step = (len / 6).max(1);
        assert_eq!(step, 5);
    }

    #[test]
    fn time_formatting() {
        let total_secs: i64 = 3600 * 14 + 60 * 30;
        let hours = (total_secs / 3600) % 24;
        let mins = (total_secs / 60) % 60;
        assert_eq!(hours, 14);
        assert_eq!(mins, 30);
    }

    #[test]
    fn midnight_wraparound() {
        let total_secs: i64 = 3600 * 25;
        let hours = (total_secs / 3600) % 24;
        assert_eq!(hours, 1, "25 hours wraps to 01");
    }
}

/// Draw axis labels for a time-series chart: 5 y-value ticks (left) and
/// 3 x-time ticks (bottom). Values are close-price based; times are unix
/// seconds formatted as MM-DD (or HH:MM for intraday spans < 2 days).
///
/// Called from the JS-facing canvas path (update_line/update_area/etc.)
/// so every WASM-consumed chart gets readable axes without changing the
/// bare `draw()` signatures used by native Rust consumers.
#[cfg(target_arch = "wasm32")]
pub fn draw_axis_labels(
    ctx: &web_sys::CanvasRenderingContext2d,
    area: &crate::types::ChartArea,
    theme: &crate::types::ChartTheme,
    min_val: f64,
    max_val: f64,
    times: Option<(f64, f64)>,
) {
    let range = (max_val - min_val).abs().max(1e-9);

    // Y ticks: 5 values, top to bottom.
    ctx.set_fill_style(&theme.text_muted.into());
    ctx.set_font("9px monospace");
    ctx.set_text_align("left");
    for i in 0..5 {
        let frac = i as f64 / 4.0;
        let val = max_val - range * frac;
        let y = area.y + area.h * frac;
        let label = if range >= 1000.0 {
            format!("{:.0}", val)
        } else if range >= 1.0 {
            format!("{:.2}", val)
        } else {
            format!("{:.4}", val)
        };
        let _ = ctx.fill_text(&label, area.x + 4.0, y + 3.0);
    }

    // X ticks: 3 time labels if timestamps are available.
    if let Some((t0, t1)) = times {
        let span_days = (t1 - t0).abs() / 86400.0;
        ctx.set_text_align("center");
        for i in 0..=2 {
            let frac = i as f64 / 2.0;
            let t = t0 + (t1 - t0) * frac;
            let x = area.x + area.w * frac;
            let label = if span_days < 2.0 {
                // HH:MM
                let h = (t / 3600.0).floor() % 24.0;
                let m = (t / 60.0).floor() % 60.0;
                format!("{:02.0}:{:02.0}", h, m)
            } else {
                // MM-DD from unix seconds
                let days = (t / 86400.0).floor() as i64;
                // days since epoch → month/day via civil-from-days algorithm
                let z = days + 719_468;
                let era = if z >= 0 { z } else { z - 145 } / 1461;
                let doe = z - era * 1461;
                let yoe = (doe - doe / 365 + doe / 1460) / 365;
                let mp = (5 * doe + 2) / 153;
                let month = if mp < 10 { mp + 3 } else { mp - 9 };
                let day = doe - (153 * mp + 2) / 5 + 1;
                format!("{:02}-{:02}", month, day)
            };
            let _ = ctx.fill_text(&label, x, area.y + area.h + 12.0);
        }
    }
}
