pub fn cd_interest(deposit: f64, apy: f64, months: f64) -> Result<(f64, f64), String> {
    if deposit < 0.0 || apy < 0.0 || months < 0.0 {
        return Err("Values must be non-negative".into());
    }
    let r = apy / 100.0;
    let t = months / 12.0;
    let total = deposit * (1.0 + r).powf(t);
    Ok((total, total - deposit))
}

pub fn bond_current_yield(coupon_rate: f64, market_price: f64) -> Result<(f64, f64), String> {
    if market_price <= 0.0 {
        return Err("Market price must be > 0".into());
    }
    let annual_coupon = 1000.0 * (coupon_rate / 100.0); // Assuming Par=1000
    Ok((annual_coupon, (annual_coupon / market_price) * 100.0))
}

pub fn arithmetic_mean_return(returns: &[f64]) -> Result<f64, String> {
    if returns.is_empty() { return Err("List cannot be empty".into()); }
    let sum: f64 = returns.iter().sum();
    Ok(sum / returns.len() as f64)
}

pub fn approximate_irr(initial_investment: f64, annual_cash_flow: f64, years: f64) -> Result<f64, String> {
    if initial_investment <= 0.0 || annual_cash_flow <= 0.0 { return Err("Invalid inputs".into()); }
    // Newton-Raphson approximation
    let mut guess: f64 = 0.1;
    for _ in 0..100 {
        let f = annual_cash_flow * ((1.0 - (1.0 + guess).powf(-years)) / guess) - initial_investment;
        let df = annual_cash_flow * ((years * (1.0 + guess).powf(-years - 1.0)) / guess - (1.0 - (1.0 + guess).powf(-years)) / (guess * guess));
        if df == 0.0 { break; }
        guess = guess - f / df;
    }
    Ok(guess * 100.0)
}

pub fn payback_period(investment: f64, annual_cash_flow: f64) -> Result<f64, String> {
    if annual_cash_flow == 0.0 { return Err("Cash flow cannot be zero".into()); }
    Ok(investment / annual_cash_flow)
}
