/// Calculates Battery Life for a robot.
///
/// # Arguments
/// * `capacity_wh` - Battery capacity in Wh
/// * `avg_power_load_w` - Average power consumption in Watts
/// * `safety_margin` - Safety margin (e.g. 0.2 for 20%)
///
/// # Returns
/// * Runtime in hours
pub fn calculate_robot_runtime(capacity_wh: f64, avg_power_load_w: f64, safety_margin: f64) -> Result<f64, String> {
    if avg_power_load_w <= 0.0 { return Err("Load must be positive".into()); }
    let usable_capacity = capacity_wh * (1.0 - safety_margin);
    Ok(usable_capacity / avg_power_load_w)
}

/// Calculates Actuator Efficiency.
///
/// # Arguments
/// * `mechanical_power_out` - Output power (Force * Velocity or Torque * AngularVel)
/// * `electrical_power_in` - Input power (V * I)
///
/// # Returns
/// * Efficiency (0.0 - 1.0)
pub fn actuator_efficiency(mechanical_power_out: f64, electrical_power_in: f64) -> Result<f64, String> {
    if electrical_power_in <= 0.0 { return Err("Input power must be positive".into()); }
    Ok(mechanical_power_out / electrical_power_in)
}
