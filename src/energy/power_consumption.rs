use crate::calculators::utils::read_input;

pub fn calculate_energy_consumption(
    power_kw: f64,
    hours: f64,
) -> Result<f64, String> {
    if power_kw < 0.0 {
        return Err("Power cannot be negative".to_string());
    }
    if hours < 0.0 {
        return Err("Hours cannot be negative".to_string());
    }

    Ok(power_kw * hours)
}

pub fn run() {
    println!("\n--- Power Consumption Calculator ---");
    let power_kw = read_input("Enter power in kilowatts (kW): ");
    let hours = read_input("Enter time in hours: ");

    match calculate_energy_consumption(power_kw, hours) {
        Ok(kwh) => println!("Total Energy Consumed: {:.2} kWh", kwh),
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_energy_consumption() {
        let result = calculate_energy_consumption(1.5, 8.0).unwrap();
        assert_eq!(result, 12.0);
    }
}
