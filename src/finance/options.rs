use std::f64::consts::PI;

/// Standard Normal Probability Density Function (PDF).
fn norm_pdf(x: f64) -> f64 {
    (1.0 / (2.0 * PI).sqrt()) * (-0.5 * x * x).exp()
}

// Minimal error function approximation if libm is not available or desired.
// However, since we are in a library, using `libm` crate is common for no_std,
// but here we want zero-dependencies. We will implement a self-contained approximation.
// Abramowitz and Stegun 7.1.26
fn erf_approx(x: f64) -> f64 {
    let a1 =  0.254829592;
    let a2 = -0.284496736;
    let a3 =  1.421413741;
    let a4 = -1.453152027;
    let a5 =  1.061405429;
    let p  =  0.3275911;

    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();

    let t = 1.0 / (1.0 + p * x);
    let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-x * x).exp();

    sign * y
}

fn norm_cdf_approx(x: f64) -> f64 {
    0.5 * (1.0 + erf_approx(x / std::f64::consts::SQRT_2))
}

/// Calculates Call Option Price using Black-Scholes.
///
/// # Arguments
/// * `s` - Current stock price
/// * `k` - Strike price
/// * `t` - Time to maturity (years)
/// * `r` - Risk-free interest rate
/// * `sigma` - Volatility (std dev)
///
/// # Returns
/// * Option Price
pub fn calculate_call_option_price(s: f64, k: f64, t: f64, r: f64, sigma: f64) -> Result<f64, String> {
    if s <= 0.0 || k <= 0.0 || t <= 0.0 || sigma <= 0.0 {
        return Err("Inputs (s, k, t, sigma) must be strictly positive".into());
    }
    let d1 = ( (s / k).ln() + (r + 0.5 * sigma.powi(2)) * t ) / (sigma * t.sqrt());
    let d2 = d1 - sigma * t.sqrt();

    let price = s * norm_cdf_approx(d1) - k * (-r * t).exp() * norm_cdf_approx(d2);
    Ok(price)
}

pub struct Greeks {
    pub delta: f64,
    pub gamma: f64,
    pub theta: f64,
}

/// Calculates Option Greeks (Delta, Gamma, Theta) for a Call Option.
pub fn calculate_greeks(s: f64, k: f64, t: f64, r: f64, sigma: f64) -> Result<Greeks, String> {
    if s <= 0.0 || k <= 0.0 || t <= 0.0 || sigma <= 0.0 {
        return Err("Inputs (s, k, t, sigma) must be strictly positive for Greeks calculation".into());
    }
    let d1 = ( (s / k).ln() + (r + 0.5 * sigma.powi(2)) * t ) / (sigma * t.sqrt());
    let d2 = d1 - sigma * t.sqrt();

    let delta = norm_cdf_approx(d1);
    let gamma = norm_pdf(d1) / (s * sigma * t.sqrt());
    let theta = -(s * norm_pdf(d1) * sigma) / (2.0 * t.sqrt()) - r * k * (-r * t).exp() * norm_cdf_approx(d2);

    Ok(Greeks { delta, gamma, theta })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_black_scholes() {
        // S=100, K=100, T=1, r=0.05, sigma=0.2
        // Call Price ~ 10.45
        let price = calculate_call_option_price(100.0, 100.0, 1.0, 0.05, 0.2).unwrap();
        assert!((price - 10.45).abs() < 0.1);
    }
}
