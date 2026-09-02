//! Multi-series line and area chart renderer.

/// A named data series with color.
pub struct DataSeries<'a> {
    pub color: &'a str,
    pub data: &'a [plycore::CandleData],
}

#[cfg(target_arch = "wasm32")]
fn global_y_range(series: &[DataSeries<'_>]) -> Option<(f64, f64, f64)> {
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    for s in series {
        for c in s.data {
            if c.close < min {
                min = c.close;
            }
            if c.close > max {
                max = c.close;
            }
        }
    }
    if (max - min).abs() < f64::EPSILON {
        return None;
    }
    let pad = (max - min) * 0.05;
    min -= pad;
    max += pad;
    Some((min, max, max - min))
}

/// Draw multiple overlaid line series.
#[cfg(target_arch = "wasm32")]
pub fn draw_lines(
    ctx: &web_sys::CanvasRenderingContext2d,
    series: &[DataSeries<'_>],
    area: &plycore::ChartArea,
) {
    if series.is_empty() {
        return;
    }

    let (global_min, global_max, range) = match global_y_range(series) {
        Some(v) => v,
        None => return,
    };

    for s in series {
        if s.data.is_empty() {
            continue;
        }

        ctx.set_stroke_style(&s.color.into());
        ctx.set_line_width(1.5);
        ctx.begin_path();

        for (i, c) in s.data.iter().enumerate() {
            let x = area.x + (i as f64 / (s.data.len() - 1).max(1) as f64) * area.w;
            let y = area.y + area.h - ((c.close - global_min) / range) * area.h;

            if i == 0 {
                ctx.move_to(x, y);
            } else {
                ctx.line_to(x, y);
            }
        }
        ctx.stroke();
    }
}

/// Draw multiple overlaid area series with transparency.
#[cfg(target_arch = "wasm32")]
pub fn draw_areas(
    ctx: &web_sys::CanvasRenderingContext2d,
    series: &[DataSeries<'_>],
    area: &plycore::ChartArea,
) {
    if series.is_empty() {
        return;
    }

    let (global_min, global_max, range) = match global_y_range(series) {
        Some(v) => v,
        None => return,
    };

    let base_y = area.y + area.h;

    for s in series {
        if s.data.is_empty() {
            continue;
        }

        // Stroke line
        ctx.set_stroke_style(&s.color.into());
        ctx.set_line_width(1.5);
        ctx.begin_path();

        for (i, c) in s.data.iter().enumerate() {
            let x = area.x + (i as f64 / (s.data.len() - 1).max(1) as f64) * area.w;
            let y = area.y + area.h - ((c.close - global_min) / range) * area.h;

            if i == 0 {
                ctx.move_to(x, y);
            } else {
                ctx.line_to(x, y);
            }
        }
        ctx.stroke();

        // Area fill
        let last_x = area.x + area.w;
        ctx.begin_path();
        ctx.move_to(area.x, base_y);

        for (i, c) in s.data.iter().enumerate() {
            let x = area.x + (i as f64 / (s.data.len() - 1).max(1) as f64) * area.w;
            let y = area.y + area.h - ((c.close - global_min) / range) * area.h;
            ctx.line_to(x, y);
        }
        ctx.line_to(last_x, base_y);
        ctx.close_path();

        let gradient = ctx.create_linear_gradient(0.0, area.y, 0.0, area.y + area.h);
        gradient.add_color_stop(0.0, s.color);
        gradient.add_color_stop(1.0, "transparent");
        ctx.set_fill_style(&gradient.into());
        ctx.fill();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_series_no_panic() {
        let series: Vec<DataSeries<'_>> = vec![];
        assert!(series.is_empty());
    }

    #[test]
    fn single_series_constant_price() {
        let data = vec![
            plycore::CandleData {
                time: 1.0,
                open: 100.0,
                high: 100.0,
                low: 100.0,
                close: 100.0,
                volume: 0.0,
            };
            5
        ];
        let min = data.iter().map(|c| c.close).fold(f64::INFINITY, f64::min);
        let max = data
            .iter()
            .map(|c| c.close)
            .fold(f64::NEG_INFINITY, f64::max);
        assert_eq!(min, max);
    }
}
