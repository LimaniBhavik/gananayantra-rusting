/// Calculates the cost per good die.
///
/// # Arguments
/// * `wafer_cost` - Total cost of the processed wafer
/// * `net_good_dies` - Number of yielding dies
///
/// # Returns
/// * Cost per die
pub fn calculate_cost_per_die(wafer_cost: f64, net_good_dies: f64) -> Result<f64, String> {
    if wafer_cost < 0.0 {
        return Err("Cost cannot be negative".into());
    }
    if net_good_dies <= 0.0 {
        return Err("Net good dies must be greater than zero".into());
    }
    Ok(wafer_cost / net_good_dies)
}
