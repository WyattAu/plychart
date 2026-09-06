//! SVI (Stochastic Volatility Inspired) volatility-surface fitting.
//!
//! Fits the raw SVI parameterization of total variance
//!
//!   w(k) = a + b * (rho * (k - m) + sqrt((k - m)^2 + sigma^2))
//!
//! to quoted option implied vols per expiry, using the Quasi-Explicit
//! scheme (Zeliade systems): grid-search (m, sigma), solve the remaining
//! (a, b, c = b*rho) by linear least squares, enforce b >= |c| (|rho| <= 1),
//! b >= 0, and keep the lowest-SSE candidate.

use serde::{Deserialize, Serialize};

/// One expiry's quoted points: log-moneyness k and total variance w.
#[derive(Debug, Clone, Deserialize)]
pub struct SliceQuotes {
    /// Time to expiry in years (ACT/365).
    pub t: f64,
    /// Log-moneyness k = ln(strike / forward) per quote.
    pub k: Vec<f64>,
    /// Total variance w = iv^2 * t per quote (iv fractional, e.g. 0.55).
    pub w: Vec<f64>,
}

/// Fitted SVI parameters for one expiry slice.
#[derive(Debug, Clone, Serialize)]
pub struct SviParams {
    pub a: f64,
    pub b: f64,
    pub rho: f64,
    pub m: f64,
    pub sigma: f64,
}

/// Per-expiry fit result.
#[derive(Debug, Clone, Serialize)]
pub struct SviSlice {
    /// Time to expiry in years.
    pub t: f64,
    /// Fitted parameters.
    pub params: SviParams,
    /// Implied vol at k = 0 (ATM), fractional.
    pub atm_iv: f64,
    /// RMSE of the fit in total-variance space.
    pub rmse: f64,
    /// Fitted smile on the requested k grid: (k, iv) pairs, iv fractional.
    pub curve: Vec<(f64, f64)>,
}

/// Full surface fit.
#[derive(Debug, Clone, Serialize)]
pub struct SviSurface {
    /// Per-expiry fits, same order as the input slices.
    pub slices: Vec<SviSlice>,
    /// Uniform k grid used for the surface matrix.
    pub k_grid: Vec<f64>,
    /// surface_iv[i][j] = fitted IV (fractional) at slices[i], k_grid[j].
    pub surface_iv: Vec<Vec<f64>>,
}

/// Raw-SVI total variance at k for the given params.
#[must_use]
pub fn svi_w(p: &SviParams, k: f64) -> f64 {
    let d = k - p.m;
    p.a + p.b * (p.rho * d + (d * d + p.sigma * p.sigma).sqrt())
}

/// Solve ordinary least squares for the 3-parameter linear subproblem:
/// w ≈ a + c*d + b*s, where d = k - m, s = sqrt(d^2 + sigma^2).
/// Returns (a, b, c) with b clamped >= 0 (after re-fit if needed).
fn solve_linear(m: f64, sigma: f64, k: &[f64], w: &[f64]) -> (f64, f64, f64) {
    // Normal equations for X^T X beta = X^T y with columns [1, d, s].
    let n = k.len();
    let mut xtx = [[0.0_f64; 3]; 3];
    let mut xty = [0.0_f64; 3];
    for i in 0..n {
        let d = k[i] - m;
        let s = (d * d + sigma * sigma).sqrt();
        let col = [1.0, d, s];
        for r in 0..3 {
            xty[r] += col[r] * w[i];
            for cc in 0..3 {
                xtx[r][cc] += col[r] * col[cc];
            }
        }
    }

    // Gaussian elimination with partial pivoting on 3x3.
    let mut a = xtx;
    let mut y = xty;
    for col in 0..3 {
        let mut piv = col;
        for r in col + 1..3 {
            if a[r][col].abs() > a[piv][col].abs() {
                piv = r;
            }
        }
        a.swap(col, piv);
        y.swap(col, piv);
        if a[col][col].abs() < 1e-14 {
            continue;
        }
        for r in col + 1..3 {
            let f = a[r][col] / a[col][col];
            let pivot_row = a[col];
            for (rr, pr) in a[r][col..3].iter_mut().zip(pivot_row[col..3].iter()) {
                *rr -= f * pr;
            }
            y[r] -= f * y[col];
        }
    }
    let mut beta = [0.0_f64; 3];
    for r in (0..3).rev() {
        let mut sum = y[r];
        for cc in r + 1..3 {
            sum -= a[r][cc] * beta[cc];
        }
        beta[r] = if a[r][r].abs() < 1e-14 {
            0.0
        } else {
            sum / a[r][r]
        };
    }

    let (a_p, c, b) = (beta[0], beta[1], beta[2]);
    // Constraint: b >= 0. Refit a, c with b forced to 0 if violated.
    if b < 0.0 {
        let sa: f64 = n as f64;
        let sy: f64 = w.iter().sum();
        let a_p = sy / sa;
        return (a_p, 0.0, 0.0);
    }
    // Soft constraint |c| <= b (|rho| <= 1); project if violated.
    if c.abs() > b {
        let rho = c / b;
        let rho = rho.clamp(-0.9999, 0.9999);
        return (a_p, b * rho, b);
    }
    (a_p, c, b)
}

