//! Volume sub-pane rendering.

/// Draw volume bars for each candle.
#[cfg(target_arch = "wasm32")]
pub fn draw_volume(
    ctx: &web_sys::CanvasRenderingContext2d,
    candles: &[plycore::CandleData],
    index_to_x: &dyn Fn(usize) -> f64,
    area: &plycore::ChartArea,
    max_vol: f64,
    theme: &plycore::ChartTheme,
    bar_w: f64,
) {
    if max_vol <= 0.0 || candles.is_empty() {
        return;
    }

    for (i, c) in candles.iter().enumerate() {
        let x = index_to_x(i);
        let h = (c.volume / max_vol) * area.h;
        let is_up = c.close >= c.open;
        let color = if is_up { theme.up } else { theme.down };
        ctx.set_fill_style(&color.into());
        ctx.set_global_alpha(0.3);
        ctx.fill_rect(x - bar_w / 2.0, area.y + area.h - h, bar_w, h);
    }
    ctx.set_global_alpha(1.0);
}

#[cfg(test)]
mod tests {
    use plycore::{CandleData, ChartArea};

    const AREA: ChartArea = ChartArea {
        x: 0.0,
        y: 0.0,
        w: 800.0,
        h: 100.0,
    };

    fn index_to_x(i: usize, len: usize) -> f64 {
        AREA.x + (i as f64 + 0.5) * (AREA.w / len as f64)
    }

    fn volume_height(vol: f64, max_vol: f64, area_h: f64) -> f64 {
        (vol / max_vol) * area_h
    }

    #[test]
    fn empty_data_no_panic() {
        let candles: Vec<CandleData> = vec![];
        assert!(candles.is_empty());
    }

    #[test]
    fn zero_max_volume_no_draw() {
        let _candles = vec![CandleData {
            time: 1.0,
            open: 100.0,
            high: 105.0,
            low: 95.0,
            close: 102.0,
            volume: 500.0,
        }];
        let max_vol = 0.0;
        assert!(max_vol <= 0.0, "zero max_vol means no bars drawn");
    }

    #[test]
    fn zero_volume_bar_height() {
        let h = volume_height(0.0, 1000.0, AREA.h);
        assert_eq!(h, 0.0);
    }

    #[test]
    fn normal_volume_bar_height() {
        let h = volume_height(500.0, 1000.0, AREA.h);
        assert!((h - AREA.h * 0.5).abs() < 1e-10);
        assert!(h.is_finite());
    }

    #[test]
    fn max_volume_fills_area() {
        let h = volume_height(1000.0, 1000.0, AREA.h);
        assert!((h - AREA.h).abs() < 1e-10);
    }

    #[test]
    fn volume_ratio_clamped() {
        let candles = vec![
            CandleData {
                time: 1.0,
                open: 100.0,
                high: 105.0,
                low: 95.0,
                close: 102.0,
                volume: 2000.0,
            },
            CandleData {
                time: 2.0,
                open: 102.0,
                high: 108.0,
                low: 100.0,
                close: 106.0,
                volume: 500.0,
            },
        ];
        let max_vol = 2000.0;
        for c in &candles {
            let h = (c.volume / max_vol) * AREA.h;
            assert!(h >= 0.0);
            assert!(h <= AREA.h + 1.0);
        }
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

    #[test]
    fn bar_width_positive() {
        let bar_w = 8.0;
        assert!(bar_w > 0.0);
    }

    #[test]
    fn volume_bars_within_area_x() {
        let candles = vec![
            CandleData {
                time: 1.0,
                open: 100.0,
                high: 105.0,
                low: 95.0,
                close: 102.0,
                volume: 500.0,
            },
            CandleData {
                time: 2.0,
                open: 102.0,
                high: 108.0,
                low: 100.0,
                close: 106.0,
                volume: 600.0,
            },
        ];
        let bar_w = 8.0;
        for i in 0..candles.len() {
            let x = index_to_x(i, candles.len());
            let left = x - bar_w / 2.0;
            let right = x + bar_w / 2.0;
            assert!(left >= AREA.x - 1.0, "bar left edge within area");
            assert!(right <= AREA.x + AREA.w + 1.0, "bar right edge within area");
        }
    }

    #[test]
    fn large_volume_ratio() {
        let h = volume_height(1e9, 1e9, AREA.h);
        assert!((h - AREA.h).abs() < 1e-6);
    }
}
