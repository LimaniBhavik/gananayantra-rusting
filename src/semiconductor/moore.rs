/// Predicts future transistor count based on Moore's Law (doubling every 2 years).
///
/// # Arguments
/// * `current_count` - Current number of transistors
/// * `years_future` - Years into the future
///
/// # Returns
/// * Projected transistor count
pub fn predict_transistor_count(current_count: f64, years_future: f64) -> Result<f64, String> {
    if current_count <= 0.0 {
        return Err("Current count must be positive".into());
    }
    // N = N0 * 2^(t/2)
    Ok(current_count * 2.0_f64.powf(years_future / 2.0))
}

/// Calculates Transistor Density (MTr/mm²).
///
/// # Arguments
/// * `transistor_count` - Total transistors
/// * `area_mm2` - Chip area in mm²
///
/// # Returns
/// * Density in Million Transistors per mm²
pub fn calculate_transistor_density(transistor_count: f64, area_mm2: f64) -> Result<f64, String> {
    if area_mm2 <= 0.0 {
        return Err("Area must be positive".into());
    }
    Ok((transistor_count / 1_000_000.0) / area_mm2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moores_law() {
        // Double in 2 years
        let future = predict_transistor_count(1000.0, 2.0).unwrap();
        assert_eq!(future, 2000.0);
    }
}
