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
    ctx.set_stroke_style(&theme.grid.into());
    ctx.set_line_width(0.5);
    ctx.set_fill_style(&theme.text_muted.into());
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

    ctx.set_stroke_style(&theme.grid.into());
    ctx.set_line_width(0.5);
    ctx.set_fill_style(&theme.text_muted.into());
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
        assert!(positions.len() > 0, "should produce grid positions");
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
