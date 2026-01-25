const GRAVITY: f64 = 9.81;

pub fn pressure_at_depth(
    density: f64,
    depth_m: f64,
) -> Result<f64, String> {
    if density <= 0.0 {
        return Err("Density must be greater than zero".to_string());
    }
    if depth_m < 0.0 {
        return Err("Depth cannot be negative".to_string());
    }

    Ok(density * GRAVITY * depth_m)
}

pub fn run() {
    println!("\n--- Water Pressure at Depth Calculator ---");
    use crate::calculators::utils::read_input;

    let density = read_input("Enter fluid density (kg/m³, e.g., 1000 for freshwater): ");
    let depth = read_input("Enter depth in meters: ");

    match pressure_at_depth(density, depth) {
        Ok(pressure) => {
            println!("Hydrostatic Pressure: {:.2} Pa", pressure);
            println!("Pressure in kPa: {:.2} kPa", pressure / 1000.0);
        }
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pressure_at_depth() {
        let pressure = pressure_at_depth(1000.0, 10.0).unwrap();
        assert_eq!(pressure, 98100.0);
    }
}
