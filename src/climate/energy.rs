/// Calculates ROI for Solar Panel installation.
///
/// # Arguments
/// * `system_cost` - Total cost of installation
/// * `annual_generation_kwh` - Expected annual generation
/// * `electricity_rate` - Price per kWh
///
/// # Returns
/// * ROI percentage
pub fn calculate_solar_roi(system_cost: f64, annual_generation_kwh: f64, electricity_rate: f64) -> Result<f64, String> {
    if system_cost <= 0.0 { return Err("System cost must be positive".into()); }
    let annual_savings = annual_generation_kwh * electricity_rate;
    Ok((annual_savings / system_cost) * 100.0)
}

/// Calculates required battery capacity.
///
/// # Arguments
/// * `daily_load_kwh` - Daily energy consumption
/// * `autonomy_days` - Days of backup needed
/// * `depth_of_discharge` - Max DOD (e.g., 0.8 for 80%)
///
/// # Returns
/// * Required Capacity in kWh
pub fn calculate_battery_capacity(daily_load_kwh: f64, autonomy_days: f64, depth_of_discharge: f64) -> Result<f64, String> {
    if depth_of_discharge <= 0.0 || depth_of_discharge > 1.0 {
        return Err("DOD must be between 0 and 1".into());
    }
    Ok((daily_load_kwh * autonomy_days) / depth_of_discharge)
}
