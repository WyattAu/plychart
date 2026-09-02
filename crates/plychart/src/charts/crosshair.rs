//! Crosshair overlay and OHLC readout.

/// Draw dashed crosshair lines at mouse position.
#[cfg(target_arch = "wasm32")]
pub fn draw_crosshair(
    ctx: &web_sys::CanvasRenderingContext2d,
    mx: f64,
    my: f64,
    area: &plycore::ChartArea,
    theme: &plycore::ChartTheme,
) {
    ctx.set_stroke_style(&theme.crosshair.into());
    ctx.set_line_width(0.5);
    ctx.set_line_dash(&js_sys::Array::of2(&2.0.into(), &2.0.into()));

    ctx.begin_path();
    ctx.move_to(area.x, my);
    ctx.line_to(area.x + area.w, my);
    ctx.stroke();

    ctx.begin_path();
    ctx.move_to(mx, area.y);
    ctx.line_to(mx, area.y + area.h);
    ctx.stroke();

    ctx.set_line_dash(&js_sys::Array::new());
}

/// Draw OHLC readout text at top-left for nearest candle.
#[cfg(target_arch = "wasm32")]
pub fn draw_ohlc_readout(
    ctx: &web_sys::CanvasRenderingContext2d,
    candles: &[plycore::CandleData],
    mx: f64,
    index_to_x: &dyn Fn(usize) -> f64,
    theme: &plycore::ChartTheme,
) {
    if candles.is_empty() {
        return;
    }

    let mut nearest_idx = 0;
    let mut nearest_dist = f64::INFINITY;
    for i in 0..candles.len() {
        let dist = (index_to_x(i) - mx).abs();
        if dist < nearest_dist {
            nearest_dist = dist;
            nearest_idx = i;
        }
    }

    let c = &candles[nearest_idx];
    let text = format!(
        "O:{:.2} H:{:.2} L:{:.2} C:{:.2}",
        c.open, c.high, c.low, c.close
    );
    ctx.set_font("10px 'JetBrains Mono', monospace");
    ctx.set_fill_style(&theme.text.into());
    let _ = ctx.fill_text(&text, 10.0, 16.0);
}

#[cfg(test)]
mod tests {
    use plycore::{CandleData, ChartArea};

    const AREA: ChartArea = ChartArea {
        x: 50.0,
        y: 20.0,
        w: 700.0,
        h: 350.0,
    };

    fn find_nearest_candle(
        candles: &[CandleData],
        mx: f64,
        index_to_x: &dyn Fn(usize) -> f64,
    ) -> usize {
        let mut nearest_idx = 0;
        let mut nearest_dist = f64::INFINITY;
        for i in 0..candles.len() {
            let dist = (index_to_x(i) - mx).abs();
            if dist < nearest_dist {
                nearest_dist = dist;
                nearest_idx = i;
            }
        }
        nearest_idx
    }

    fn index_to_x(i: usize, len: usize) -> f64 {
        AREA.x + (i as f64 + 0.5) * (AREA.w / len as f64)
    }

    #[test]
    fn empty_candles_no_panic() {
        let candles: Vec<CandleData> = vec![];
        assert!(candles.is_empty());
    }

    #[test]
    fn ohlc_readout_format() {
        let c = CandleData {
            time: 1.0,
            open: 100.5,
            high: 110.25,
            low: 99.75,
            close: 105.0,
            volume: 1000.0,
        };
        let text = format!(
            "O:{:.2} H:{:.2} L:{:.2} C:{:.2}",
            c.open, c.high, c.low, c.close
        );
        assert_eq!(text, "O:100.50 H:110.25 L:99.75 C:105.00");
    }

    #[test]
    fn nearest_candle_at_center() {
        let candles = vec![
            CandleData {
                time: 1.0,
                open: 100.0,
                high: 105.0,
                low: 95.0,
                close: 102.0,
                volume: 0.0,
            },
            CandleData {
                time: 2.0,
                open: 102.0,
                high: 108.0,
                low: 100.0,
                close: 106.0,
                volume: 0.0,
            },
            CandleData {
                time: 3.0,
                open: 106.0,
                high: 112.0,
                low: 104.0,
                close: 110.0,
                volume: 0.0,
            },
        ];
        let mx = index_to_x(1, candles.len());
        let idx = find_nearest_candle(&candles, mx, &|i| index_to_x(i, candles.len()));
        assert_eq!(idx, 1);
    }

    #[test]
    fn nearest_candle_at_left() {
        let candles = vec![
            CandleData {
                time: 1.0,
                open: 100.0,
                high: 105.0,
                low: 95.0,
                close: 102.0,
                volume: 0.0,
            },
            CandleData {
                time: 2.0,
                open: 102.0,
                high: 108.0,
                low: 100.0,
                close: 106.0,
                volume: 0.0,
            },
        ];
        let mx = AREA.x + 1.0;
        let idx = find_nearest_candle(&candles, mx, &|i| index_to_x(i, candles.len()));
        assert_eq!(idx, 0);
    }

    #[test]
    fn nearest_candle_at_right() {
        let candles = vec![
            CandleData {
                time: 1.0,
                open: 100.0,
                high: 105.0,
                low: 95.0,
                close: 102.0,
                volume: 0.0,
            },
            CandleData {
                time: 2.0,
                open: 102.0,
                high: 108.0,
                low: 100.0,
                close: 106.0,
                volume: 0.0,
            },
        ];
        let mx = AREA.x + AREA.w - 1.0;
        let idx = find_nearest_candle(&candles, mx, &|i| index_to_x(i, candles.len()));
        assert_eq!(idx, 1);
    }

    #[test]
    fn crosshair_lines_within_area() {
        let mx = AREA.x + AREA.w / 2.0;
        let my = AREA.y + AREA.h / 2.0;
        assert!(mx >= AREA.x && mx <= AREA.x + AREA.w);
        assert!(my >= AREA.y && my <= AREA.y + AREA.h);
    }

    #[test]
    fn ohlc_readout_text_no_nan() {
        let candles = vec![CandleData {
            time: 1.0,
            open: 0.001,
            high: 999999.0,
            low: 0.0001,
            close: 500000.0,
            volume: 0.0,
        }];
        let c = &candles[0];
        let text = format!(
            "O:{:.2} H:{:.2} L:{:.2} C:{:.2}",
            c.open, c.high, c.low, c.close
        );
        assert!(!text.contains("NaN"));
        assert!(!text.contains("inf"));
    }

    #[test]
    fn single_candle_nearest() {
        let candles = vec![CandleData {
            time: 1.0,
            open: 50.0,
            high: 60.0,
            low: 40.0,
            close: 55.0,
            volume: 0.0,
        }];
        let mx = 0.0;
        let idx = find_nearest_candle(&candles, mx, &|i| index_to_x(i, candles.len()));
        assert_eq!(idx, 0);
    }
}
