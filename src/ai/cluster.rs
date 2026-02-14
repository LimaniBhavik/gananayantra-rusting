/// Calculates Total Cost of Ownership (TCO) for a GPU cluster.
///
/// # Arguments
/// * `nodes` - Number of server nodes
/// * `gpus_per_node` - GPUs per node
/// * `node_cost` - Hardware cost per node (including GPUs)
/// * `networking_cost` - Total networking infrastructure cost
/// * `operating_cost_per_month` - Power, cooling, maintenance per month
/// * `useful_life_years` - Depreciation period
///
/// # Returns
/// * Total Annual Cost (CapEx amortized + OpEx)
pub fn calculate_gpu_cluster_cost(
    nodes: f64,
    _gpus_per_node: f64, // Used for capacity planning but not direct TCO if node_cost includes it
    node_cost: f64,
    networking_cost: f64,
    operating_cost_per_month: f64,
    useful_life_years: f64
) -> Result<f64, String> {
    if useful_life_years <= 0.0 {
        return Err("Useful life must be positive".into());
    }
    let capex = (nodes * node_cost) + networking_cost;
    let annual_capex = capex / useful_life_years;
    let annual_opex = operating_cost_per_month * 12.0;
    Ok(annual_capex + annual_opex)
}

/// Calculates Power Usage Effectiveness (PUE).
///
/// # Arguments
/// * `total_facility_power` - Total power entering the data center
/// * `it_equipment_power` - Power consumed by servers/storage/network
///
/// # Returns
/// * PUE Ratio (Ideal is 1.0)
pub fn calculate_pue(total_facility_power: f64, it_equipment_power: f64) -> Result<f64, String> {
    if it_equipment_power <= 0.0 {
        return Err("IT power must be greater than zero".into());
    }
    let pue = total_facility_power / it_equipment_power;
    if pue < 1.0 {
        return Err("PUE cannot be less than 1.0".into());
    }
    Ok(pue)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pue() {
        assert_eq!(calculate_pue(150.0, 100.0).unwrap(), 1.5);
    }
}
