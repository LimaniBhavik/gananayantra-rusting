/// Calculates the future value of an investment using compound interest.
///
/// # Arguments
/// * `present_value` - Initial amount invested
/// * `annual_rate` - Annual interest rate (decimal, e.g. 0.08 for 8%)
/// * `years` - Number of years
///
/// # Returns
/// * Future value of the investment
pub fn future_value(present_value: f64, annual_rate: f64, years: u32) -> Result<f64, String> {
    if present_value <= 0.0 {
        return Err("Present value must be greater than zero".into());
    }
    if annual_rate < 0.0 {
        return Err("Annual rate cannot be negative".into());
    }

    Ok(present_value * (1.0 + annual_rate).powi(years as i32))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_future_value() {
        let fv = future_value(10_000.0, 0.08, 5).unwrap();
        assert!((fv - 14693.28).abs() < 0.1);
    }

    #[test]
    fn test_invalid_present_value() {
        assert!(future_value(0.0, 0.08, 5).is_err());
    }
}
