pub fn ohms_law_voltage(current: f64, resistance: f64) -> f64 { current * resistance }
pub fn ohms_law_current(voltage: f64, resistance: f64) -> Result<f64, String> {
    if resistance == 0.0 { Err("Resistance cannot be zero".into()) } else { Ok(voltage / resistance) }
}
pub fn ohms_law_resistance(voltage: f64, current: f64) -> Result<f64, String> {
    if current == 0.0 { Err("Current cannot be zero".into()) } else { Ok(voltage / current) }
}

pub fn calculate_resistor_value(digit1: f64, digit2: f64, multiplier_pow10: f64) -> f64 {
    (digit1 * 10.0 + digit2) * 10.0_f64.powf(multiplier_pow10)
}

pub fn voltage_drop(current: f64, wire_resistance: f64) -> f64 {
    current * wire_resistance
}
