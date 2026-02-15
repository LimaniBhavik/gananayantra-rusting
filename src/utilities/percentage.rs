/// Calculates a percentage of a given value.
///
/// # Arguments
/// * `value` - Base value
/// * `percent` - Percentage to apply (e.g. 15 for 15%)
///
/// # Returns
/// * Calculated percentage value
pub fn percentage_of(value: f64, percent: f64) -> Result<f64, String> {
    if value < 0.0 {
        return Err("Value cannot be negative".into());
    }

    Ok((percent / 100.0) * value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_percentage_of() {
        let result = percentage_of(200.0, 15.0).unwrap();
        assert_eq!(result, 30.0);
    }

    #[test]
    fn test_invalid_value() {
        assert!(percentage_of(-100.0, 10.0).is_err());
    }
}
