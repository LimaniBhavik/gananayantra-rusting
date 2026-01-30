// Average factors (approximate global/US averages)
const CO2_KG_PER_KWH: f64 = 0.385; // ~0.85 lbs -> 0.385 kg (US Avg)
const CO2_KG_PER_GALLON_GAS: f64 = 8.887; // EPA standard

/// Calculates estimated carbon footprint from electricity usage.
///
/// # Arguments
/// * `kwh` - Electricity used in kilowatt-hours
///
/// # Returns
/// * Carbon footprint in kg CO2e
pub fn calculate_carbon_footprint_electricity(kwh: f64) -> Result<f64, String> {
    if kwh < 0.0 {
        return Err("Usage cannot be negative".into());
    }
    Ok(kwh * CO2_KG_PER_KWH)
}

/// Calculates estimated carbon footprint from gasoline usage.
///
/// # Arguments
/// * `gallons` - Fuel consumed in gallons
///
/// # Returns
/// * Carbon footprint in kg CO2e
pub fn calculate_carbon_footprint_fuel(gallons: f64) -> Result<f64, String> {
    if gallons < 0.0 {
        return Err("Fuel cannot be negative".into());
    }
    Ok(gallons * CO2_KG_PER_GALLON_GAS)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_electricity_carbon() {
        assert!((calculate_carbon_footprint_electricity(100.0).unwrap() - 38.5).abs() < 0.1);
    }
}
