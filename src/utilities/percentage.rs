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

pub fn run() {
    println!("\n--- Percentage Calculator ---");
    use crate::calculators::utils::read_input;
    let value = read_input("Enter base value: ");
    let percent = read_input("Enter percentage (e.g., 15 for 15%): ");

    match percentage_of(value, percent) {
        Ok(result) => println!("{:.2}% of {:.2} is: {:.2}", percent, value, result),
        Err(e) => println!("Error: {}", e),
    }
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
