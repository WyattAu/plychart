# Changelog

## 0.1.0 (2026-08-31)

### Added
- 13 chart types: Candlestick, Line, Area, Bar, Heatmap, Scatter, Gauge, Radar, Treemap, Waterfall, OrderBook, Backtest, Correlation
- Drawing primitives: grid (price + time), crosshair + OHLC readout, volume bars
- `ChartInteraction` state machine: zoom, pan, drag, touch, pinch, keyboard
- `CanvasChart` struct with builder pattern and polymorphic dispatch
- 14 WASM entry points for JS interop
- DPR-aware Canvas2D rendering
- Dark, light, midnight themes
- 35 unit tests + 12 property-based tests
