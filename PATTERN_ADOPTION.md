# PATTERN_ADOPTION

Rust WASM charting library: plychart (render), plycompute (financial
computation), plycore (shared types), plus a Solid wrapper package.

| Pattern | Status | Evidence |
|---|---|---|
| P64 | adopted | crates/plychart/tests/geometry_properties.rs:9 (proptest suite: price_to_y linear and log mappings never return NaN and stay monotonic) |
| P13 | adopted | crates/plycompute/src/portfolio_backtest.rs:4 (target weights re-derived from trailing volatility only, no look-ahead) |
| T27 | n/a | verified absent: chart data is a repaint-in-place candle series (crates/plychart/src/canvas.rs:143 clears and redraws); no ledger or balance domain |

plycompute's xorshift128+ generator is seedable for reproducible runs
(crates/plycompute/src/rng.rs:6) but no canon ID covers seeded PRNG
reproducibility on its own, so no row is claimed for it.
