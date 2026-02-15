/// Calculates ROI for an AI Project.
///
/// # Arguments
/// * `gains` - Revenue increase + Cost savings
/// * `costs` - Development + Deployment + Maintenance costs
///
/// # Returns
/// * ROI Percentage
pub fn calculate_ai_roi(gains: f64, costs: f64) -> Result<f64, String> {
    if costs <= 0.0 { return Err("Costs must be positive".into()); }
    Ok(((gains - costs) / costs) * 100.0)
}

/// Calculates AI-driven Customer Acquisition Cost (CAC).
///
/// # Arguments
/// * `total_marketing_spend` - Total spend
/// * `ai_tool_cost` - Cost of AI tools used
/// * `new_customers` - Number of new customers acquired
///
/// # Returns
/// * CAC per customer
pub fn calculate_ai_cac(total_marketing_spend: f64, ai_tool_cost: f64, new_customers: f64) -> Result<f64, String> {
    if new_customers <= 0.0 { return Err("New customers must be > 0".into()); }
    Ok((total_marketing_spend + ai_tool_cost) / new_customers)
}

/// Estimates Customer Lifetime Value (CLV).
///
/// # Arguments
/// * `avg_purchase_value` - Average value per purchase
/// * `purchase_frequency` - Purchases per year
/// * `customer_lifespan` - Average lifespan in years
///
/// # Returns
/// * CLV
pub fn calculate_clv(avg_purchase_value: f64, purchase_frequency: f64, customer_lifespan: f64) -> Result<f64, String> {
    Ok(avg_purchase_value * purchase_frequency * customer_lifespan)
}
