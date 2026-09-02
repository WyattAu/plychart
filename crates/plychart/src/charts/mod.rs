//! Chart types and registry.

use serde::{Deserialize, Serialize};

pub mod area;
pub mod backtest;
pub mod bar;
pub mod candlestick;
pub mod correlation;
pub mod crosshair;
pub mod gauge;
pub mod grid;
pub mod heatmap;
pub mod histogram;
pub mod line;
pub mod multiline;
pub mod order_book;
pub mod pie;
pub mod radar;
pub mod scatter;
pub mod sparkline;
pub mod treemap;
pub mod volume;
pub mod waterfall;

/// Re-export core types from plycore for convenience.
pub use plycore::{BarData, CandleData, ChartArea, ChartTheme, ChartViewport};

/// Supported chart types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChartType {
    Candlestick,
    Line,
    Area,
    Bar,
    Heatmap,
    Scatter,
    Gauge,
    Radar,
    Treemap,
    Waterfall,
    OrderBook,
    Backtest,
    Correlation,
    Pie,
    Histogram,
    Sparkline,
}

impl ChartType {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Candlestick => "Candles",
            Self::Line => "Line",
            Self::Area => "Area",
            Self::Bar => "Bars",
            Self::Heatmap => "Heatmap",
            Self::Scatter => "Scatter",
            Self::Gauge => "Gauge",
            Self::Radar => "Radar",
            Self::Treemap => "Treemap",
            Self::Waterfall => "Waterfall",
            Self::OrderBook => "Order Book",
            Self::Backtest => "Backtest",
            Self::Correlation => "Correlation",
            Self::Pie => "Pie",
            Self::Histogram => "Histogram",
            Self::Sparkline => "Sparkline",
        }
    }

    #[must_use]
    pub const fn all() -> &'static [Self] {
        &[
            Self::Candlestick,
            Self::Line,
            Self::Area,
            Self::Bar,
            Self::Heatmap,
            Self::Scatter,
            Self::Gauge,
            Self::Radar,
            Self::Treemap,
            Self::Waterfall,
            Self::OrderBook,
            Self::Backtest,
            Self::Correlation,
            Self::Pie,
            Self::Histogram,
            Self::Sparkline,
        ]
    }

    #[must_use]
    pub fn from_label(label: &str) -> Option<Self> {
        match label {
            "Candles" => Some(Self::Candlestick),
            "Line" => Some(Self::Line),
            "Area" => Some(Self::Area),
            "Bars" => Some(Self::Bar),
            "Heatmap" => Some(Self::Heatmap),
            "Scatter" => Some(Self::Scatter),
            "Gauge" => Some(Self::Gauge),
            "Radar" => Some(Self::Radar),
            "Treemap" => Some(Self::Treemap),
            "Waterfall" => Some(Self::Waterfall),
            "Order Book" => Some(Self::OrderBook),
            "Backtest" => Some(Self::Backtest),
            "Correlation" => Some(Self::Correlation),
            "Pie" => Some(Self::Pie),
            "Histogram" => Some(Self::Histogram),
            "Sparkline" => Some(Self::Sparkline),
            _ => None,
        }
    }
}
