/// Calculates the electricity cost based on energy consumption.
///
/// # Arguments
/// * `energy_kwh` - Energy consumed in kilowatt-hours (kWh)
/// * `cost_per_kwh` - Cost per kilowatt-hour
///
/// # Returns
/// * Total electricity cost
pub fn electricity_cost(
    energy_kwh: f64,
    cost_per_kwh: f64,
) -> Result<f64, String> {
    if energy_kwh < 0.0 {
        return Err("Energy consumption cannot be negative".into());
    }
    if cost_per_kwh < 0.0 {
        return Err("Cost per kWh cannot be negative".into());
    }

    Ok(energy_kwh * cost_per_kwh)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_electricity_cost() {
        let cost = electricity_cost(12.0, 6.0).unwrap();
        assert_eq!(cost, 72.0);
    }

    #[test]
    fn test_invalid_energy() {
        assert!(electricity_cost(-1.0, 6.0).is_err());
    }
}
