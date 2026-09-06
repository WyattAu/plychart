# Changelog

All notable changes to this project will be documented in this file.

## [0.2.0] — 2026-09-05

### plychart

- **New chart exports (26 total):** `update_histogram_log` (log-scale bars),
  `update_heatmap_div` (diverging red/accent around a center value),
  `update_gauge_zoned` (threshold tick marks with labels),
  `update_stacked_bar`, `update_pie`, `update_sparkline`,
  `update_scatter_multi`, `update_radar_multi`, `get_tooltip_data`,
  `get_click_data`
- **Rendering fixes:** DPR transform no longer accumulates across updates
  (charts overlapped/ghosted on data change); proper retina backing store
- **Quality:** multiline Y-range helper deduplicated; pie legend wraps;
  theme threading through all render paths; DOM lookup helper extracted
- **Fix:** `mod wasm` was never declared — all `#[wasm_bindgen]` exports were
  silently absent from the binary until declared

### plycompute

- **New engines:** `backtest.rs` — walk-forward strategy backtester
  (SMA-cross, momentum, mean-reversion) with slippage/commission and no
  look-ahead; `portfolio_backtest.rs` — multi-asset rebalancing portfolio
  (equal-weight / inverse-vol) with turnover costs; `svi.rs` — Quasi-explicit
  SVI volatility-surface fitting
- **New exports:** `quant_backtest`, `quant_portfolio_backtest`,
  `quant_svi_surface`
- **simd_utils** moved in from hydrated_personal_site (SIMD128 dot/add/scale)
- **Collision fix:** export bodies extracted to macro-free `json.rs`;
  `wasm.rs` wrappers gated behind opt-in `wasm-exports` feature so downstream
  cdylibs can wrap the same logic without describe-symbol collisions

### Infrastructure

- wasm-bindgen unpinned (0.2.127 externref emits correct exports once `mod
  wasm` is declared); js-sys/web-sys unpinned
- `scripts/build-wasm.sh`: export-count guard (fails under 20 exports) +
  npm `package.json`/`types.d.ts` stamping + `version.txt` content hash
- `examples/demo`: 22 chart cards covering every export
- 280 tests passing across the workspace

## [0.1.0] — 2026-08-31

- Initial release: 13 chart types, 15 exports, Canvas2D via web-sys,
  interaction state machine (zoom/pan/pinch/keyboard), DPR-aware, 145 tests
