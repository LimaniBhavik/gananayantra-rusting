/// Calculates Basal Metabolic Rate (BMR) using the Mifflin-St Jeor Equation.
///
/// # Arguments
/// * `weight_kg` - Weight in kilograms
/// * `height_cm` - Height in centimeters
/// * `age_years` - Age in years
/// * `is_male` - True for male, false for female
///
/// # Returns
/// * BMR value in kcal/day as f64
pub fn calculate_bmr(
    weight_kg: f64,
    height_cm: f64,
    age_years: u32,
    is_male: bool,
) -> Result<f64, String> {
    if weight_kg <= 0.0 || height_cm <= 0.0 || age_years == 0 {
        return Err("Weight, height, and age must be positive values".into());
    }

    // Mifflin-St Jeor Formula
    let bmr = (10.0 * weight_kg) + (6.25 * height_cm) - (5.0 * age_years as f64);
    
    if is_male {
        Ok(bmr + 5.0)
    } else {
        Ok(bmr - 161.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bmr_male() {
        // Example: 70kg, 175cm, 25 years old, Male
        let result = calculate_bmr(70.0, 175.0, 25, true).unwrap();
        // (10 * 70) + (6.25 * 175) - (5 * 25) + 5 = 700 + 1093.75 - 125 + 5 = 1673.75
        assert!((result - 1673.75).abs() < 0.01);
    }

    #[test]
    fn test_bmr_female() {
        // Example: 60kg, 165cm, 30 years old, Female
        let result = calculate_bmr(60.0, 165.0, 30, false).unwrap();
        // (10 * 60) + (6.25 * 165) - (5 * 30) - 161 = 600 + 1031.25 - 150 - 161 = 1320.25
        assert!((result - 1320.25).abs() < 0.01);
    }

    #[test]
    fn test_invalid_input() {
        assert!(calculate_bmr(0.0, 175.0, 25, true).is_err());
        assert!(calculate_bmr(70.0, 0.0, 25, true).is_err());
        assert!(calculate_bmr(70.0, 175.0, 0, true).is_err());
    }
}
