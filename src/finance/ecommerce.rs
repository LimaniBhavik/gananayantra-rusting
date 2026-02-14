pub fn calculate_discount(
    original_price: f64,
    discount_percent: f64,
) -> Result<(f64, f64), String> {
    if original_price < 0.0 || discount_percent < 0.0 {
        return Err("Values must be non-negative".into());
    }
    let savings = original_price * (discount_percent / 100.0);
    Ok((savings, original_price - savings))
}

pub fn calculate_profit_margin(cost: f64, revenue: f64) -> Result<(f64, f64), String> {
    if revenue <= 0.0 {
        return Err("Revenue must be greater than zero".into());
    }
    let gross_profit = revenue - cost;
    let margin = (gross_profit / revenue) * 100.0;
    Ok((gross_profit, margin))
}
