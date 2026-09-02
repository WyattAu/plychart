//! Shared types for plychart and plycompute.
//!
//! Provides the core data types used across all charting and computation modules.

use serde::{Deserialize, Serialize};

/// OHLCV candle data point — the universal time-series data type.
/// Used by candlestick, line, area, bar, and all time-series charts.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct CandleData {
    /// Unix timestamp in seconds.
    pub time: f64,
    /// Open price.
    pub open: f64,
    /// High price.
    pub high: f64,
    /// Low price.
    pub low: f64,
    /// Close price.
    pub close: f64,
    /// Volume.
    pub volume: f64,
}

/// 2D point for scatter plots, line charts, etc.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

/// Bounding rectangle for chart rendering.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct ChartArea {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

/// Bar data for bar charts.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BarData {
    pub label: String,
    pub value: f64,
    pub color: Option<String>,
}

/// Scatter point with size and color.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ScatterPoint {
    pub x: f64,
    pub y: f64,
    pub size: f64,
    pub color: Option<String>,
}

/// Heatmap cell.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct HeatmapCell {
    pub row: usize,
    pub col: usize,
    pub value: f64,
}

/// Chart viewport state (zoom + pan).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChartViewport {
    /// Index of the first visible data point.
    pub start: usize,
    /// Number of visible data points.
    pub count: usize,
    /// Use logarithmic scale for Y-axis.
    pub log_scale: bool,
}

impl Default for ChartViewport {
    fn default() -> Self {
        Self {
            start: 0,
            count: 100,
            log_scale: false,
        }
    }
}

/// Theme colors for chart rendering.
#[derive(Debug, Clone, Copy)]
pub struct ChartTheme {
    pub bg: &'static str,
    pub text: &'static str,
    pub text_muted: &'static str,
    pub grid: &'static str,
    pub up: &'static str,
    pub down: &'static str,
    pub accent: &'static str,
    pub volume: &'static str,
    pub crosshair: &'static str,
}

impl ChartTheme {
    pub const fn dark() -> Self {
        Self {
            bg: "#0a0a0a",
            text: "#e0e0e0",
            text_muted: "#666666",
            grid: "#1a1a1a",
            up: "#4ade80",
            down: "#f87171",
            accent: "#c8a23c",
            volume: "#333333",
            crosshair: "#555555",
        }
    }
    pub const fn light() -> Self {
        Self {
            bg: "#ffffff",
            text: "#1a1a1a",
            text_muted: "#999999",
            grid: "#e0e0e0",
            up: "#22c55e",
            down: "#ef4444",
            accent: "#c8a23c",
            volume: "#cccccc",
            crosshair: "#999999",
        }
    }
    pub const fn midnight() -> Self {
        Self {
            bg: "#0c0c0c",
            text: "#ffffff",
            text_muted: "#666666",
            grid: "#1c1c1c",
            up: "#4ade80",
            down: "#ff4081",
            accent: "#00e5ff",
            volume: "#2a2a2a",
            crosshair: "#ffffff",
        }
    }
}

impl Default for ChartTheme {
    fn default() -> Self {
        Self::dark()
    }
}

/// Chart error type — never panics in production.
#[derive(Debug, Clone, PartialEq)]
pub enum ChartError {
    CanvasNotFound(String),
    DataParseError(String),
    InvalidData(String),
    RenderError(String),
}

impl std::fmt::Display for ChartError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CanvasNotFound(id) => write!(f, "Canvas not found: {}", id),
            Self::DataParseError(msg) => write!(f, "Data parse error: {}", msg),
            Self::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
            Self::RenderError(msg) => write!(f, "Render error: {}", msg),
        }
    }
}

impl std::error::Error for ChartError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn candle_data_default() {
        let c = CandleData::default();
        assert_eq!(c.time, 0.0);
        assert_eq!(c.open, 0.0);
        assert_eq!(c.volume, 0.0);
    }

    #[test]
    fn candle_data_serde_roundtrip() {
        let c = CandleData {
            time: 1.0,
            open: 100.0,
            high: 105.0,
            low: 99.0,
            close: 102.0,
            volume: 500.0,
        };
        let json = serde_json::to_string(&c).expect("serialize");
        let back: CandleData = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(c.time, back.time);
        assert_eq!(c.close, back.close);
    }

    #[test]
    fn chart_area_default() {
        let a = ChartArea::default();
        assert_eq!(a.x, 0.0);
        assert_eq!(a.w, 0.0);
    }

    #[test]
    fn chart_viewport_default() {
        let v = ChartViewport::default();
        assert_eq!(v.start, 0);
        assert_eq!(v.count, 100);
        assert!(!v.log_scale);
    }

    #[test]
    fn theme_dark_up_is_green() {
        let t = ChartTheme::dark();
        assert_eq!(t.up, "#4ade80");
        assert_eq!(t.down, "#f87171");
    }

    #[test]
    fn theme_light_up_is_green() {
        let t = ChartTheme::light();
        assert_eq!(t.up, "#22c55e");
        assert_eq!(t.down, "#ef4444");
    }

    #[test]
    fn theme_midnight_accent() {
        let t = ChartTheme::midnight();
        assert_eq!(t.accent, "#00e5ff");
    }

    #[test]
    fn theme_default_is_dark() {
        let t = ChartTheme::default();
        assert_eq!(t.bg, ChartTheme::dark().bg);
    }

    #[test]
    fn chart_error_display() {
        let e = ChartError::CanvasNotFound("test".into());
        assert_eq!(format!("{e}"), "Canvas not found: test");
        let e = ChartError::DataParseError("bad json".into());
        assert_eq!(format!("{e}"), "Data parse error: bad json");
        let e = ChartError::InvalidData("nan".into());
        assert_eq!(format!("{e}"), "Invalid data: nan");
        let e = ChartError::RenderError("ctx lost".into());
        assert_eq!(format!("{e}"), "Render error: ctx lost");
    }

    #[test]
    fn chart_error_is_std_error() {
        let e = ChartError::CanvasNotFound("x".into());
        let _: &dyn std::error::Error = &e;
    }

    #[test]
    fn bar_data_serde_roundtrip() {
        let b = BarData {
            label: "AAPL".into(),
            value: 150.0,
            color: Some("#ff0000".into()),
        };
        let json = serde_json::to_string(&b).expect("serialize");
        let back: BarData = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(b.label, back.label);
        assert_eq!(b.value, back.value);
    }

    #[test]
    fn scatter_point_serde_roundtrip() {
        let p = ScatterPoint {
            x: 1.0,
            y: 2.0,
            size: 5.0,
            color: None,
        };
        let json = serde_json::to_string(&p).expect("serialize");
        let back: ScatterPoint = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(p.x, back.x);
        assert_eq!(p.size, back.size);
    }

    #[test]
    fn point2d_serde_roundtrip() {
        let p = Point2D { x: 3.14, y: 2.71 };
        let json = serde_json::to_string(&p).expect("serialize");
        let back: Point2D = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(p.x, back.x);
        assert_eq!(p.y, back.y);
    }

    #[test]
    fn heatmap_cell_default() {
        let h = HeatmapCell::default();
        assert_eq!(h.row, 0);
        assert_eq!(h.col, 0);
        assert_eq!(h.value, 0.0);
    }
}
