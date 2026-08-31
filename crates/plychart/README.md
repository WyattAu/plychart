# plychart

Full-featured graphing library for Rust/WASM. Zero dependencies, zero watermarks, zero JS.

## Features

- **13 chart types**: Candlestick, Line, Area, Bar, Heatmap, Scatter, Gauge, Radar, Treemap, Waterfall, OrderBook, Backtest, Correlation
- **Canvas2D rendering**: Direct Canvas2D via web-sys, no JS dependencies
- **DPR-aware**: HiDPI/Retina support via devicePixelRatio
- **Interaction state machine**: Zoom, pan, crosshair, touch events, pinch-to-zoom
- **Grid + axis labels**: Price grid, time grid with HH:MM labels
- **Volume sub-pane**: Direction-colored volume bars with alpha
- **Crosshair + OHLC readout**: Dashed crosshair with nearest candle data
- **Theme system**: Dark, light, midnight themes with const constructors
- **WASM entry points**: 14 `#[wasm_bindgen]` exports for JS interop

## Usage

```toml
[dependencies]
plychart = "0.1"
```

### Rust (Leptos, Dioxus, etc.)

```rust
use plychart::{CanvasChart, ChartType, ChartTheme, ChartInteraction};

let chart = CanvasChart::new("my-canvas")
    .with_theme(ChartTheme::dark())
    .with_chart_type(ChartType::Candlestick);

chart.update(&candles)?;
```

### WASM (JavaScript)

```javascript
import init, { create_chart, update_candles } from 'plychart';

await init();
create_chart("my-canvas", 800, 400);
update_candles("my-canvas", JSON.stringify(candles));
```

## Chart Types

| Type | Description |
|------|-------------|
| Candlestick | OHLCV candle rendering with wick + body |
| Line | Polylime close prices |
| Area | Filled area with gradient |
| Bar | OHLC bars with ticks |
| Heatmap | Color-coded matrix |
| Scatter | 2D point cloud |
| Gauge | Semicircle dial |
| Radar | Spider/radar chart |
| Treemap | Squarified nested rectangles |
| Waterfall | Incremental bar chart |
| OrderBook | Bid/ask depth visualization |
| Backtest | Equity curve + drawdown split pane |
| Correlation | NxN correlation matrix heatmap |

## Architecture

```text
plycore (shared types)
  └── plychart (charting library)
        ├── canvas.rs — Canvas2D lifecycle
        ├── charts/ — 13 chart renderers + grid, crosshair, volume
        ├── interaction.rs — Zoom, pan, crosshair, touch state machine
        └── wasm.rs — 14 #[wasm_bindgen] exports
```

## License

Apache-2.0
