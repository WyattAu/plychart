# plycompute

Quantitative computation library for Rust/WASM. 21 modules covering risk, portfolio optimization, derivatives pricing, and signal processing.

## Modules

| Module | Description |
|--------|-------------|
| `montecarlo` | GBM simulation, percentile bands, log returns |
| `blackscholes` | Call/put pricing, Greeks, implied volatility |
| `risk` | VaR, Expected Shortfall, Sharpe, Sortino, histogram |
| `volatility` | EWMA, realized volatility, GARCH(1,1) |
| `portfolio` | Efficient frontier, random portfolios, tangency portfolio |
| `stats` | Correlation, covariance, matrix inverse, linear regression |
| `drawdown` | Drawdown analysis, Calmar ratio, ulcer index, pain index |
| `concentration` | HHI, Gini, entropy, effective N |
| `yieldcurve` | Nelson-Siegel fit, recession probability |
| `factor` | OLS regression, rolling regression |
| `cointegration` | Engle-Granger test, half-life |
| `liquidity` | Amihud, Kyle's lambda, CS spread |
| `realizedvol` | Bipower variation, jump detection |
| `copula` | Tail dependence, Kendall's tau |
| `regime` | 2-state HMM, Viterbi decoding |
| `pairs` | Pairs trading signal generation |
| `stress` | Historical stress test scenarios |
| `risk_decomp` | Component VaR, Kelly criterion |
| `hrp` | Hierarchical Risk Parity |
| `overlap` | Holdings overlap (Jaccard, weighted) |
| `rng` | Deterministic PRNG (xorshift128+) |

## Usage

```toml
[dependencies]
plycompute = "0.1"
```

```rust
use plycompute::{montecarlo, risk, volatility};

let closes = vec![100.0, 101.0, 99.0, 102.0, 103.0];
let result = montecarlo::montecarlo_from_prices(&closes, 252, 30, 1000);
println!("Drift: {:.2}%, Vol: {:.2}%", result.drift * 100.0, result.volatility * 100.0);

let returns = montecarlo::log_returns(&closes);
let var = risk::var_historical(&returns, 0.05);
```

## WASM

```rust
use plycompute::wasm;

let json = wasm::quant_montecarlo(&closes, 252, 30, 1000);
// Returns: { drift, volatility, s0, p5, p25, p50, p75, p95 }
```

## License

Apache-2.0