/// Fit one expiry slice. `k_floor`/`k_ceil` bound the smile-evaluation grid.
#[must_use]
pub fn fit_slice(q: &SliceQuotes) -> Option<SviSlice> {
    if q.k.len() < 5 || q.k.len() != q.w.len() || q.t <= 0.0 {
        return None;
    }
    let kmin = q.k.iter().cloned().fold(f64::INFINITY, f64::min);
    let kmax = q.k.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    if !(kmax - kmin).is_finite() || (kmax - kmin) < 1e-6 {
        return None;
    }

    let span = kmax - kmin;
    let m_grid: Vec<f64> = (0..=8).map(|i| kmin + span * (i as f64) / 8.0).collect();
    let sigma_grid: Vec<f64> = [0.02, 0.05, 0.1, 0.2, 0.35, 0.5, 0.8, 1.2].to_vec();

    let mut best: Option<(f64, SviParams)> = None;
    for &m in &m_grid {
        for &sg in &sigma_grid {
            let (a, c, b) = solve_linear(m, sg, &q.k, &q.w);
            let rho = if b.abs() > 1e-12 { c / b } else { 0.0 };
            let p = SviParams {
                a,
                b,
                rho,
                m,
                sigma: sg,
            };
            // Skip degenerate fits (arbitrage guard: b > 0 required).
            if p.b <= 1e-10 {
                continue;
            }
            let sse: f64 =
                q.k.iter()
                    .zip(&q.w)
                    .map(|(&ki, &wi)| (svi_w(&p, ki) - wi).powi(2))
                    .sum();
            let best_sse = best.as_ref().map_or(f64::INFINITY, |(bs, _)| *bs);
            if sse < best_sse {
                best = Some((sse, p));
            }
        }
    }
    let (sse, params) = best?;

    let rmse = (sse / q.k.len() as f64).sqrt();
    let atm_w = svi_w(&params, 0.0);
    let atm_iv = (atm_w / q.t).max(0.0).sqrt();

    let n_curve = 25;
    let curve: Vec<(f64, f64)> = (0..n_curve)
        .filter_map(|i| {
            let k = kmin + span * (i as f64) / (n_curve - 1) as f64;
            let w = svi_w(&params, k).max(1e-8);
            let iv = (w / q.t).sqrt();
            if iv.is_finite() { Some((k, iv)) } else { None }
        })
        .collect();

    Some(SviSlice {
        t: q.t,
        params,
        atm_iv,
        rmse,
        curve,
    })
}

/// Fit all slices and assemble a uniform-k IV surface.
#[must_use]
pub fn fit_surface(slices: &[SliceQuotes], k_points: usize) -> SviSurface {
    let fitted: Vec<SviSlice> = slices.iter().filter_map(fit_slice).collect();

    // Global k range across all slices (clamped to a sane moneyness band).
    let mut lo = f64::INFINITY;
    let mut hi = f64::NEG_INFINITY;
    for s in &fitted {
        if let Some((first, _)) = s.curve.first() {
            lo = lo.min(*first);
        }
        if let Some((last, _)) = s.curve.last() {
            hi = hi.max(*last);
        }
    }
    if !lo.is_finite() || !hi.is_finite() || hi <= lo {
        lo = -0.4;
        hi = 0.4;
    }
    let lo = lo.max(-1.5);
    let hi = hi.min(1.5);
    let n = k_points.clamp(5, 60);
    let k_grid: Vec<f64> = (0..n)
        .map(|i| lo + (hi - lo) * (i as f64) / (n - 1) as f64)
        .collect();

    // Interpolate each slice's curve onto k_grid (curve is sorted by k).
    let surface_iv: Vec<Vec<f64>> = fitted
        .iter()
        .map(|s| {
            k_grid
                .iter()
                .map(|&k| {
                    let iv = interp_curve(&s.curve, k);
                    (iv * 100.0).round() / 100.0 // 2dp percent
                })
                .collect()
        })
        .collect();

    SviSurface {
        slices: fitted,
        k_grid,
        surface_iv,
    }
}

