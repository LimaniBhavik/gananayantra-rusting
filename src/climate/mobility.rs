/// Calculates the cost to charge an Electric Vehicle.
///
/// # Arguments
/// * `battery_capacity_kwh` - Total battery capacity
/// * `current_charge_percent` - Current state of charge (0-100)
/// * `target_charge_percent` - Target state of charge (0-100)
/// * `electricity_rate` - Cost per kWh
/// * `charging_efficiency` - Charger efficiency (0.0 to 1.0, typically 0.85-0.95)
///
/// # Returns
/// * Cost to charge
pub fn calculate_ev_charging_cost(
    battery_capacity_kwh: f64,
    current_charge_percent: f64,
    target_charge_percent: f64,
    electricity_rate: f64,
    charging_efficiency: f64
) -> Result<f64, String> {
    if charging_efficiency <= 0.0 || charging_efficiency > 1.0 {
        return Err("Efficiency must be between 0 and 1".into());
    }
    if target_charge_percent < current_charge_percent {
        return Ok(0.0); // No charging needed
    }
    let needed_percent = target_charge_percent - current_charge_percent;
    let needed_kwh = battery_capacity_kwh * (needed_percent / 100.0);
    let drawn_kwh = needed_kwh / charging_efficiency;
    Ok(drawn_kwh * electricity_rate)
}

/// Calculates estimated EV Range.
///
/// # Arguments
/// * `battery_kwh` - Usable battery capacity
/// * `efficiency_wh_per_km` - Energy consumption (Wh/km)
///
/// # Returns
/// * Range in km
pub fn calculate_ev_range(battery_kwh: f64, efficiency_wh_per_km: f64) -> Result<f64, String> {
    if efficiency_wh_per_km <= 0.0 {
        return Err("Efficiency must be positive".into());
    }
    Ok((battery_kwh * 1000.0) / efficiency_wh_per_km)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_charging_cost() {
        // 100kWh battery, 0->100%, $0.10/kwh, 100% eff -> $10
        assert_eq!(calculate_ev_charging_cost(100.0, 0.0, 100.0, 0.10, 1.0).unwrap(), 10.0);
    }
}
