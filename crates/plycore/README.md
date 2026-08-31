# plycore

Shared types for the plychart ecosystem. Provides the core data types used across charting and computation modules.

## Types

- `CandleData` — OHLCV candle data point (time, open, high, low, close, volume)
- `ChartArea` — Bounding rectangle for chart rendering
- `ChartViewport` — Zoom + pan state (start, count, log_scale)
- `ChartTheme` — Theme colors (dark, light, midnight)
- `ChartError` — Error type (CanvasNotFound, DataParseError, InvalidData, RenderError)
- `ChartOpts` — Render configuration (show_grid, show_crosshair, show_volume, show_axis_labels)
- `BarData`, `ScatterPoint`, `HeatmapCell`, `Point2D` — Additional data types

## Usage

```toml
[dependencies]
plycore = "0.1"
```

```rust
use plycore::{CandleData, ChartTheme, ChartViewport};

let candle = CandleData { time: 1.0, open: 100.0, high: 105.0, low: 99.0, close: 102.0, volume: 500.0 };
let theme = ChartTheme::dark();
let viewport = ChartViewport::default();
```

## License

Apache-2.0
