/// Calculates river discharge (flow rate).
///
/// # Arguments
/// * `cross_section_area_m2` - Cross-sectional area in square meters (m²)
/// * `velocity_m_per_s` - Average flow velocity in meters per second (m/s)
///
/// # Returns
/// * Discharge in cubic meters per second (m³/s)
pub fn river_flow_rate(cross_section_area_m2: f64, velocity_m_per_s: f64) -> Result<f64, String> {
    if cross_section_area_m2 <= 0.0 {
        return Err("Cross-sectional area must be greater than zero".into());
    }
    if velocity_m_per_s < 0.0 {
        return Err("Velocity cannot be negative".into());
    }

    Ok(cross_section_area_m2 * velocity_m_per_s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_river_flow_rate() {
        let q = river_flow_rate(50.0, 2.5).unwrap();
        assert_eq!(q, 125.0);
    }

    #[test]
    fn test_invalid_area() {
        assert!(river_flow_rate(0.0, 2.0).is_err());
    }
}
