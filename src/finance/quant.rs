/// Calculates the Sharpe Ratio.
pub fn calculate_sharpe_ratio(return_rate: f64, risk_free_rate: f64, std_dev: f64) -> Result<f64, String> {
    if std_dev == 0.0 {
        return Err("Standard deviation cannot be zero".into());
    }
    Ok((return_rate - risk_free_rate) / std_dev)
}

/// Calculates Value at Risk (VaR) using the parametric method (Normal Distribution).
/// Assumes 95% confidence (Z = 1.645) or 99% (Z = 2.33).
pub fn calculate_var_parametric(portfolio_value: f64, std_dev: f64, confidence_level: f64) -> Result<f64, String> {
    let z_score = if (confidence_level - 0.95).abs() < 0.01 { 1.645 } else { 2.33 };
    Ok(portfolio_value * std_dev * z_score)
}

/// Simple Monte Carlo simulation for portfolio growth (Geometric Brownian Motion).
///
/// # Arguments
/// * `seed` - A seed for the deterministic random number generator.
pub fn monte_carlo_simulation(
    initial_value: f64,
    mu: f64, // Expected return
    sigma: f64, // Volatility
    years: usize,
    simulations: usize,
    seed: u64
) -> Result<Vec<f64>, String> {
    if simulations == 0 { return Err("Simulations must be > 0".into()); }
    let dt = 1.0; // Annual steps
    let mut results = Vec::with_capacity(simulations);

    let mut current_seed = seed as u128;

    for _ in 0..simulations {
        let mut price = initial_value;
        for _ in 0..years {
            // Box-Muller transform for normal distribution
            current_seed = (current_seed.wrapping_mul(1103515245) + 12345) & 0x7fffffff;
            let u1 = (current_seed as f64) / 2147483648.0;
            current_seed = (current_seed.wrapping_mul(1103515245) + 12345) & 0x7fffffff;
            let u2 = (current_seed as f64) / 2147483648.0;

            let z = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
            price = price * ((mu - 0.5 * sigma.powi(2)) * dt + sigma * dt.sqrt() * z).exp();
        }
        results.push(price);
    }
    Ok(results)
}
