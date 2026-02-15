/// Calculates Cost per Prediction.
pub fn cost_per_prediction(total_inference_cost: f64, total_predictions: f64) -> Result<f64, String> {
    if total_predictions <= 0.0 { return Err("Predictions must be > 0".into()); }
    Ok(total_inference_cost / total_predictions)
}

/// Estimates Revenue Impact of AI.
pub fn revenue_impact(base_revenue: f64, improvement_percent: f64) -> Result<f64, String> {
    Ok(base_revenue * (improvement_percent / 100.0))
}

/// Calculates Supply Chain Efficiency (Turnover Ratio).
pub fn supply_chain_efficiency(cogs: f64, avg_inventory: f64) -> Result<f64, String> {
    if avg_inventory == 0.0 { return Err("Inventory cannot be zero".into()); }
    Ok(cogs / avg_inventory)
}
