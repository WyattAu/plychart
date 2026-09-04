//! Walk-forward backtester with SMA-crossover strategy, slippage, and
//! commission modeling.
//!
//! The engine re-optimizes strategy parameters on a rolling in-sample
//! window and trades them forward on the out-of-sample window, so the
//! equity curve never uses future data. Costs are charged per position
//! flip (slippage + commission, in basis points).

use serde::{Deserialize, Serialize};

/// One walk-forward window's chosen parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowParam {
    /// First bar index of the out-of-sample segment.
    pub start: usize,
    /// One-past-last bar index of the out-of-sample segment.
    pub end: usize,
    /// Chosen fast SMA length.
    pub fast: usize,
    /// Chosen slow SMA length.
    pub slow: usize,
    /// In-sample (fit) Sharpe for the chosen pair.
    pub is_sharpe: f64,
}

/// Full walk-forward backtest output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacktestResult {
    /// Strategy equity curve, normalized to 1.0 at start (len == n).
    pub equity: Vec<f64>,
    /// Buy-and-hold equity curve (close / close[0], len == n).
    pub buyhold: Vec<f64>,
    /// Strategy underwater curve, <= 0 (len == n).
    pub drawdown: Vec<f64>,
    /// Per-window chosen parameters.
    pub windows: Vec<WindowParam>,
    /// Annualized compound growth rate of the strategy.
    pub cagr: f64,
    /// Annualized Sharpe ratio of daily strategy returns.
    pub sharpe: f64,
    /// Annualized Sortino ratio of daily strategy returns.
    pub sortino: f64,
    /// Max drawdown of the strategy (negative).
    pub max_drawdown: f64,
    /// Fraction of executed round-trips that were profitable.
    pub win_rate: f64,
    /// Position flips per year.
    pub trades_per_year: f64,
    /// Buy-and-hold CAGR for comparison.
    pub bh_cagr: f64,
    /// Buy-and-hold max drawdown (negative).
    pub bh_max_drawdown: f64,
}

const TRADING_DAYS: f64 = 252.0;

fn sma(closes: &[f64], len: usize, i: usize) -> Option<f64> {
    if len == 0 || i + 1 < len {
        return None;
    }
    Some(closes[i + 1 - len..=i].iter().sum::<f64>() / len as f64)
}

/// Net Sharpe of the long/flat SMA strategy on `closes[start..end]` using
/// the given SMA pair, charging `cost_bps` per flip. Returns (sharpe, flips).
fn eval_pair(
    closes: &[f64],
    start: usize,
    end: usize,
    fast: usize,
    slow: usize,
    cost: f64,
) -> (f64, usize) {
    let mut rets: Vec<f64> = Vec::with_capacity(end - start);
    let mut position = 0.0_f64;
    let mut flips = 0usize;
    for i in (start + 1)..end {
        let target = match (sma(closes, fast, i), sma(closes, slow, i)) {
            (Some(f), Some(s)) => {
                if f > s {
                    1.0
                } else {
                    0.0
                }
            }
            _ => 0.0,
        };
        if target != position {
            position = target;
            flips += 1;
            rets.push(-cost);
        }
        let r = closes[i] / closes[i - 1] - 1.0;
        rets.push(position * r);
    }
    if rets.is_empty() {
        return (f64::NEG_INFINITY, 0);
    }
    let mean = rets.iter().sum::<f64>() / rets.len() as f64;
    let var = rets.iter().map(|r| (r - mean).powi(2)).sum::<f64>() / rets.len() as f64;
    let std = var.sqrt();
    if std < 1e-12 {
        return (0.0, flips);
    }
    (mean / std * TRADING_DAYS.sqrt(), flips)
}

