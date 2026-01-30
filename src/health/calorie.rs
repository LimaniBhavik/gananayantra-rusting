/// Calculates Daily Calorie Requirements based on BMR and Activity Multiplier.
///
/// # Arguments
/// * `bmr` - Basal Metabolic Rate in kcal/day
/// * `activity_multiplier` - Activity level multiplier (e.g., 1.2 for Sedentary, 1.9 for Extra Active)
///
/// # Returns
/// * Daily maintenance calories as f64
pub fn daily_calorie_requirement(bmr: f64, activity_multiplier: f64) -> Result<f64, String> {
    if bmr <= 0.0 {
        return Err("BMR must be a positive value".into());
    }
    if !(1.0..=2.5).contains(&activity_multiplier) {
        return Err("Activity multiplier must be between 1.0 and 2.5".into());
    }

    Ok(bmr * activity_multiplier)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calorie_requirement() {
        // Example: BMR 1673.75, Moderate Active 1.55
        let result = daily_calorie_requirement(1673.75, 1.55).unwrap();
        assert!((result - 2594.31).abs() < 0.01);
    }

    #[test]
    fn test_invalid_input() {
        assert!(daily_calorie_requirement(0.0, 1.55).is_err());
        assert!(daily_calorie_requirement(1600.0, 0.5).is_err());
        assert!(daily_calorie_requirement(1600.0, 3.0).is_err());
    }
}
