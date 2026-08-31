# Changelog

## 0.1.0 (2026-08-31)

### Added
- Initial release
- `CandleData` — OHLCV candle data point (time, open, high, low, close, volume)
- `ChartArea` — Bounding rectangle for chart rendering
- `ChartViewport` — Zoom + pan state (start, count, log_scale)
- `ChartTheme` — Theme colors (dark, light, midnight)
- `ChartError` — Error type with Display + std::error::Error
- `ChartOpts` — Render configuration
- `BarData`, `ScatterPoint`, `HeatmapCell`, `Point2D` — Additional data types
- Serde support for all data types
- 15 unit tests
