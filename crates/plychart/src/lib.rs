//! plychart — Full-featured graphing library for Rust/WASM.
//!
//! Zero-dependency, zero-watermark, zero-JS Canvas2D charting.

pub mod canvas;
pub mod charts;
pub mod interaction;
pub mod theme;
pub mod types;
#[cfg(target_arch = "wasm32")]
pub mod wasm;

pub use charts::{CandleData, ChartType};
pub use interaction::ChartInteraction;
pub use plycore::ChartError;
pub use theme::get_theme;
pub use types::{ChartArea, ChartTheme, ChartViewport};

/// Core canvas chart component.
/// Renders OHLCV data to an HTML5 Canvas element via Canvas2D.
pub struct CanvasChart {
    canvas_id: String,
    theme: ChartTheme,
    chart_type: ChartType,
}

impl CanvasChart {
    /// Create a new chart bound to a canvas element.
    #[must_use]
    pub fn new(canvas_id: &str) -> Self {
        Self {
            canvas_id: canvas_id.to_string(),
            theme: ChartTheme::dark(),
            chart_type: ChartType::Candlestick,
        }
    }

    /// Set chart theme.
    pub fn with_theme(mut self, theme: ChartTheme) -> Self {
        self.theme = theme;
        self
    }

    /// Set chart type.
    pub fn with_chart_type(mut self, chart_type: ChartType) -> Self {
        self.chart_type = chart_type;
        self
    }

    /// Get current chart type.
    #[must_use]
    pub fn chart_type(&self) -> ChartType {
        self.chart_type
    }

    /// Update chart with OHLCV candle data.
    /// Dispatches to the appropriate renderer based on chart_type.
    pub fn update(&self, data: &[CandleData]) -> Result<(), ChartError> {
        match self.chart_type {
            ChartType::Candlestick | ChartType::Line | ChartType::Area | ChartType::Bar => {
                canvas::update_candles(&self.canvas_id, data, &self.theme)
            }
            ChartType::Heatmap => canvas::update_heatmap(&self.canvas_id, "", &self.theme),
            ChartType::OrderBook => canvas::update_order_book(&self.canvas_id, "", &self.theme),
            _ => canvas::update_candles(&self.canvas_id, data, &self.theme),
        }
    }

    /// Get canvas ID.
    #[must_use]
    pub fn canvas_id(&self) -> &str {
        &self.canvas_id
    }

    /// Get theme.
    #[must_use]
    pub fn theme(&self) -> &ChartTheme {
        &self.theme
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
        assert_eq!(chart.canvas_id(), "my-canvas");
    }

    #[test]
    fn canvas_chart_default_theme_is_dark() {
        let chart = CanvasChart::new("c");
        assert_eq!(chart.theme().bg, "#0a0a0a");
    }

    #[test]
    fn canvas_chart_with_theme() {
        let theme = ChartTheme::light();
        let chart = CanvasChart::new("c").with_theme(theme);
        assert_eq!(chart.theme().bg, "#ffffff");
    }

    #[test]
    fn canvas_chart_with_chart_type() {
        let chart = CanvasChart::new("c").with_chart_type(ChartType::Line);
        assert_eq!(chart.chart_type(), ChartType::Line);
    }

    #[test]
    fn canvas_chart_default_type_is_candlestick() {
        let chart = CanvasChart::new("c");
        assert_eq!(chart.chart_type(), ChartType::Candlestick);
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
        assert_eq!(ChartType::all().len(), 17);
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
        assert!(chart.update(&[]).is_ok());
    }

    #[test]
    fn canvas_update_single_candle() {
        let chart = CanvasChart::new("c");
        let data = vec![CandleData {
            time: 1.0,
            open: 100.0,
            high: 105.0,
            low: 99.0,
            close: 102.0,
            volume: 500.0,
        }];
        assert!(chart.update(&data).is_ok());
    }

    #[test]
    fn canvas_update_line_type() {
        let chart = CanvasChart::new("c").with_chart_type(ChartType::Line);
        let data = vec![CandleData {
            time: 1.0,
            open: 100.0,
            high: 105.0,
            low: 99.0,
            close: 102.0,
            volume: 500.0,
        }];
        assert!(chart.update(&data).is_ok());
    }

    #[test]
    fn canvas_update_area_type() {
        let chart = CanvasChart::new("c").with_chart_type(ChartType::Area);
        let data = vec![CandleData {
            time: 1.0,
            open: 100.0,
            high: 105.0,
            low: 99.0,
            close: 102.0,
            volume: 500.0,
        }];
        assert!(chart.update(&data).is_ok());
    }

    #[test]
    fn canvas_update_bar_type() {
        let chart = CanvasChart::new("c").with_chart_type(ChartType::Bar);
        let data = vec![CandleData {
            time: 1.0,
            open: 100.0,
            high: 105.0,
            low: 99.0,
            close: 102.0,
            volume: 500.0,
        }];
        assert!(chart.update(&data).is_ok());
    }

    #[test]
    fn device_pixel_ratio_native() {
        assert_eq!(canvas::device_pixel_ratio(), 1.0);
    }

    #[test]
    fn canvas_destroy_chart() {
        assert!(canvas::destroy_chart("any-id").is_ok());
    }

    #[test]
    fn candle_data_serialization_roundtrip() {
        let data = vec![
            CandleData {
                time: 1.0,
                open: 100.0,
                high: 105.0,
                low: 99.0,
                close: 102.0,
                volume: 500.0,
            },
            CandleData {
                time: 2.0,
                open: 102.0,
                high: 108.0,
                low: 101.0,
                close: 107.0,
                volume: 600.0,
            },
        ];
        let json = serde_json::to_string(&data).unwrap();
        let back: Vec<CandleData> = serde_json::from_str(&json).unwrap();
        assert_eq!(data.len(), back.len());
        assert_eq!(data[0].close, back[0].close);
    }
}
