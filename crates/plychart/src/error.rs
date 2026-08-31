//! Chart error types — exhaustive, documented, never panics.

use std::fmt;

/// Chart error type.
#[derive(Debug, Clone)]
pub enum ChartError {
    /// Canvas element not found by ID.
    CanvasNotFound(String),
    /// Failed to parse input data.
    DataParseError(String),
    /// Invalid data values.
    InvalidData(String),
    /// Rendering operation failed.
    RenderError(String),
}

impl fmt::Display for ChartError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CanvasNotFound(id) => write!(f, "Canvas not found: {id}"),
            Self::DataParseError(msg) => write!(f, "Data parse error: {msg}"),
            Self::InvalidData(msg) => write!(f, "Invalid data: {msg}"),
            Self::RenderError(msg) => write!(f, "Render error: {msg}"),
        }
    }
}

impl std::error::Error for ChartError {}
