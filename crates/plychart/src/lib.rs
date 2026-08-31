//! plychart — Full-featured graphing library for Rust/WASM.
//!
//! Zero-dependency, zero-watermark, zero-JS Canvas2D charting.
//! Extracted from the hydrated_personal_site rendering infrastructure.

pub mod canvas;
pub mod charts;
pub mod error;
pub mod interaction;
pub mod theme;
pub mod types;

pub use charts::{ChartType, CandleData};
pub use error::ChartError;
pub use theme::get_theme;
pub use types::{ChartArea, ChartOpts, ChartTheme, ChartViewport};

/// Core canvas chart component.
/// Renders OHLCV data to an HTML5 Canvas element via Canvas2D.
pub struct CanvasChart {
    canvas_id: String,
    theme: ChartTheme,
    viewport: ChartViewport,
}

impl CanvasChart {
    /// Create a new chart bound to a canvas element.
    #[must_use]
    pub fn new(canvas_id: &str) -> Self {
        Self {
            canvas_id: canvas_id.to_string(),
            theme: ChartTheme::dark(),
            viewport: ChartViewport::default(),
        }
    }

    /// Set chart theme.
    pub fn with_theme(mut self, theme: ChartTheme) -> Self {
        self.theme = theme;
        self
    }

    /// Update chart data.
    pub fn update(&self, data: &[CandleData]) -> Result<(), ChartError> {
        canvas::update_candles(&self.canvas_id, data)
    }

    /// Get current viewport state.
    #[must_use]
    pub fn viewport(&self) -> ChartViewport {
        self.viewport
    }
}

/// High-level entry point for WASM.
pub fn create_canvas_chart(canvas_id: &str, width: u32, height: u32) -> Result<(), ChartError> {
    canvas::create_chart(canvas_id, width, height)
        .map_err(|e| ChartError::CanvasNotFound(format!("{e:?}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canvas_chart_new() {
        let chart = CanvasChart::new("my-canvas");
        assert_eq!(chart.canvas_id, "my-canvas");
    }

    #[test]
    fn canvas_chart_default_theme_is_dark() {
        let chart = CanvasChart::new("c");
        assert_eq!(chart.theme.bg, "#0a0a0a");
    }

    #[test]
    fn canvas_chart_with_theme() {
        let theme = ChartTheme::light();
        let chart = CanvasChart::new("c").with_theme(theme);
        assert_eq!(chart.theme.bg, "#ffffff");
    }

    #[test]
    fn canvas_chart_viewport_default() {
        let chart = CanvasChart::new("c");
        let vp = chart.viewport();
        assert_eq!(vp.start, 0);
        assert_eq!(vp.count, 100);
        assert!(!vp.log_scale);
    }

    #[test]
    fn create_canvas_chart_native_ok() {
        let result = create_canvas_chart("nonexistent", 800, 600);
        assert!(result.is_ok());
    }

    #[test]
    fn get_theme_dark() {
        let t = get_theme(true);
        assert_eq!(t.bg, "#0a0a0a");
    }

    #[test]
    fn get_theme_light() {
        let t = get_theme(false);
        assert_eq!(t.bg, "#ffffff");
    }

    #[test]
    fn chart_type_all_count() {
        assert_eq!(ChartType::all().len(), 13);
    }

    #[test]
    fn chart_type_labels() {
        assert_eq!(ChartType::Candlestick.label(), "Candles");
        assert_eq!(ChartType::Line.label(), "Line");
        assert_eq!(ChartType::Area.label(), "Area");
        assert_eq!(ChartType::Bar.label(), "Bars");
        assert_eq!(ChartType::Heatmap.label(), "Heatmap");
        assert_eq!(ChartType::Scatter.label(), "Scatter");
        assert_eq!(ChartType::Gauge.label(), "Gauge");
        assert_eq!(ChartType::Radar.label(), "Radar");
        assert_eq!(ChartType::Treemap.label(), "Treemap");
        assert_eq!(ChartType::Waterfall.label(), "Waterfall");
        assert_eq!(ChartType::OrderBook.label(), "Order Book");
        assert_eq!(ChartType::Backtest.label(), "Backtest");
        assert_eq!(ChartType::Correlation.label(), "Correlation");
    }

    #[test]
    fn chart_type_equality() {
        assert_eq!(ChartType::Candlestick, ChartType::Candlestick);
        assert_ne!(ChartType::Candlestick, ChartType::Line);
    }

    #[test]
    fn canvas_update_empty_data() {
        let chart = CanvasChart::new("c");
        let result = chart.update(&[]);
        assert!(result.is_ok());
    }

    #[test]
    fn canvas_update_single_candle() {
        let chart = CanvasChart::new("c");
        let data = vec![CandleData { time: 1.0, open: 100.0, high: 105.0, low: 99.0, close: 102.0, volume: 500.0 }];
        let result = chart.update(&data);
        assert!(result.is_ok());
    }

    #[test]
    fn canvas_update_multiple_candles() {
        let chart = CanvasChart::new("c");
        let data: Vec<CandleData> = (0..10)
            .map(|i| CandleData {
                time: i as f64,
                open: 100.0 + i as f64,
                high: 101.0 + i as f64,
                low: 99.0 + i as f64,
                close: 100.5 + i as f64,
                volume: 1000.0,
            })
            .collect();
        let result = chart.update(&data);
        assert!(result.is_ok());
    }

    #[test]
    fn device_pixel_ratio_native() {
        let dpr = canvas::device_pixel_ratio();
        assert_eq!(dpr, 1.0);
    }

    #[test]
    fn canvas_destroy_chart() {
        let result = canvas::destroy_chart("any-id");
        assert!(result.is_ok());
    }

    #[test]
    fn canvas_update_heatmap() {
        let result = canvas::update_heatmap("c", "[]");
        assert!(result.is_ok());
    }

    #[test]
    fn canvas_update_order_book() {
        let result = canvas::update_order_book("c", "[]");
        assert!(result.is_ok());
    }

    #[test]
    fn candle_data_construction() {
        let c = CandleData {
            time: 1700000000.0,
            open: 150.0,
            high: 155.0,
            low: 148.0,
            close: 153.0,
            volume: 1_000_000.0,
        };
        assert_eq!(c.time, 1700000000.0);
        assert_eq!(c.open, 150.0);
        assert_eq!(c.high, 155.0);
        assert_eq!(c.low, 148.0);
        assert_eq!(c.close, 153.0);
        assert_eq!(c.volume, 1_000_000.0);
    }

    #[test]
    fn candle_data_serialization_roundtrip() {
        let data = vec![
            CandleData { time: 1.0, open: 100.0, high: 105.0, low: 99.0, close: 102.0, volume: 500.0 },
            CandleData { time: 2.0, open: 102.0, high: 108.0, low: 101.0, close: 107.0, volume: 600.0 },
        ];
        let json = serde_json::to_string(&data).unwrap();
        let back: Vec<CandleData> = serde_json::from_str(&json).unwrap();
        assert_eq!(data.len(), back.len());
        assert_eq!(data[0].close, back[0].close);
        assert_eq!(data[1].high, back[1].high);
    }

    #[test]
    fn chart_theme_serde_roundtrip_via_viewport() {
        let vp = ChartViewport { start: 10, count: 50, log_scale: true };
        assert_eq!(vp.start, 10);
        assert_eq!(vp.count, 50);
        assert!(vp.log_scale);
    }
}
