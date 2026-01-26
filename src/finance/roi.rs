/// Calculates Return on Investment (ROI).
///
/// # Arguments
/// * `initial_investment` - The initial amount invested
/// * `final_value` - The final value after investment
///
/// # Returns
/// * ROI percentage as f64
pub fn roi(initial_investment: f64, final_value: f64) -> Result<f64, String> {
    if initial_investment <= 0.0 {
        return Err("Initial investment must be greater than zero".into());
    }

    Ok(((final_value - initial_investment) / initial_investment) * 100.0)
}

pub fn run() {
    println!("\n--- ROI Calculator ---");
    use crate::calculators::utils::read_input;
    let initial = read_input("Enter initial investment: ");
    let final_value = read_input("Enter final value: ");

    match roi(initial, final_value) {
        Ok(roi_val) => println!("ROI Result: {:.2}%", roi_val),
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roi() {
        let r = roi(1000.0, 1200.0).unwrap();
        assert_eq!(r, 20.0);
    }

    #[test]
    fn test_invalid_initial() {
        assert!(roi(0.0, 1200.0).is_err());
    }
}
