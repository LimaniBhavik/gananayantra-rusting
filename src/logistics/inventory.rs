/// Calculates Economic Order Quantity (EOQ).
///
/// # Arguments
/// * `demand_rate` - Annual demand quantity
/// * `ordering_cost` - Cost per order (setup cost)
/// * `holding_cost` - Annual holding cost per unit
///
/// # Returns
/// * Optimal order quantity
pub fn calculate_eoq(demand_rate: f64, ordering_cost: f64, holding_cost: f64) -> Result<f64, String> {
    if demand_rate <= 0.0 || ordering_cost <= 0.0 || holding_cost <= 0.0 {
        return Err("All inputs must be greater than zero".into());
    }
    Ok(((2.0 * demand_rate * ordering_cost) / holding_cost).sqrt())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eoq() {
        // D=1000, S=10, H=2 -> sqrt(2*1000*10 / 2) = sqrt(10000) = 100
        assert_eq!(calculate_eoq(1000.0, 10.0, 2.0).unwrap(), 100.0);
    }
}
