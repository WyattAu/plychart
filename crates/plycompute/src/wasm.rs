//! WASM exports — thin #[wasm_bindgen] wrappers over crate::json::*.
//! Logic lives in json.rs (macro-free) so downstream cdylibs can call
//! the same code without wasm-bindgen symbol collisions.

use wasm_bindgen::prelude::*;

use crate::json;

#[wasm_bindgen]
pub fn quant_montecarlo(closes: &[f64],
    periods_per_year: u32,
    horizon_days: usize,
    num_paths: usize,) -> String {
    json::quant_montecarlo(closes, periods_per_year, horizon_days, num_paths)
}

#[wasm_bindgen]
pub fn quant_greeks(spot_min: f64,
    spot_max: f64,
    num_points: usize,
    strike: f64,
    time_to_expiry: f64,
    risk_free_rate: f64,
    implied_vol: f64,
    is_call: bool,) -> String {
    json::quant_greeks(spot_min, spot_max, num_points, strike, time_to_expiry, risk_free_rate, implied_vol, is_call)
}

#[wasm_bindgen]
pub fn quant_var(returns: &[f64], alpha: f64) -> String {
    json::quant_var(returns, alpha)
}

#[wasm_bindgen]
pub fn quant_garch(returns: &[f64], forecast_steps: usize) -> String {
    json::quant_garch(returns, forecast_steps)
}

#[wasm_bindgen]
pub fn quant_correlation_matrix(returns: &[f64], n_assets: usize, n_periods: usize) -> String {
    json::quant_correlation_matrix(returns, n_assets, n_periods)
}

#[wasm_bindgen]
pub fn quant_drawdown(prices: &[f64], periods_per_year: u32) -> String {
    json::quant_drawdown(prices, periods_per_year)
}

#[wasm_bindgen]
pub fn quant_concentration(weights: &[f64]) -> String {
    json::quant_concentration(weights)
}

#[wasm_bindgen]
pub fn quant_yield_curve(maturities: &[f64], yields: &[f64]) -> String {
    json::quant_yield_curve(maturities, yields)
}

#[wasm_bindgen]
pub fn quant_seed(s0: f64, s1: f64) {
    json::quant_seed(s0, s1)
}

#[wasm_bindgen]
pub fn quant_factor_regression(y: &[f64], x: &[f64], n_factors: usize, n_obs: usize) -> String {
    json::quant_factor_regression(y, x, n_factors, n_obs)
}

#[wasm_bindgen]
pub fn quant_efficient_frontier(returns: &[f64],
    n_assets: usize,
    n_periods: usize,
    risk_free: f64,
    n_random: usize,
    n_frontier: usize,
    periods_per_year: u32,) -> String {
    json::quant_efficient_frontier(returns, n_assets, n_periods, risk_free, n_random, n_frontier, periods_per_year)
}

#[wasm_bindgen]
pub fn quant_cointegration(y: &[f64], x: &[f64]) -> String {
    json::quant_cointegration(y, x)
}

#[wasm_bindgen]
pub fn quant_liquidity(highs: &[f64], lows: &[f64], closes: &[f64], volumes: &[f64]) -> String {
    json::quant_liquidity(highs, lows, closes, volumes)
}

#[wasm_bindgen]
pub fn quant_realized_vol(returns: &[f64]) -> String {
    json::quant_realized_vol(returns)
}

#[wasm_bindgen]
pub fn quant_tail_dependence(x: &[f64], y: &[f64]) -> String {
    json::quant_tail_dependence(x, y)
}

#[wasm_bindgen]
pub fn quant_regime(returns: &[f64]) -> String {
    json::quant_regime(returns)
}

#[wasm_bindgen]
pub fn quant_holdings_overlap(a: &str, b: &str) -> String {
    json::quant_holdings_overlap(a, b)
}

#[wasm_bindgen]
pub fn quant_pairs_signal(y: &[f64], x: &[f64]) -> String {
    json::quant_pairs_signal(y, x)
}

#[wasm_bindgen]
pub fn quant_stress_test(symbols_json: &str, weights: &[f64]) -> String {
    json::quant_stress_test(symbols_json, weights)
}

#[wasm_bindgen]
pub fn quant_component_var(returns: &[f64],
    n_assets: usize,
    n_periods: usize,
    weights: &[f64],
    alpha: f64,) -> String {
    json::quant_component_var(returns, n_assets, n_periods, weights, alpha)
}

#[wasm_bindgen]
pub fn quant_kelly(returns: &[f64]) -> String {
    json::quant_kelly(returns)
}

#[wasm_bindgen]
pub fn quant_hrp(returns: &[f64], n_assets: usize, n_periods: usize) -> String {
    json::quant_hrp(returns, n_assets, n_periods)
}

#[wasm_bindgen]
pub fn quant_risk_ratios(closes: &[f64], risk_free: f64, periods_per_year: u32) -> String {
    json::quant_risk_ratios(closes, risk_free, periods_per_year)
}

#[wasm_bindgen]
pub fn quant_forward_rates(maturities: &[f64], yields: &[f64]) -> String {
    json::quant_forward_rates(maturities, yields)
}

#[wasm_bindgen]
pub fn quant_full_overlap(tickers_a: &str,
    weights_a: &[f64],
    tickers_b: &str,
    weights_b: &[f64],) -> String {
    json::quant_full_overlap(tickers_a, weights_a, tickers_b, weights_b)
}

#[wasm_bindgen]
pub fn quant_tail_matrix(returns: &[f64], n_assets: usize, n_periods: usize) -> String {
    json::quant_tail_matrix(returns, n_assets, n_periods)
}

#[wasm_bindgen]
pub fn quant_rolling_factor(y: &[f64],
    x: &[f64],
    n_factors: usize,
    window: usize,
    step: usize,) -> String {
    json::quant_rolling_factor(y, x, n_factors, window, step)
}

#[wasm_bindgen]
pub fn quant_black_litterman(returns: &[f64],
    n_assets: usize,
    n_periods: usize,
    views: &[f64],
    picking: &[f64],
    n_views: usize,
    risk_free: f64,
    tau: f64,) -> String {
    json::quant_black_litterman(returns, n_assets, n_periods, views, picking, n_views, risk_free, tau)
}

#[wasm_bindgen]
pub fn quant_backtest(closes: &[f64],
    fasts: &[usize],
    slows: &[usize],
    is_window: usize,
    oos_window: usize,
    slippage_bps: f64,
    commission_bps: f64,) -> String {
    json::quant_backtest(closes, fasts, slows, is_window, oos_window, slippage_bps, commission_bps)
}
