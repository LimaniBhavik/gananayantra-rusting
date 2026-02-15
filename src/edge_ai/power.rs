/// Estimates IoT Battery Life for Edge AI devices.
///
/// # Arguments
/// * `battery_capacity_mah` - Battery capacity in mAh
/// * `active_current_ma` - Current draw when active (inference/transmission) in mA
/// * `sleep_current_ma` - Current draw when sleeping in mA
/// * `duty_cycle` - Fraction of time active (0.0 to 1.0)
///
/// # Returns
/// * Expected battery life in hours
pub fn iot_battery_life(
    battery_capacity_mah: f64,
    active_current_ma: f64,
    sleep_current_ma: f64,
    duty_cycle: f64
) -> Result<f64, String> {
    if duty_cycle < 0.0 || duty_cycle > 1.0 {
        return Err("Duty cycle must be between 0.0 and 1.0".into());
    }
    if battery_capacity_mah <= 0.0 {
        return Err("Battery capacity must be positive".into());
    }

    let avg_current = (active_current_ma * duty_cycle) + (sleep_current_ma * (1.0 - duty_cycle));

    if avg_current <= 0.0 {
        return Err("Average current draw must be positive".into());
    }

    Ok(battery_capacity_mah / avg_current)
}
