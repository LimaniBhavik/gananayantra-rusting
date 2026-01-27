/// Calculates Body Mass Index (BMI).
///
/// # Arguments
/// * `weight_kg` - Weight in kilograms
/// * `height_m` - Height in meters
///
/// # Returns
/// * BMI value as f64
pub fn bmi(weight_kg: f64, height_m: f64) -> Result<f64, String> {
    if weight_kg <= 0.0 || height_m <= 0.0 {
        return Err("Weight and height must be positive values".into());
    }

    Ok(weight_kg / (height_m * height_m))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bmi() {
        let result = bmi(70.0, 1.75).unwrap();
        assert!((result - 22.86).abs() < 0.01);
    }

    #[test]
    fn test_invalid_input() {
        assert!(bmi(0.0, 1.75).is_err());
        assert!(bmi(70.0, 0.0).is_err());
    }
}