/// Run a walk-forward SMA-crossover backtest.
///
/// * `closes` — daily close prices.
/// * `fasts` / `slows` — candidate SMA lengths (fast < slow enforced per pair).
/// * `is_window` — in-sample fit length (bars).
/// * `oos_window` — out-of-sample trade length (bars).
/// * `slippage_bps`, `commission_bps` — per-flip costs in basis points.
///
/// # Errors
///
/// Returns `InvalidData` when inputs are too short or malformed.
pub fn walk_forward(
    closes: &[f64],
    fasts: &[usize],
    slows: &[usize],
    is_window: usize,
    oos_window: usize,
    slippage_bps: f64,
    commission_bps: f64,
) -> Result<BacktestResult, String> {
    let n = closes.len();
    if n < 60 {
        return Err(format!("need >= 60 closes, got {n}"));
    }
    if fasts.is_empty() || slows.is_empty() {
        return Err("fast/slow grids must be non-empty".into());
    }
    if is_window < 30 || oos_window < 5 {
        return Err("is_window >= 30 and oos_window >= 5 required".into());
    }
    if n < is_window + oos_window + 2 {
        return Err(format!(
            "series too short: need >= is_window + oos_window + 2 = {}",
            is_window + oos_window + 2
        ));
    }
    if closes.iter().any(|c| !c.is_finite() || *c <= 0.0) {
        return Err("closes must be finite and positive".into());
    }

    let cost = (slippage_bps + commission_bps) / 10_000.0;

    // Pairs with fast < slow, both fitting the in-sample window.
    let mut pairs: Vec<(usize, usize)> = Vec::new();
    for &f in fasts {
        for &s in slows {
            if f < s && s <= is_window {
                pairs.push((f, s));
            }
        }
    }
    if pairs.is_empty() {
        return Err("no valid (fast<slow<=is_window) pairs in grid".into());
    }

    // Flat (no position) during the first in-sample fit window.
    let mut equity = vec![1.0_f64; is_window];
    let mut position = 0.0_f64;
    let mut windows: Vec<WindowParam> = Vec::new();
    let mut trade_pnls: Vec<f64> = Vec::new();
    let mut entry_equity = 1.0_f64;
    let mut flips = 0usize;

    let mut t = is_window;
    while t < n {
        // 1. Fit: best pair by net IS Sharpe (tie-break: fewer flips).
        let mut best: Option<(usize, usize, f64, usize)> = None;
        for &(f, s) in &pairs {
            let (sharpe, iflips) = eval_pair(closes, t - is_window, t, f, s, cost);
            let better = match best {
                None => true,
                Some((_, _, bs, bf)) => sharpe > bs || (sharpe == bs && iflips < bf),
            };
            if better {
                best = Some((f, s, sharpe, iflips));
            }
        }
        let (fast, slow, is_sharpe, _) = best.ok_or("pair selection failed")?;
        let trade_end = (t + oos_window).min(n);
        windows.push(WindowParam {
            start: t,
            end: trade_end,
            fast,
            slow,
            is_sharpe,
        });

        // 2. Trade the OOS segment with the chosen pair.
        for i in t..trade_end {
            let target = match (sma(closes, fast, i), sma(closes, slow, i)) {
                (Some(fv), Some(sv)) => {
                    if fv > sv {
                        1.0
                    } else {
                        0.0
                    }
                }
                _ => 0.0,
            };
            let prev = *equity.last().ok_or("equity underflow")?;
            let mut day_equity = prev;
            if target != position {
                position = target;
                flips += 1;
                day_equity *= 1.0 - cost;
                // Close out the previous round-trip P&L (if any).
                if flips > 1 {
                    trade_pnls.push(day_equity / entry_equity - 1.0);
                }
                entry_equity = day_equity;
            }
            let r = closes[i] / closes[i - 1] - 1.0;
            day_equity *= 1.0 + position * r;
            equity.push(day_equity);
        }
        t = trade_end;
    }
    // Final open round-trip.
    if flips > 0 && equity.len() >= 2 {
        let last = *equity.last().ok_or("equity underflow")?;
        trade_pnls.push(last / entry_equity - 1.0);
    }

    // Buy & hold.
    let c0 = closes[0];
    let buyhold: Vec<f64> = closes.iter().map(|c| c / c0).collect();

    // Strategy daily returns from the equity curve.
    let rets: Vec<f64> = equity.windows(2).map(|w| w[1] / w[0] - 1.0).collect();
    let mean = rets.iter().sum::<f64>() / rets.len() as f64;
    let var = rets.iter().map(|r| (r - mean).powi(2)).sum::<f64>() / rets.len() as f64;
    let std = var.sqrt();
    let downside = rets
        .iter()
        .filter(|r| **r < 0.0)
        .copied()
        .collect::<Vec<_>>();
    let dvar = if downside.is_empty() {
        0.0
    } else {
        downside.iter().map(|r| r.powi(2)).sum::<f64>() / downside.len() as f64
    };
    let dstd = dvar.sqrt();

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
    let sortino = if dstd > 1e-12 {
        mean / dstd * TRADING_DAYS.sqrt()
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

    let bh_peak = buyhold.iter().cloned().fold(1.0_f64, f64::max);
    let bh_max_dd = buyhold
        .iter()
        .scan(1.0_f64, |peak, e| {
            if *e > *peak {
                *peak = *e;
            }
            Some(e / *peak - 1.0)
        })
        .fold(0.0_f64, f64::min);
    let bh_final = *buyhold.last().ok_or("buyhold underflow")?;
    let bh_cagr = if bh_final > 0.0 && years > 0.0 {
        bh_final.powf(1.0 / years) - 1.0
    } else {
        -1.0
    };

    let wins = trade_pnls.iter().filter(|p| **p > 0.0).count();
    let win_rate = if trade_pnls.is_empty() {
        0.0
    } else {
        wins as f64 / trade_pnls.len() as f64
    };

    Ok(BacktestResult {
        equity,
        buyhold,
        drawdown,
        windows,
        cagr,
        sharpe,
        sortino,
        max_drawdown: max_dd,
        win_rate,
        trades_per_year: flips as f64 / years.max(1e-9),
        bh_cagr,
        bh_max_drawdown: bh_max_dd.min(bh_peak * 0.0 + bh_max_dd),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Deterministic uptrend with pullbacks: sine wave + drift.
    fn synthetic(n: usize) -> Vec<f64> {
        (0..n)
            .map(|i| {
                let t = i as f64;
                100.0 * (1.0 + 0.002 * t + 0.03 * (t / 15.0).sin())
            })
            .collect()
    }

    #[test]
    fn basic_run_shapes() {
        let closes = synthetic(400);
        let r = walk_forward(&closes, &[5, 10], &[20, 50], 120, 40, 1.0, 1.0).unwrap();
        assert_eq!(r.equity.len(), 400);
        assert_eq!(r.buyhold.len(), 400);
        assert_eq!(r.drawdown.len(), 400);
        assert!(!r.windows.is_empty());
        // First OOS segment starts exactly at is_window.
        assert_eq!(r.windows[0].start, 120);
        assert!(r.equity.iter().all(|e| e.is_finite() && *e > 0.0));
        // Buy&hold matches price ratio.
        assert!((r.buyhold[399] - closes[399] / closes[0]).abs() < 1e-12);
    }

    #[test]
    fn costs_reduce_equity() {
        let closes = synthetic(400);
        let free = walk_forward(&closes, &[5, 10], &[20, 50], 120, 40, 0.0, 0.0).unwrap();
        let costly = walk_forward(&closes, &[5, 10], &[20, 50], 120, 40, 5.0, 5.0).unwrap();
        let f = *free.equity.last().unwrap();
        let c = *costly.equity.last().unwrap();
        assert!(c <= f + 1e-12, "costs must not improve equity: {c} vs {f}");
    }

    #[test]
    fn too_short_rejected() {
        let closes = synthetic(50);
        assert!(walk_forward(&closes, &[5], &[20], 120, 40, 1.0, 1.0).is_err());
    }

    #[test]
    fn empty_grid_rejected() {
        let closes = synthetic(400);
        assert!(walk_forward(&closes, &[], &[20], 120, 40, 1.0, 1.0).is_err());
    }

    #[test]
    fn nonpositive_prices_rejected() {
        let mut closes = synthetic(400);
        closes[10] = 0.0;
        assert!(walk_forward(&closes, &[5], &[20], 120, 40, 1.0, 1.0).is_err());
    }

    #[test]
    fn windows_tile_the_oos_range() {
        let closes = synthetic(500);
        let r = walk_forward(&closes, &[5, 10], &[20, 50], 120, 40, 1.0, 1.0).unwrap();
        assert_eq!(r.windows[0].start, 120);
        for w in r.windows.windows(2) {
            assert_eq!(w[1].start, w[0].end, "windows must be contiguous");
        }
        assert_eq!(r.windows.last().unwrap().end, 500);
    }

    #[test]
    fn max_drawdown_nonpositive() {
        let closes = synthetic(400);
        let r = walk_forward(&closes, &[5, 10], &[20, 50], 120, 40, 1.0, 1.0).unwrap();
        assert!(r.max_drawdown <= 0.0);
        assert!(r.bh_max_drawdown <= 0.0);
    }

    #[test]
    fn serde_roundtrip() {
        let closes = synthetic(300);
        let r = walk_forward(&closes, &[5], &[20], 100, 40, 1.0, 1.0).unwrap();
        let json = serde_json::to_string(&r).unwrap();
        let back: BacktestResult = serde_json::from_str(&json).unwrap();
        assert_eq!(back.equity.len(), r.equity.len());
        assert_eq!(back.windows.len(), r.windows.len());
    }
}
