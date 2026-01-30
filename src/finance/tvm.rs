/// Calculates the future value of an investment using compound interest.
///
/// # Arguments
/// * `present_value` - Initial amount invested
/// * `annual_rate` - Annual interest rate (decimal, e.g. 0.08 for 8%)
/// * `years` - Number of years
///
/// # Returns
/// * Future value of the investment
pub fn future_value(
    present_value: f64,
    annual_rate: f64,
    years: u32,
) -> Result<f64, String> {
    if present_value <= 0.0 {
        return Err("Present value must be greater than zero".into());
    }
    if annual_rate < 0.0 {
        return Err("Annual rate cannot be negative".into());
    }

    Ok(present_value * (1.0 + annual_rate).powi(years as i32))
}

/// Calculates the present value needed to reach a future goal.
///
/// # Arguments
/// * `future_value` - Desired future amount
/// * `annual_rate` - Annual interest rate (decimal, e.g. 0.08 for 8%)
/// * `years` - Number of years
///
/// # Returns
/// * Present value required
pub fn present_value(
    future_value: f64,
    annual_rate: f64,
    years: u32,
) -> Result<f64, String> {
    if future_value <= 0.0 {
        return Err("Future value must be greater than zero".into());
    }
    if annual_rate < 0.0 {
        return Err("Annual rate cannot be negative".into());
    }

    Ok(future_value / (1.0 + annual_rate).powi(years as i32))
}

/// Calculates the Net Present Value (NPV) of a series of cash flows.
///
/// # Arguments
/// * `rate` - Discount rate (decimal, e.g. 0.08 for 8%)
/// * `cash_flows` - Slice of cash flows where index 0 is time 0, index 1 is time 1, etc.
///                  Typically cash_flows[0] is the initial investment (negative).
///
/// # Returns
/// * Net Present Value
pub fn net_present_value(
    rate: f64,
    cash_flows: &[f64],
) -> Result<f64, String> {
    if rate < 0.0 {
        return Err("Discount rate cannot be negative".into());
    }

    let mut npv = 0.0;
    for (t, &flow) in cash_flows.iter().enumerate() {
        npv += flow / (1.0 + rate).powi(t as i32);
    }
    Ok(npv)
}

pub fn run() {
    println!("\n--- TVM Calculator ---");
    use crate::calculators::utils::read_input;

    println!("1. Future Value");
    println!("2. Present Value");
    println!("3. Net Present Value (NPV)");

    let choice = read_input("Select an option: ");

    match choice as i32 {
        1 => {
            let pv = read_input("Enter Present Value: ");
            let rate = read_input("Enter Annual Interest Rate (e.g., 0.08 for 8%): ");
            let years = read_input("Enter Number of Years: ") as u32;

            match future_value(pv, rate, years) {
                Ok(fv) => println!("Future Value Result: {:.2}", fv),
                Err(e) => println!("Error: {}", e),
            }
        }
        2 => {
            let fv = read_input("Enter Future Value: ");
            let rate = read_input("Enter Annual Interest Rate (e.g., 0.08 for 8%): ");
            let years = read_input("Enter Number of Years: ") as u32;

            match present_value(fv, rate, years) {
                Ok(pv) => println!("Present Value Result: {:.2}", pv),
                Err(e) => println!("Error: {}", e),
            }
        }
        3 => {
            let rate = read_input("Enter Discount Rate (e.g., 0.10 for 10%): ");
            let count = read_input("How many cash flows? (including initial investment): ") as usize;
            let mut flows = Vec::with_capacity(count);
            println!("Enter cash flows in order (Time 0, Time 1, ...):");
            for i in 0..count {
                let val = read_input(&format!("Cash Flow at T={}: ", i));
                flows.push(val);
            }
            match net_present_value(rate, &flows) {
                Ok(npv) => println!("Net Present Value: {:.2}", npv),
                Err(e) => println!("Error: {}", e),
            }
        }
        _ => println!("Invalid choice"),
    }
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

    #[test]
    fn test_present_value() {
        let pv = present_value(14693.28, 0.08, 5).unwrap();
        assert!((pv - 10_000.0).abs() < 0.1);
    }

    #[test]
    fn test_invalid_present_value_inputs() {
        assert!(present_value(0.0, 0.08, 5).is_err());
        assert!(present_value(100.0, -0.1, 5).is_err());
    }

    #[test]
    fn test_net_present_value() {
        let cash_flows = vec![-1000.0, 100.0, 100.0, 1100.0];
        let npv = net_present_value(0.10, &cash_flows).unwrap();
        // Calculation: -1000 + 100/1.1 + 100/1.21 + 1100/1.331
        // = -1000 + 90.909 + 82.645 + 826.446
        // = -0.000... ~ 0.0
        assert!(npv.abs() < 1.0);
    }

    #[test]
    fn test_net_present_value_positive() {
        let cash_flows = vec![-1000.0, 500.0, 500.0, 500.0];
        let npv = net_present_value(0.10, &cash_flows).unwrap();
        // -1000 + 500/1.1 + 500/1.21 + 500/1.331
        // -1000 + 454.54 + 413.22 + 375.65 = 243.41
        assert!((npv - 243.4).abs() < 0.1);
    }

    #[test]
    fn test_invalid_net_present_value() {
        let cash_flows = vec![-100.0, 10.0];
        assert!(net_present_value(-0.5, &cash_flows).is_err());
    }
}
