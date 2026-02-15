/// Pure logic for calculating Return on Investment (ROI)
/// ROI = ((Final Value - Initial Investment) / Initial Investment) * 100
pub fn calculate_roi(initial_investment: f64, final_value: f64) -> Result<f64, String> {
    if initial_investment <= 0.0 {
        return Err("Initial investment must be greater than zero.".to_string());
    }
    if final_value < 0.0 {
        return Err("Final value cannot be negative.".to_string());
    }

    Ok(((final_value - initial_investment) / initial_investment) * 100.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_roi_valid() {
        assert_eq!(calculate_roi(100.0, 150.0).unwrap(), 50.0);
        assert_eq!(calculate_roi(100.0, 50.0).unwrap(), -50.0);
    }

    #[test]
    fn test_calculate_roi_invalid() {
        assert!(calculate_roi(0.0, 150.0).is_err());
        assert!(calculate_roi(-10.0, 150.0).is_err());
        assert!(calculate_roi(100.0, -10.0).is_err());
    }
}
