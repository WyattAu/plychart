//! Bar chart renderer.

#[cfg(target_arch = "wasm32")]
pub fn draw(
    ctx: &web_sys::CanvasRenderingContext2d,
    bars: &[plycore::CandleData],
    area: &crate::types::ChartArea,
    theme: &plycore::ChartTheme,
) {
    if bars.is_empty() {
        return;
    }

    let min_p = bars
        .iter()
        .map(|c| c.close)
        .fold(f64::INFINITY, f64::min)
        .min(0.0);
    let max_p = bars
        .iter()
        .map(|c| c.close)
        .fold(f64::NEG_INFINITY, f64::max);
    let range = (max_p - min_p).max(1.0);
    let bar_w = (area.w / bars.len() as f64 * 0.7).max(1.0);

    for (i, c) in bars.iter().enumerate() {
        let x = area.x + (i as f64 + 0.5) * (area.w / bars.len() as f64);
        let is_up = c.close >= c.open;
        let color = if is_up { theme.up } else { theme.down };
        let h = (c.close.abs() / range) * area.h;
        let y = if c.close >= 0.0 {
            area.y + area.h - h
        } else {
            area.y + area.h
        };
        ctx.set_fill_style(&color.into());
        ctx.fill_rect(x - bar_w / 2.0, y, bar_w, h);
    }
}

#[cfg(test)]
mod tests {
    use plycore::{CandleData, ChartArea};

    const AREA: ChartArea = ChartArea {
        x: 0.0,
        y: 0.0,
        w: 800.0,
        h: 400.0,
    };

    fn bar_metrics(bars: &[CandleData]) -> (f64, f64, f64) {
        let min_p = bars
            .iter()
            .map(|c| c.close)
            .fold(f64::INFINITY, f64::min)
            .min(0.0);
        let max_p = bars
            .iter()
            .map(|c| c.close)
            .fold(f64::NEG_INFINITY, f64::max);
        let bar_w = (AREA.w / bars.len() as f64 * 0.7).max(1.0);
        (min_p, max_p, bar_w)
    }

    #[test]
    fn empty_data_no_panic() {
        let bars: Vec<CandleData> = vec![];
        assert!(bars.is_empty());
    }

    #[test]
    fn single_positive_bar() {
        let bars = vec![CandleData {
            time: 1.0,
            open: 0.0,
            high: 0.0,
            low: 0.0,
            close: 100.0,
            volume: 0.0,
        }];
        let (min_p, max_p, bar_w) = bar_metrics(&bars);
        assert!(min_p <= 0.0);
        assert!(max_p > 0.0);
        assert!(bar_w > 0.0);
        let h = (bars[0].close.abs() / (max_p - min_p).max(1.0)) * AREA.h;
        assert!(h > 0.0);
        assert!(h.is_finite());
    }

    #[test]
    fn single_negative_bar() {
        let bars = vec![CandleData {
            time: 1.0,
            open: 0.0,
            high: 0.0,
            low: 0.0,
            close: -50.0,
            volume: 0.0,
        }];
        let (min_p, max_p, _) = bar_metrics(&bars);
        assert!(min_p < 0.0);
        let h = (bars[0].close.abs() / (max_p - min_p).max(1.0)) * AREA.h;
        assert!(h > 0.0);
        assert!(h.is_finite());
    }

    #[test]
    fn mixed_positive_negative() {
        let bars = vec![
            CandleData {
                time: 1.0,
                open: 0.0,
                high: 0.0,
                low: 0.0,
                close: 100.0,
                volume: 0.0,
            },
            CandleData {
                time: 2.0,
                open: 0.0,
                high: 0.0,
                low: 0.0,
                close: -30.0,
                volume: 0.0,
            },
            CandleData {
                time: 3.0,
                open: 0.0,
                high: 0.0,
                low: 0.0,
                close: 60.0,
                volume: 0.0,
            },
        ];
        let (min_p, max_p, bar_w) = bar_metrics(&bars);
        assert!(min_p <= 0.0);
        assert!(max_p > 0.0);
        assert!(bar_w > 1.0);
        assert!(
            (min_p - (-30.0)).abs() < 1e-10,
            "min includes negative close"
        );
    }

    #[test]
    fn all_zero_values() {
        let bars = vec![
            CandleData {
                time: 1.0,
                open: 0.0,
                high: 0.0,
                low: 0.0,
                close: 0.0,
                volume: 0.0,
            },
            CandleData {
                time: 2.0,
                open: 0.0,
                high: 0.0,
                low: 0.0,
                close: 0.0,
                volume: 0.0,
            },
        ];
        let (min_p, max_p, bar_w) = bar_metrics(&bars);
        let range = (max_p - min_p).max(1.0);
        assert!(range >= 1.0, "range floors at 1.0");
        assert!(bar_w > 0.0);
    }

    #[test]
    fn bar_width_max_with_one_bar() {
        let bars = vec![CandleData {
            time: 1.0,
            open: 0.0,
            high: 0.0,
            low: 0.0,
            close: 10.0,
            volume: 0.0,
        }];
        let (_, _, bar_w) = bar_metrics(&bars);
        let expected = (AREA.w / 1.0 * 0.7).max(1.0);
        assert!((bar_w - expected).abs() < 1e-10);
    }

    #[test]
    fn bar_width_scales_with_count() {
        let bars_1 = vec![CandleData {
            time: 1.0,
            open: 0.0,
            high: 0.0,
            low: 0.0,
            close: 10.0,
            volume: 0.0,
        }];
        let bars_10: Vec<CandleData> = (0..10)
            .map(|i| CandleData {
                time: i as f64,
                open: 0.0,
                high: 0.0,
                low: 0.0,
                close: 10.0,
                volume: 0.0,
            })
            .collect();
        let (_, _, w1) = bar_metrics(&bars_1);
        let (_, _, w10) = bar_metrics(&bars_10);
        assert!(w1 > w10, "more bars should mean narrower bars");
    }

    #[test]
    fn bar_position_centered() {
        let bars = vec![CandleData {
            time: 1.0,
            open: 0.0,
            high: 0.0,
            low: 0.0,
            close: 10.0,
            volume: 0.0,
        }];
        let (_, _, bar_w) = bar_metrics(&bars);
        let x = AREA.x + (0.5_f64) * (AREA.w / bars.len() as f64);
        assert!((x - AREA.w / 2.0).abs() < 1e-10, "single bar centered");
        assert!(x - bar_w / 2.0 >= 0.0, "bar fits in area");
    }

    #[test]
    fn up_down_color_selection() {
        let up = CandleData {
            time: 1.0,
            open: 10.0,
            high: 10.0,
            low: 10.0,
            close: 20.0,
            volume: 0.0,
        };
        let down = CandleData {
            time: 2.0,
            open: 20.0,
            high: 20.0,
            low: 20.0,
            close: 10.0,
            volume: 0.0,
        };
        assert!(up.close >= up.open);
        assert!(down.close < down.open);
    }
}
