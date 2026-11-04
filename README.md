# plychart

> High-performance WASM charting library for Rust; candlestick, line, area, scatter, heatmap, and more.

[![License](https://img.shields.io/badge/license-Apache--2.0-blue)](LICENSE-APACHE)

## Crates

| Crate | Purpose |
|-------|---------|
| `plychart` | Chart rendering (Canvas2D, WASM) |
| `plycompute` | Financial computation (Monte Carlo, stats, risk, portfolio) |
| `plycore` | Shared types (OHLCV, themes, viewport) |

## Quick Start

```rust
use plychart::{ChartConfig, ChartType};
```

## WASM / npm

The browser artifact is built with wasm-pack and committed at
`crates/plychart/pkg/` (ESM, `--target web`), so consumers can vendor it
directly without installing the Rust toolchain.

Build it locally:

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-pack --locked
bash scripts/build-wasm.sh
```

The script guards against a dead build (fewer than 20 exports), copies npm
metadata (`@wyattau/plychart`) and hand-written TypeScript types into `pkg/`,
stamps `pkg/version.txt` with a content hash of the WASM binary for cache
busting, and stages the same files into `packages/solid-chart/pkg/` for the
Solid wrapper.

| Package | Path | Purpose |
|---------|------|---------|
| `@wyattau/plychart` | `crates/plychart/pkg/` | Raw wasm-bindgen ESM module |
| `@wyattau/plychart-solid` | `packages/solid-chart/` | `<Chart>` Solid component |

Direct ESM usage (any bundler):

```js
import init, { create_chart, update_candles } from '@wyattau/plychart';

await init();                       // loads plychart_bg.wasm
create_chart('markets', 900, 420);  // canvas element id, size
update_candles('markets', candlesJson, themeJson);
```

Solid island (Astro / Vite with the Solid plugin — the wrapper ships TSX and
compiles in your bundler; `solid-js` is a peer dependency):

```tsx
import { Chart } from '@wyattau/plychart-solid';

<Chart id="markets" type="candlestick" data={candlesJson} width={900} height={420} theme={themeJson} />
```

`data` is a JSON string (for candles: `[{time, open, high, low, close,
volume}, ...]`); `theme` is an optional JSON theme string. Neither package is
published to a registry — consume via the committed `pkg/` or a git
dependency and run `bash scripts/build-wasm.sh` once. CI builds and verifies
the artifact on every push (`Wasm Pack` job).

## License

Apache-2.0
