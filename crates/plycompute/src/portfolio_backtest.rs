//! Multi-asset portfolio backtest with periodic rebalancing.
//!
//! Walks daily returns for N assets, re-deriving target weights at each
//! rebalance date from trailing volatility (no look-ahead), charging
//! turnover costs per rebalance.

use serde::{Deserialize, Serialize};

/// Weighting scheme for rebalance targets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeightMode {
    /// 1/N across all assets.
    Equal,
    /// Inverse trailing volatility (vol-weighted risk parity proxy).
    InvVol,
    /// Minimum-variance weights fitted from the trailing covariance matrix
    /// (long-only, simplex-projected). The true walk-forward mode: weights
    /// are *fitted* per rebalance from data, not fixed by rule.
    MinVar,
}

impl WeightMode {
    /// Parse from a JSON-facing string; unknown defaults to Equal.
    #[must_use]
    pub fn parse(s: &str) -> Self {
        match s {
            "inv_vol" | "inverse_vol" | "invvol" => Self::InvVol,
            "min_var" | "minvar" | "min_variance" => Self::MinVar,
            _ => Self::Equal,
        }
    }

    /// Canonical JSON-facing name (see [`WeightMode::parse`]).
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Equal => "equal",
            Self::InvVol => "inv_vol",
            Self::MinVar => "min_var",
        }
    }
}

/// Portfolio backtest output.
#[derive(Debug, Clone, Serialize)]
pub struct PortfolioBacktestResult {
    /// Weighting scheme used.
    pub mode: String,
    /// Portfolio equity curve normalized to 1.0 (len == n_periods).
    pub equity: Vec<f64>,
    /// Underwater curve, <= 0.
    pub drawdown: Vec<f64>,
    /// Individual assets' buy-and-hold, normalized to 1.0 (n_assets rows).
    pub asset_equity: Vec<Vec<f64>>,
    /// (period_index, [weights]) at each rebalance.
    pub rebalances: Vec<(usize, Vec<f64>)>,
    /// Annualized growth rate.
    pub cagr: f64,
    /// Annualized Sharpe of daily portfolio returns.
    pub sharpe: f64,
    /// Max drawdown (negative).
    pub max_drawdown: f64,
    /// Total turnover cost paid as a fraction of equity.
    pub total_cost: f64,
    /// Rebalances executed per year.
    pub rebalances_per_year: f64,
    /// Per-rebalance-period stats: one window per OOS segment.
    pub windows: Vec<PortfolioWindow>,
}

/// Stats for one rebalance period.
#[derive(Debug, Clone, Serialize)]
pub struct PortfolioWindow {
    /// First day index of the window.
    pub start: usize,
    /// One-past-last day index.
    pub end: usize,
    /// Simple return of the portfolio over the window (fraction).
    pub ret: f64,
    /// Weights held during the window (post-rebalance targets).
    pub weights: Vec<f64>,
}

const TRADING_DAYS: f64 = 252.0;
/// Trailing window for inverse-vol estimates.
const VOL_WINDOW: usize = 60;

fn trailing_vols(returns: &[Vec<f64>], upto: usize) -> Vec<f64> {
    returns
        .iter()
        .map(|r| {
            let start = upto.saturating_sub(VOL_WINDOW).max(1);
            let seg = &r[start..=upto.min(r.len() - 1)];
            let n = seg.len().max(1) as f64;
            let mean = seg.iter().sum::<f64>() / n;
            let var = seg.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n;
            var.sqrt().max(1e-9)
        })
        .collect()
}

