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
