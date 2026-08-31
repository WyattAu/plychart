pub mod blackscholes;
pub mod cointegration;
pub mod concentration;
pub mod copula;
pub mod drawdown;
pub mod factor;
pub mod hrp;
pub mod liquidity;
pub mod montecarlo;
pub mod overlap;
pub mod pairs;
pub mod portfolio;
pub mod realizedvol;
pub mod regime;
pub mod risk;
pub mod risk_decomp;
pub mod rng;
pub mod stats;
pub mod stress;
pub mod volatility;
pub mod yieldcurve;

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn montecarlo_pipeline() {
        let closes: Vec<f64> = (0..60)
            .map(|i| 100.0 + (i as f64 * 0.5).sin() * 5.0)
            .collect();
        let result = montecarlo::montecarlo_from_prices(&closes, 252, 30, 1000);
        assert!(result.volatility >= 0.0);
        assert!(result.p50.len() == 30);
        assert!(result.p5.len() <= result.p50.len());
    }

    #[test]
    fn stats_correlation_matrix() {
        let returns = vec![0.01, 0.02, -0.01, 0.03, 0.02, 0.01, -0.02, 0.04];
        let mat = stats::correlation_matrix(&returns, 2, 4);
        assert!((mat[0] - 1.0).abs() < 0.001);
        assert!((mat[3] - 1.0).abs() < 0.001);
        assert!((mat[1] - mat[2]).abs() < 0.001);
    }

    #[test]
    fn stats_covariance_matrix_symmetry() {
        let returns = vec![0.01, 0.02, -0.01, 0.03, 0.02, 0.01];
        let cov = stats::covariance_matrix(&returns, 2, 3);
        assert!((cov[1] - cov[2]).abs() < 1e-10);
    }

    #[test]
    fn stats_matrix_inverse() {
        let mat = vec![2.0, 1.0, 1.0, 3.0];
        let inv = stats::matrix_inverse(&mat, 2).unwrap();
        let det = 2.0 * 3.0 - 1.0 * 1.0;
        assert!((inv[0] - 3.0 / det).abs() < 1e-10);
    }

    #[test]
    fn risk_var_parametric() {
        let var = risk::var_parametric(0.0, 0.02, 0.05);
        assert!(var < 0.0);
    }

    #[test]
    fn risk_es_parametric() {
        let es = risk::es_parametric(0.0, 0.02, 0.05);
        assert!(es < 0.0);
    }

    #[test]
    fn risk_drawdown_series_length() {
        let prices = vec![100.0, 110.0, 105.0, 90.0, 95.0];
        let dd = risk::drawdown_series(&prices);
        assert_eq!(dd.len(), 5);
        assert!((dd[0] - 0.0).abs() < 1e-10);
    }

    #[test]
    fn portfolio_efficient_frontier() {
        rng::seed(42, 123);
        let mut returns = vec![0.0; 2 * 50];
        for i in 0..2 {
            for t in 0..50 {
                returns[i * 50 + t] = 0.001 * (i as f64 + 1.0) + 0.01 * rng::standard_normal();
            }
        }
        let frontier = portfolio::efficient_frontier(&returns, 2, 50, 0.02, 10, 252);
        assert!(frontier.len() > 3);
    }

    #[test]
    fn volatility_ewma_length() {
        let returns = vec![0.01, -0.02, 0.005, 0.03, -0.01, 0.02];
        let vol = volatility::ewma_volatility(&returns, 0.94);
        assert_eq!(vol.len(), 6);
    }

    #[test]
    fn blackscholes_deep_itm() {
        let price = blackscholes::bs_call_price(200.0, 100.0, 1.0, 0.05, 0.2);
        assert!(price > 100.0);
    }

    #[test]
    fn blackscholes_put_deep_itm() {
        let price = blackscholes::bs_put_price(50.0, 100.0, 1.0, 0.05, 0.2);
        assert!(price > 45.0);
    }

    #[test]
    fn concentration_diversified() {
        let w = vec![0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1];
        let result = concentration::full_analysis(&w);
        assert_eq!(result.classification, "Diversified");
    }

    #[test]
    fn concentration_concentrated() {
        let w = vec![0.9, 0.05, 0.03, 0.02];
        let result = concentration::full_analysis(&w);
        assert_eq!(result.classification, "Highly Concentrated");
    }

    #[test]
    fn drawdown_analyze_single_price() {
        let prices = vec![100.0];
        let dd = drawdown::analyze_drawdowns(&prices);
        assert_eq!(dd.max_drawdown, 0.0);
    }

    #[test]
    fn drawdown_pain_index() {
        let prices = vec![100.0, 110.0, 90.0, 95.0, 85.0];
        let pain = drawdown::pain_index(&prices);
        assert!(pain > 0.0);
    }

    #[test]
    fn drawdown_ulcer_index() {
        let prices = vec![100.0, 110.0, 90.0, 95.0];
        let u = drawdown::ulcer_index(&prices);
        assert!(u > 0.0);
    }

    #[test]
    fn montecarlo_mean_std_dev() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(montecarlo::mean(&data), 3.0);
        let sd = montecarlo::std_dev(&data);
        assert!((sd - (2.0_f64).sqrt()).abs() < 1e-10);
    }

    #[test]
    fn montecarlo_annualize() {
        let ann_ret = montecarlo::annualize_return(0.001, 252);
        assert!((ann_ret - 0.252).abs() < 1e-10);
        let ann_vol = montecarlo::annualize_volatility(0.01, 252);
        assert!((ann_vol - 0.01 * (252.0_f64).sqrt()).abs() < 1e-10);
    }

    #[test]
    fn risk_sortino_all_positive_returns() {
        let returns = vec![0.01, 0.02, 0.03, 0.04];
        let so = risk::sortino_ratio(&returns, 0.02, 252);
        assert_eq!(so, 0.0);
    }

    #[test]
    fn stats_tangency_equal_weights_fallback() {
        let cov = vec![0.0, 0.0, 0.0, 0.0];
        let mean = vec![0.0, 0.0];
        let w = stats::tangency_portfolio(&cov, &mean, 0.0, 2);
        assert_eq!(w.len(), 2);
        assert!((w[0] + w[1] - 1.0).abs() < 0.001);
    }

    #[test]
    fn stats_evaluate_portfolio_equal_weight() {
        let w = vec![0.5, 0.5];
        let mean = vec![0.10, 0.05];
        let cov = vec![0.0225, 0.0045, 0.0045, 0.01];
        let (ret, risk, sharpe) = stats::evaluate_portfolio(&w, &mean, &cov, 0.02, 2);
        assert!(ret > 0.0);
        assert!(risk > 0.0);
        assert!(sharpe > 0.0);
    }

    #[test]
    fn montecarlo_log_returns_length() {
        let prices = vec![100.0, 105.0, 103.0, 110.0];
        let r = montecarlo::log_returns(&prices);
        assert_eq!(r.len(), 3);
    }

    #[test]
    fn risk_histogram_bins_sum() {
        let data: Vec<f64> = (0..100).map(|i| (i as f64 - 50.0) / 10.0).collect();
        let (edges, counts) = risk::histogram(&data, 10);
        assert_eq!(edges.len(), 11);
        assert_eq!(counts.iter().sum::<usize>(), 100);
    }

    #[test]
    fn blackscholes_implied_vol_put() {
        let true_iv = 0.25;
        let price = blackscholes::bs_put_price(100.0, 105.0, 0.5, 0.03, true_iv);
        let recovered = blackscholes::implied_vol(price, 100.0, 105.0, 0.5, 0.03, false);
        assert!((recovered - true_iv).abs() < 0.002);
    }
}
