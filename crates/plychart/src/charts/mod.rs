//! Chart types and registry.

use serde::{Deserialize, Serialize};

pub mod candlestick;
pub mod line;
pub mod area;
pub mod bar;
pub mod heatmap;
pub mod scatter;
pub mod gauge;
pub mod radar;
pub mod treemap;
pub mod waterfall;
pub mod order_book;
pub mod backtest;
pub mod correlation;

/// Re-export core types from plycore for convenience.
pub use plycore::{CandleData, BarData, ChartArea, ChartTheme, ChartViewport};

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
        }
    }

    #[must_use]
    pub const fn all() -> &'static [Self] {
        &[
            Self::Candlestick, Self::Line, Self::Area, Self::Bar,
            Self::Heatmap, Self::Scatter, Self::Gauge, Self::Radar,
            Self::Treemap, Self::Waterfall, Self::OrderBook,
            Self::Backtest, Self::Correlation,
        ]
    }
}
