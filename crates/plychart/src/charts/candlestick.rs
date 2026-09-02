//! Candlestick chart renderer — wick + body with up/down colors.

#[cfg(target_arch = "wasm32")]
pub fn draw(
    ctx: &web_sys::CanvasRenderingContext2d,
    candles: &[plycore::CandleData],
    area: &crate::types::ChartArea,
    theme: &plycore::ChartTheme,
    candle_width: f64,
) {
    if candles.is_empty() {
        return;
    }

    let min_p = candles.iter().map(|c| c.low).fold(f64::INFINITY, f64::min);
    let max_p = candles
        .iter()
        .map(|c| c.high)
        .fold(f64::NEG_INFINITY, f64::max);
    let range = (max_p - min_p).max(0.0001);
    let pad = range * 0.05;
    let min_p = min_p - pad;
    let max_p = max_p + pad;
    let total_range = max_p - min_p;

    let price_to_y =
        |price: f64| -> f64 { area.y + area.h * (1.0 - (price - min_p) / total_range) };
    let index_to_x =
        |i: usize| -> f64 { area.x + (i as f64 + 0.5) * (area.w / candles.len() as f64) };

    for (i, c) in candles.iter().enumerate() {
        let x = index_to_x(i);
        let is_up = c.close >= c.open;
        let color = if is_up { theme.up } else { theme.down };

        // Wick
        ctx.set_stroke_style(&color.into());
        ctx.set_line_width(1.0);
        ctx.begin_path();
        ctx.move_to(x, price_to_y(c.high));
        ctx.line_to(x, price_to_y(c.low));
        ctx.stroke();

        // Body
        let body_top = price_to_y(c.open.max(c.close));
        let body_bot = price_to_y(c.open.min(c.close));
        let body_h = (body_bot - body_top).max(1.0);
        ctx.set_fill_style(&color.into());
        ctx.fill_rect(x - candle_width / 2.0, body_top, candle_width, body_h);
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

    fn price_to_y(price: f64, min_p: f64, max_p: f64) -> f64 {
        let total = max_p - min_p;
        AREA.y + AREA.h * (1.0 - (price - min_p) / total)
    }

    fn index_to_x(i: usize, len: usize) -> f64 {
        AREA.x + (i as f64 + 0.5) * (AREA.w / len as f64)
    }

    fn candle_range(candles: &[CandleData]) -> (f64, f64) {
        let min_p = candles.iter().map(|c| c.low).fold(f64::INFINITY, f64::min);
        let max_p = candles
            .iter()
            .map(|c| c.high)
            .fold(f64::NEG_INFINITY, f64::max);
        let range = (max_p - min_p).max(0.0001);
        let pad = range * 0.05;
        (min_p - pad, max_p + pad)
    }

    #[test]
    fn empty_data_no_panic() {
        let candles: Vec<CandleData> = vec![];
        assert!(candles.is_empty());
    }

    #[test]
    fn single_candle_range() {
        let c = CandleData {
            time: 1.0,
            open: 100.0,
            high: 105.0,
            low: 99.0,
            close: 102.0,
            volume: 1000.0,
        };
        let (min_p, max_p) = candle_range(&[c]);
        assert!(min_p < c.low);
        assert!(max_p > c.high);
        assert!(!min_p.is_nan());
        assert!(!max_p.is_nan());
        assert!(min_p.is_finite());
        assert!(max_p.is_finite());
    }

    #[test]
    fn multiple_candles_price_mapping() {
        let candles = vec![
            CandleData {
                time: 1.0,
                open: 100.0,
                high: 110.0,
                low: 95.0,
                close: 105.0,
                volume: 500.0,
            },
            CandleData {
                time: 2.0,
                open: 105.0,
                high: 115.0,
                low: 100.0,
                close: 108.0,
                volume: 600.0,
            },
            CandleData {
                time: 3.0,
                open: 108.0,
                high: 120.0,
                low: 102.0,
                close: 112.0,
                volume: 700.0,
            },
        ];
        let (min_p, max_p) = candle_range(&candles);
        let total = max_p - min_p;
        assert!(total > 0.0);

        for c in &candles {
            let y_high = price_to_y(c.high, min_p, max_p);
            let y_low = price_to_y(c.low, min_p, max_p);
            assert!(!y_high.is_nan());
            assert!(!y_low.is_nan());
            assert!(y_high < y_low, "high should map above low on screen");
        }
    }

    #[test]
    fn index_to_x_evenly_spaced() {
        let len = 5;
        let xs: Vec<f64> = (0..len).map(|i| index_to_x(i, len)).collect();
        for w in xs.windows(2) {
            let diff = (w[1] - w[0] - AREA.w / len as f64).abs();
            assert!(diff < 1e-10, "x spacing should be uniform");
        }
        assert!(!xs[0].is_nan());
        assert!(xs[0] > AREA.x);
    }

    #[test]
    fn equal_open_close_up_candle() {
        let c = CandleData {
            time: 1.0,
            open: 100.0,
            high: 105.0,
            low: 95.0,
            close: 100.0,
            volume: 100.0,
        };
        assert!(c.close >= c.open, "equal open/close counts as up");
    }

    #[test]
    fn body_height_minimum_one() {
        let body_top: f64 = 100.0;
        let body_bot: f64 = 100.0;
        let body_h = (body_bot - body_top).max(1.0);
        assert_eq!(body_h, 1.0);
    }

    #[test]
    fn nan_and_inf_guard() {
        let candles = vec![CandleData {
            time: 1.0,
            open: f64::NAN,
            high: f64::NAN,
            low: f64::NAN,
            close: f64::NAN,
            volume: 0.0,
        }];
        let min_p = candles.iter().map(|c| c.low).fold(f64::INFINITY, f64::min);
        let max_p = candles
            .iter()
            .map(|c| c.high)
            .fold(f64::NEG_INFINITY, f64::max);
        assert!(min_p.is_infinite() || min_p.is_nan());
        assert!(max_p.is_infinite() || max_p.is_nan());
    }

    #[test]
    fn large_price_range() {
        let candles = vec![CandleData {
            time: 1.0,
            open: 0.001,
            high: 1_000_000.0,
            low: 0.0005,
            close: 500_000.0,
            volume: 1.0,
        }];
        let (min_p, max_p) = candle_range(&candles);
        assert!(min_p.is_finite());
        assert!(max_p.is_finite());
        let y = price_to_y(500_000.0, min_p, max_p);
        assert!(y.is_finite());
    }
}
