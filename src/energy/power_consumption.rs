/// Calculates energy consumption in kilowatt-hours (kWh).
///
/// # Arguments
/// * `power_kw` - Power rating in kilowatts (kW)
/// * `hours` - Usage duration in hours
///
/// # Returns
/// * Energy consumed in kWh
pub fn energy_consumption(
    power_kw: f64,
    hours: f64,
) -> Result<f64, String> {
    if power_kw < 0.0 {
        return Err("Power cannot be negative".into());
    }
    if hours < 0.0 {
        return Err("Hours cannot be negative".into());
    }

    Ok(power_kw * hours)
}

pub fn run() {
    println!("\n--- Power Consumption Calculator ---");
    use crate::calculators::utils::read_input;
    let power_kw = read_input("Enter power in kilowatts (kW): ");
    let hours = read_input("Enter time in hours: ");

    match energy_consumption(power_kw, hours) {
        Ok(kwh) => println!("Total Energy Consumed: {:.2} kWh", kwh),
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_energy_consumption() {
        let kwh = energy_consumption(1.5, 8.0).unwrap();
        assert_eq!(kwh, 12.0);
    }

    #[test]
    fn test_invalid_input() {
        assert!(energy_consumption(-1.0, 5.0).is_err());
    }
}