/// Long-only minimum-variance weights from the trailing covariance matrix
/// over `returns[..=upto]` (VOL_WINDOW days, no look-ahead).
///
/// Solver: projected gradient descent on the simplex (min wᵀΣw s.t. Σw = 1,
/// w >= 0). Simplex projection via sort-based algorithm. Deterministic.
fn min_var_weights(returns: &[Vec<f64>], upto: usize, n_assets: usize) -> Vec<f64> {
    // Trailing covariance matrix Σ (n_assets x n_assets).
    let start = upto.saturating_sub(VOL_WINDOW).max(1);
    let end = upto.min(returns[0].len().saturating_sub(1));
    let n_obs = end.saturating_sub(start) + 1;
    if n_obs < 10 {
        return vec![1.0 / n_assets as f64; n_assets];
    }

    // Demeaned return matrix.
    let mut means = vec![0.0_f64; n_assets];
    for (a, r) in returns.iter().enumerate() {
        let seg = &r[start..=end];
        means[a] = seg.iter().sum::<f64>() / n_obs as f64;
    }
    let mut cov = vec![0.0_f64; n_assets * n_assets];
    for i in 0..n_assets {
        for j in i..n_assets {
            let mut c = 0.0_f64;
            for t in start..=end {
                c += (returns[i][t] - means[i]) * (returns[j][t] - means[j]);
            }
            let v = c / (n_obs - 1) as f64;
            cov[i * n_assets + j] = v;
            cov[j * n_assets + i] = v;
        }
    }

    // Projected gradient descent: w <- proj_simplex(w - lr * Σw).
    let max_eig = (0..n_assets)
        .map(|i| cov[i * n_assets + i])
        .fold(0.0_f64, f64::max)
        * (n_assets as f64)
        .max(1.0);
    let lr = 1.0 / max_eig.max(1e-12);
    let mut w = vec![1.0 / n_assets as f64; n_assets];
    for _ in 0..300 {
        // grad = Σw
        let mut grad = vec![0.0_f64; n_assets];
        for i in 0..n_assets {
            for j in 0..n_assets {
                grad[i] += cov[i * n_assets + j] * w[j];
            }
        }
        // Step + project onto simplex.
        let mut next: Vec<f64> = w
            .iter()
            .zip(&grad)
            .map(|(wi, gi)| (wi - lr * gi).max(0.0))
            .collect();
        project_simplex(&mut next);
        // Converged?
        let shift: f64 = w.iter().zip(&next).map(|(a, b)| (a - b).abs()).sum();
        w = next;
        if shift < 1e-10 {
            break;
        }
    }
    // Renormalize against float drift.
    let sum: f64 = w.iter().sum();
    if sum > 1e-12 {
        w.iter().map(|x| x / sum).collect()
    } else {
        vec![1.0 / n_assets as f64; n_assets]
    }
}

/// Project a vector onto the probability simplex (sum = 1, all >= 0).
/// Sort-based O(n log n) algorithm (Duchi et al. 2008).
fn project_simplex(v: &mut [f64]) {
    let n = v.len() as f64;
    let mut sorted: Vec<f64> = v.to_vec();
    sorted.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));
    let mut cumsum = 0.0_f64;
    let mut theta = 0.0_f64;
    for (i, &x) in sorted.iter().enumerate() {
        let t = (cumsum + x - 1.0) / (i as f64 + 1.0);
        if x - t > 0.0 {
            cumsum += x;
            theta = t;
        } else {
            break;
        }
    }
    for x in v.iter_mut() {
        *x = (*x - theta).max(0.0);
    }
}

