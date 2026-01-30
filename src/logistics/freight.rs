/// Calculates the volumetric weight of a shipment.
///
/// # Arguments
/// * `length` - Length in cm
/// * `width` - Width in cm
/// * `height` - Height in cm
/// * `divisor` - Volumetric factor (Standard: 5000 for Air/Courier, 6000 often for others)
///
/// # Returns
/// * Volumetric Weight in kg
pub fn calculate_volumetric_weight(
    length: f64,
    width: f64,
    height: f64,
    divisor: f64,
) -> Result<f64, String> {
    if length <= 0.0 || width <= 0.0 || height <= 0.0 || divisor <= 0.0 {
        return Err("Dimensions and divisor must be greater than zero".into());
    }
    Ok((length * width * height) / divisor)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_volumetric_weight() {
        // 50x40x30 box, divisor 5000 = 60000 / 5000 = 12kg
        assert_eq!(
            calculate_volumetric_weight(50.0, 40.0, 30.0, 5000.0).unwrap(),
            12.0
        );
    }
}
