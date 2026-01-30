/// Calculates Body Fat Percentage using the U.S. Navy Method.
///
/// # Arguments
/// * `waist_cm` - Waist circumference in centimeters
/// * `neck_cm` - Neck circumference in centimeters
/// * `height_cm` - Height in centimeters
/// * `hip_cm` - Hip circumference in centimeters (required for females, ignored for males)
/// * `is_male` - True for male, false for female
///
/// # Returns
/// * Body fat percentage as f64
pub fn body_fat_percentage(
    waist_cm: f64,
    neck_cm: f64,
    height_cm: f64,
    hip_cm: Option<f64>,
    is_male: bool,
) -> Result<f64, String> {
    if waist_cm <= 0.0 || neck_cm <= 0.0 || height_cm <= 0.0 {
        return Err("All measurements must be positive values".into());
    }

    if is_male {
        // U.S. Navy Formula for Men
        // 495 / (1.0324 - 0.19077 * log10(waist - neck) + 0.15456 * log10(height)) - 450
        let diff = waist_cm - neck_cm;
        if diff <= 0.0 {
            return Err("Waist must be larger than neck".into());
        }
        let bf = 495.0 / (1.0324 - 0.19077 * diff.log10() + 0.15456 * height_cm.log10()) - 450.0;
        Ok(bf)
    } else {
        // U.S. Navy Formula for Women
        // 495 / (1.29579 - 0.35004 * log10(waist + hip - neck) + 0.22100 * log10(height)) - 450
        let hip = hip_cm.ok_or("Hip circumference is required for females")?;
        if hip <= 0.0 {
            return Err("Hip measurement must be positive".into());
        }
        let sum_diff = waist_cm + hip - neck_cm;
        if sum_diff <= 0.0 {
            return Err("Invalid waist/hip/neck measurements".into());
        }
        let bf =
            495.0 / (1.29579 - 0.35004 * sum_diff.log10() + 0.22100 * height_cm.log10()) - 450.0;
        Ok(bf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_body_fat_male() {
        // Example: Waist 90, Neck 38, Height 180
        let result = body_fat_percentage(90.0, 38.0, 180.0, None, true).unwrap();
        assert!(result > 0.0 && result < 50.0);
    }

    #[test]
    fn test_body_fat_female() {
        // Example: Waist 75, Neck 32, Hip 95, Height 165
        let result = body_fat_percentage(75.0, 32.0, 165.0, Some(95.0), false).unwrap();
        assert!(result > 0.0 && result < 50.0);
    }

    #[test]
    fn test_invalid_input() {
        assert!(body_fat_percentage(0.0, 38.0, 180.0, None, true).is_err());
        assert!(body_fat_percentage(90.0, 38.0, 180.0, None, false).is_err()); // Missing hip
    }
}
