use wasm_bindgen::prelude::*;

use crate::{
    blackscholes, concentration, drawdown, montecarlo, risk, rng, stats, volatility, yieldcurve,
};

#[wasm_bindgen]
pub fn quant_montecarlo(
    closes: &[f64],
    periods_per_year: u32,
    horizon_days: usize,
    num_paths: usize,
) -> String {
    let result =
        montecarlo::montecarlo_from_prices(closes, periods_per_year, horizon_days, num_paths);
    serde_json::to_string(&serde_json::json!({
        "drift": result.drift,
        "volatility": result.volatility,
        "s0": result.s0,
        "p5": result.p5,
        "p25": result.p25,
        "p50": result.p50,
        "p75": result.p75,
        "p95": result.p95,
    }))
    .unwrap_or_else(|_| "{}".to_string())
}

#[wasm_bindgen]
pub fn quant_greeks(
    spot_min: f64,
    spot_max: f64,
    num_points: usize,
    strike: f64,
    time_to_expiry: f64,
    risk_free_rate: f64,
    implied_vol: f64,
    is_call: bool,
) -> String {
    let step = (spot_max - spot_min) / (num_points.max(1) - 1).max(1) as f64;
    let mut spots = Vec::with_capacity(num_points);
    let mut deltas = Vec::with_capacity(num_points);
    let mut gammas = Vec::with_capacity(num_points);
    let mut thetas = Vec::with_capacity(num_points);
    let mut vegas = Vec::with_capacity(num_points);
    let mut rhos = Vec::with_capacity(num_points);

    for i in 0..num_points {
        let s = spot_min + i as f64 * step;
        let (d, g, t, v, r) = if is_call {
            blackscholes::call_greeks(s, strike, time_to_expiry, risk_free_rate, implied_vol)
        } else {
            blackscholes::put_greeks(s, strike, time_to_expiry, risk_free_rate, implied_vol)
        };
        spots.push(s);
        deltas.push(d);
        gammas.push(g);
        thetas.push(t);
        vegas.push(v);
        rhos.push(r);
    }

    serde_json::to_string(&serde_json::json!({
        "spot": spots, "delta": deltas, "gamma": gammas,
        "theta": thetas, "vega": vegas, "rho": rhos,
    }))
    .unwrap_or_else(|_| "{}".to_string())
}

#[wasm_bindgen]
pub fn quant_var(returns: &[f64], alpha: f64) -> String {
    let var_hist = risk::var_historical(returns, alpha);
    let es_hist = risk::expected_shortfall(returns, alpha);
    let m = montecarlo::mean(returns);
    let s = montecarlo::std_dev(returns);
    let var_par = risk::var_parametric(m, s, alpha);
    let es_par = risk::es_parametric(m, s, alpha);
    let (edges, counts) = risk::histogram(returns, 50);

    serde_json::to_string(&serde_json::json!({
        "var_historical": var_hist,
        "es_historical": es_hist,
        "var_parametric": var_par,
        "es_parametric": es_par,
        "histogram_edges": edges,
        "histogram_counts": counts,
    }))
    .unwrap_or_else(|_| "{}".to_string())
}

#[wasm_bindgen]
pub fn quant_garch(returns: &[f64], forecast_steps: usize) -> String {
    let result = volatility::fit_garch11(returns, forecast_steps);
    let ewma = volatility::ewma_volatility(returns, 0.94);
    let realized = volatility::realized_volatility(returns, 21.min(returns.len()));

    serde_json::to_string(&serde_json::json!({
        "omega": result.omega,
        "alpha": result.alpha,
        "beta": result.beta,
        "long_run_var": result.long_run_var,
        "conditional_vars": result.conditional_vars,
        "forecast": result.forecast,
        "ewma_vol": ewma,
        "realized_vol": realized,
    }))
    .unwrap_or_else(|_| "{}".to_string())
}

#[wasm_bindgen]
pub fn quant_correlation_matrix(returns: &[f64], n_assets: usize, n_periods: usize) -> String {
    let matrix = stats::correlation_matrix(returns, n_assets, n_periods);
    serde_json::to_string(&serde_json::json!({
        "matrix": matrix,
        "n_assets": n_assets,
    }))
    .unwrap_or_else(|_| "{}".to_string())
}

#[wasm_bindgen]
pub fn quant_drawdown(prices: &[f64], periods_per_year: u32) -> String {
    let dd = drawdown::analyze_drawdowns(prices);
    let calmar = drawdown::calmar_ratio(prices, periods_per_year);
    let ulcer = drawdown::ulcer_index(prices);
    let pain = drawdown::pain_index(prices);
    serde_json::to_string(&serde_json::json!({
        "underwater": dd.underwater,
        "max_drawdown": dd.max_drawdown,
        "max_dd_duration": dd.max_dd_duration,
        "current_drawdown": dd.current_drawdown,
        "calmar": calmar,
        "ulcer_index": ulcer,
        "pain_index": pain,
    }))
    .unwrap_or_else(|_| "{}".to_string())
}

#[wasm_bindgen]
pub fn quant_concentration(weights: &[f64]) -> String {
    let result = concentration::full_analysis(weights);
    serde_json::to_string(&serde_json::json!({
        "hhi": result.hhi,
        "normalised_hhi": result.normalised_hhi,
        "effective_n": result.effective_n,
        "entropy": result.entropy,
        "max_entropy": result.max_entropy,
        "top5_concentration": result.top5_concentration,
        "top10_concentration": result.top10_concentration,
        "gini": result.gini,
        "classification": result.classification,
    }))
    .unwrap_or_else(|_| "{}".to_string())
}

#[wasm_bindgen]
pub fn quant_yield_curve(maturities: &[f64], yields: &[f64]) -> String {
    let fit = yieldcurve::fit_nelson_siegel(maturities, yields);
    let spread_10y_3m = {
        let mut y10 = 0.0;
        let mut y3m = 0.0;
        for i in 0..maturities.len().min(yields.len()) {
            if (maturities[i] - 10.0).abs() < 0.1 {
                y10 = yields[i];
            }
            if (maturities[i] - 0.25).abs() < 0.1 {
                y3m = yields[i];
            }
        }
        y10 - y3m
    };
    let recession_prob = yieldcurve::recession_probability(spread_10y_3m);

    serde_json::to_string(&serde_json::json!({
        "beta0": fit.beta0,
        "beta1": fit.beta1,
        "beta2": fit.beta2,
        "lambda": fit.lambda,
        "fitted": fit.fitted,
        "spread_10y_3m": spread_10y_3m,
        "recession_probability": recession_prob,
    }))
    .unwrap_or_else(|_| "{}".to_string())
}

#[wasm_bindgen]
pub fn quant_seed(s0: f64, s1: f64) {
    rng::seed(s0 as u64, s1 as u64);
}

#[wasm_bindgen]
pub fn quant_implied_vol(
    market_price: f64,
    spot: f64,
    strike: f64,
    t: f64,
    r: f64,
    is_call: bool,
) -> f64 {
    if t <= 0.0 || market_price <= 0.0 || spot <= 0.0 || strike <= 0.0 {
        return 0.0;
    }
    blackscholes::implied_vol(market_price, spot, strike, t, r, is_call)
}