/// Run the portfolio backtest.
///
/// * `closes` — flat row-major matrix, n_assets x n_periods close prices.
/// * `n_assets`, `n_periods` — matrix dims (n_periods >= 90).
/// * `mode_str` — `"equal"`, `"inv_vol"`, or `"min_var"`.
/// * `rebalance_every` — trading days between rebalances (>= 1).
/// * `cost_bps` — one-way cost in basis points applied to turnover.
///
/// # Errors
///
/// Returns a message when dims mismatch, inputs are non-finite, or too short.
#[allow(clippy::too_many_arguments)]
pub fn run(
    closes: &[f64],
    n_assets: usize,
    n_periods: usize,
    mode_str: &str,
    rebalance_every: usize,
    cost_bps: f64,
) -> Result<PortfolioBacktestResult, String> {
    if n_assets == 0 || n_assets > 20 {
        return Err(format!("n_assets must be in 1..=20, got {n_assets}"));
    }
    if n_periods < 90 {
        return Err(format!("need >= 90 periods, got {n_periods}"));
    }
    if closes.len() != n_assets * n_periods {
        return Err(format!(
            "closes.len() {} != n_assets * n_periods = {}",
            closes.len(),
            n_assets * n_periods
        ));
    }
    if closes.iter().any(|c| !c.is_finite() || *c <= 0.0) {
        return Err("closes must be finite and positive".into());
    }
    if rebalance_every == 0 {
        return Err("rebalance_every must be >= 1".into());
    }
    let mode = WeightMode::parse(mode_str);

    // Daily returns per asset (n_assets x (n_periods - 1)).
    let mut returns: Vec<Vec<f64>> = Vec::with_capacity(n_assets);
    for a in 0..n_assets {
        let row = &closes[a * n_periods..(a + 1) * n_periods];
        let mut r = Vec::with_capacity(n_periods - 1);
        for i in 1..n_periods {
            r.push(row[i] / row[i - 1] - 1.0);
        }
        returns.push(r);
    }

    let cost_rate = cost_bps / 10_000.0;
    let n_ret = n_periods - 1;

    let mut weights = vec![1.0 / n_assets as f64; n_assets];
    let mut equity = vec![1.0_f64];
    let mut rebalances: Vec<(usize, Vec<f64>)> = Vec::new();
    let mut windows: Vec<(usize, Vec<f64>)> = Vec::new();
    let mut total_cost = 0.0_f64;
    let mut rebalance_count = 0usize;

    // Normalize initial weights (equal or first trailing estimate after
    // VOL_WINDOW so inv-vol/min-var have data; before that, equal).
    if mode == WeightMode::InvVol && n_ret > VOL_WINDOW {
        let vols = trailing_vols(&returns, VOL_WINDOW);
        let inv: Vec<f64> = vols.iter().map(|v| 1.0 / v).collect();
        let sum: f64 = inv.iter().sum();
        weights = inv.iter().map(|x| x / sum).collect();
    } else if mode == WeightMode::MinVar && n_ret > VOL_WINDOW {
        weights = min_var_weights(&returns, VOL_WINDOW, n_assets);
    }
    rebalances.push((0, weights.clone()));

    for t in 0..n_ret {
        // Rebalance check (day 0 weights already set).
        if t > 0 && t % rebalance_every == 0 {
            let target = match mode {
                WeightMode::InvVol => {
                    let vols = trailing_vols(&returns, t);
                    let inv: Vec<f64> = vols.iter().map(|v| 1.0 / v).collect();
                    let sum: f64 = inv.iter().sum();
                    inv.iter().map(|x| x / sum).collect()
                }
                WeightMode::MinVar => min_var_weights(&returns, t, n_assets),
                WeightMode::Equal => vec![1.0 / n_assets as f64; n_assets],
            };
            let turnover: f64 = target
                .iter()
                .zip(&weights)
                .map(|(tn, wo)| (tn - wo).abs())
                .sum();
            let day_cost = turnover * cost_rate;
            total_cost += day_cost;
            if let Some(e) = equity.last_mut() {
                *e *= 1.0 - day_cost;
            }
            weights = target;
            rebalance_count += 1;
            rebalances.push((t, weights.clone()));
            windows.push((t, weights.clone()));
        }

        // Portfolio return for day t.
        let port_ret: f64 = weights.iter().zip(&returns).map(|(w, r)| w * r[t]).sum();
        let prev = *equity.last().ok_or("equity underflow")?;
        equity.push(prev * (1.0 + port_ret));
    }

    // Per-window returns from the completed equity curve. Window i spans
    // windows[i].start ..= windows[i].end - 1 in equity indices.
    let portfolio_windows: Vec<PortfolioWindow> = windows
        .iter()
        .enumerate()
        .map(|(i, (start, w))| {
            let end_idx = windows.get(i + 1).map_or(equity.len() - 1, |(next_start, _)| *next_start);
            let e0 = equity.get(*start).copied().unwrap_or(1.0);
            let e1 = equity.get(end_idx).copied().unwrap_or(e0);
            PortfolioWindow {
                start: *start,
                end: end_idx,
                ret: if e0.abs() > 1e-12 { e1 / e0 - 1.0 } else { 0.0 },
                weights: w.clone(),
            }
        })
        .collect();

    // Asset buy-and-hold curves.
    let mut asset_equity: Vec<Vec<f64>> = Vec::with_capacity(n_assets);
    for a in 0..n_assets {
        let row = &closes[a * n_periods..(a + 1) * n_periods];
        let c0 = row[0];
        asset_equity.push(row.iter().map(|c| c / c0).collect());
    }

    // Stats.
    let rets: Vec<f64> = equity.windows(2).map(|w| w[1] / w[0] - 1.0).collect();
    let mean = rets.iter().sum::<f64>() / rets.len() as f64;
    let var = rets.iter().map(|r| (r - mean).powi(2)).sum::<f64>() / rets.len() as f64;
    let std = var.sqrt();
    let years = rets.len() as f64 / TRADING_DAYS;
    let final_eq = *equity.last().ok_or("equity underflow")?;
    let cagr = if final_eq > 0.0 && years > 0.0 {
        final_eq.powf(1.0 / years) - 1.0
    } else {
        -1.0
    };
    let sharpe = if std > 1e-12 {
        mean / std * TRADING_DAYS.sqrt()
    } else {
        0.0
    };

    let mut peak = 1.0_f64;
    let mut max_dd = 0.0_f64;
    let drawdown: Vec<f64> = equity
        .iter()
        .map(|e| {
            if *e > peak {
                peak = *e;
            }
            let dd = e / peak - 1.0;
            if dd < max_dd {
                max_dd = dd;
            }
            dd
        })
        .collect();

    Ok(PortfolioBacktestResult {
        mode: mode.as_str().to_string(),
        equity,
        drawdown,
        asset_equity,
        rebalances,
        cagr,
        sharpe,
        max_drawdown: max_dd,
        total_cost,
        rebalances_per_year: rebalance_count as f64 / years.max(1e-9),
        windows: portfolio_windows,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// n_assets synthetic price series with drift + noise.
    fn synthetic(n_assets: usize, n_periods: usize) -> Vec<f64> {
        let mut out = Vec::with_capacity(n_assets * n_periods);
        for a in 0..n_assets {
            let drift = 0.0002 + a as f64 * 0.0001;
            let mut price = 100.0;
            let mut seed = (a as u64 + 1) * 7919;
            for _ in 0..n_periods {
                seed = seed
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                let noise = ((seed >> 33) as f64 / (1u64 << 31) as f64 - 0.5) * 0.02;
                price *= 1.0 + drift + noise;
                out.push(price);
            }
        }
        out
    }

    #[test]
    fn equal_weight_shapes() {
        let closes = synthetic(3, 300);
        let r = run(&closes, 3, 300, "equal", 20, 2.0).unwrap();
        assert_eq!(r.equity.len(), 300);
        assert_eq!(r.drawdown.len(), 300);
        assert_eq!(r.asset_equity.len(), 3);
        assert_eq!(r.asset_equity[0].len(), 300);
        assert_eq!(r.mode, "equal");
        assert!(!r.rebalances.is_empty());
        assert!(r.equity.iter().all(|e| e.is_finite() && *e > 0.0));
    }

    #[test]
    fn inv_vol_runs_and_sets_mode() {
        let closes = synthetic(3, 300);
        let r = run(&closes, 3, 300, "inv_vol", 20, 2.0).unwrap();
        assert_eq!(r.mode, "inv_vol");
        assert!(r.rebalances.len() > 1);
        // Weights sum to ~1 at every rebalance.
        for (_, w) in &r.rebalances {
            let sum: f64 = w.iter().sum();
            assert!((sum - 1.0).abs() < 1e-9);
        }
    }

    #[test]
    fn costs_reduce_equity() {
        let closes = synthetic(3, 300);
        let free = run(&closes, 3, 300, "inv_vol", 10, 0.0).unwrap();
        let costly = run(&closes, 3, 300, "inv_vol", 10, 10.0).unwrap();
        let f = *free.equity.last().unwrap();
        let c = *costly.equity.last().unwrap();
        assert!(c <= f + 1e-12, "costs must not improve equity");
        assert!(costly.total_cost > 0.0);
    }

    #[test]
    fn dim_mismatch_rejected() {
        let closes = synthetic(3, 300);
        assert!(run(&closes, 3, 299, "equal", 20, 1.0).is_err());
        assert!(run(&closes, 2, 300, "equal", 20, 1.0).is_err());
        let short = synthetic(2, 50);
        assert!(run(&short, 2, 50, "equal", 20, 1.0).is_err());
    }

    #[test]
    fn weights_sum_to_one_initial() {
        let closes = synthetic(4, 200);
        let r = run(&closes, 4, 200, "inv_vol", 60, 1.0).unwrap();
        let (_, w) = &r.rebalances[0];
        assert!((w.iter().sum::<f64>() - 1.0).abs() < 1e-9);
    }

    #[test]
    fn min_var_runs_and_sums() {
        let closes = synthetic(4, 300);
        let r = run(&closes, 4, 300, "min_var", 20, 2.0).unwrap();
        assert_eq!(r.mode, "min_var");
        assert!(r.rebalances.len() > 1);
        for (_, w) in &r.rebalances {
            let sum: f64 = w.iter().sum();
            assert!((sum - 1.0).abs() < 1e-6, "weights must sum to 1, got {sum}");
            assert!(w.iter().all(|x| *x >= 0.0), "long-only violated");
        }
        assert!(r.equity.iter().all(|e| e.is_finite() && *e > 0.0));
    }

    #[test]
    fn min_var_parse() {
        assert_eq!(WeightMode::parse("min_var"), WeightMode::MinVar);
        assert_eq!(WeightMode::parse("minvar"), WeightMode::MinVar);
        assert_eq!(WeightMode::parse("x"), WeightMode::Equal);
    }

    #[test]
    fn project_simplex_basics() {
        let mut v = vec![2.0, -1.0, 0.5];
        project_simplex(&mut v);
        let sum: f64 = v.iter().sum();
        assert!((sum - 1.0).abs() < 1e-9);
        assert!(v.iter().all(|x| *x >= 0.0));
    }

    #[test]
    fn serde_roundtrip() {
        let closes = synthetic(3, 250);
        let r = run(&closes, 3, 250, "equal", 20, 1.0).unwrap();
        let json = serde_json::to_string(&r).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(parsed["equity"].is_array());
        assert_eq!(parsed["mode"], "equal");
    }
}