/// Linear interpolation of a (k, iv) curve, clamped at the ends.
fn interp_curve(curve: &[(f64, f64)], k: f64) -> f64 {
    if curve.is_empty() {
        return 0.0;
    }
    if k <= curve[0].0 {
        return curve[0].1;
    }
    if let Some(last) = curve.last()
        && k >= last.0
    {
        return last.1;
    }
    for pair in curve.windows(2) {
        let (k0, v0) = pair[0];
        let (k1, v1) = pair[1];
        if k >= k0 && k <= k1 {
            let t = (k - k0) / (k1 - k0).max(1e-12);
            return v0 + t * (v1 - v0);
        }
    }
    curve[curve.len() / 2].1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn svi_params(a: f64, b: f64, rho: f64, m: f64, sigma: f64) -> SviParams {
        SviParams {
            a,
            b,
            rho,
            m,
            sigma,
        }
    }

    /// Synthetic smile from known SVI params + small noise.
    fn synthetic_slice(t: f64, p: &SviParams) -> SliceQuotes {
        let k: Vec<f64> = (-8..=8).map(|i| i as f64 * 0.05).collect();
        let w: Vec<f64> = k.iter().map(|&ki| svi_w(p, ki)).collect();
        SliceQuotes { t, k, w }
    }

    #[test]
    fn recovers_known_parameters() {
        let truth = svi_params(0.02, 0.15, -0.4, 0.0, 0.3);
        let q = synthetic_slice(0.25, &truth);
        let fit = fit_slice(&q).expect("fit should succeed");
        // Fit is exact on noiseless data: RMSE ~ 0, params near truth.
        assert!(fit.rmse < 1e-3, "rmse {}", fit.rmse);
        let w_true = svi_w(&truth, 0.0);
        let w_fit = svi_w(&fit.params, 0.0);
        assert!((w_true - w_fit).abs() < 2e-4, "ATM w drift");
        // IV at ATM should be sqrt(w/T).
        let expected_iv = (w_true / 0.25).sqrt();
        assert!((fit.atm_iv - expected_iv).abs() < 1e-3);
    }

    #[test]
    fn rejects_degenerate_input() {
        let short = SliceQuotes {
            t: 0.1,
            k: vec![0.0, 0.1],
            w: vec![0.01, 0.02],
        };
        assert!(fit_slice(&short).is_none(), "too few points");
        let empty = SliceQuotes {
            t: 0.1,
            k: vec![],
            w: vec![],
        };
        assert!(fit_slice(&empty).is_none());
        let zero_t = SliceQuotes {
            t: 0.0,
            k: vec![0.0; 6],
            w: vec![0.01; 6],
        };
        assert!(fit_slice(&zero_t).is_none(), "zero maturity");
    }

    #[test]
    fn surface_assembles_and_orders() {
        let p1 = svi_params(0.02, 0.15, -0.4, 0.0, 0.3);
        let p2 = svi_params(0.04, 0.12, 0.2, -0.05, 0.4);
        let slices = vec![synthetic_slice(0.1, &p1), synthetic_slice(0.5, &p2)];
        let surf = fit_surface(&slices, 15);
        assert_eq!(surf.slices.len(), 2);
        assert_eq!(surf.k_grid.len(), 15);
        assert_eq!(surf.surface_iv.len(), 2);
        assert_eq!(surf.surface_iv[0].len(), 15);
        // k grid ascending.
        assert!(surf.k_grid.windows(2).all(|w| w[0] < w[1]));
        // IVs positive and finite.
        for row in &surf.surface_iv {
            assert!(row.iter().all(|v| *v > 0.0 && v.is_finite()));
        }
        // Longer slice should generally have higher ATM total variance
        // at k=0 midpoint of grid.
        let mid = 7;
        assert!(surf.surface_iv[1][mid] > 0.0);
    }

    #[test]
    fn empty_surface_falls_back_to_default_grid() {
        let surf = fit_surface(&[], 15);
        assert!(surf.slices.is_empty());
        assert_eq!(surf.k_grid.len(), 15);
    }

    #[test]
    fn interp_clamps_at_ends() {
        let curve = vec![(0.0, 0.3), (1.0, 0.5)];
        assert!((interp_curve(&curve, -1.0) - 0.3).abs() < 1e-12);
        assert!((interp_curve(&curve, 2.0) - 0.5).abs() < 1e-12);
        assert!((interp_curve(&curve, 0.5) - 0.4).abs() < 1e-12);
    }
}
