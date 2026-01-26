/// Calculates the future value of an investment.
///
/// # Arguments
/// * `present_value` - Initial amount
/// * `annual_rate` - Annual interest rate (decimal, e.g., 0.08 for 8%)
/// * `years` - Number of years
pub fn future_value(
    present_value: f64,
    annual_rate: f64,
    years: u32,
) -> Result<f64, String> {
    if present_value <= 0.0 {
        return Err("Present value must be greater than zero".to_string());
    }
    if annual_rate < 0.0 {
        return Err("Interest rate cannot be negative".to_string());
    }

    let fv = present_value * (1.0 + annual_rate).powi(years as i32);
    Ok(fv)
}

pub fn run() {
    println!("\n--- TVM (Future Value) Calculator ---");
    use crate::calculators::utils::read_input;
    
    let pv = read_input("Enter Present Value: ");
    let rate = read_input("Enter Annual Interest Rate (e.g., 0.08 for 8%): ");
    let years = read_input("Enter Number of Years: ") as u32;

    match future_value(pv, rate, years) {
        Ok(fv) => println!("Future Value Result: {:.2}", fv),
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_future_value() {
        let fv = future_value(10000.0, 0.08, 5).unwrap();
        assert!((fv - 14693.28).abs() < 0.01);
    }
}
