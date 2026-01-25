pub fn river_flow_rate(
    cross_section_area_m2: f64,
    velocity_m_per_s: f64,
) -> Result<f64, String> {
    if cross_section_area_m2 <= 0.0 {
        return Err("Cross-sectional area must be greater than zero".to_string());
    }
    if velocity_m_per_s < 0.0 {
        return Err("Velocity cannot be negative".to_string());
    }

    Ok(cross_section_area_m2 * velocity_m_per_s)
}

pub fn run() {
    println!("\n--- River Flow Rate (Discharge) Calculator ---");
    use crate::calculators::utils::read_input;

    let area = read_input("Enter cross-sectional area (m²): ");
    let velocity = read_input("Enter average flow velocity (m/s): ");

    match river_flow_rate(area, velocity) {
        Ok(q) => println!("River Discharge (Flow Rate): {:.2} m³/s", q),
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_river_flow_rate() {
        let flow = river_flow_rate(50.0, 2.5).unwrap();
        assert_eq!(flow, 125.0);
    }
}
