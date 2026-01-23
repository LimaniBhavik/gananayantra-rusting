use crate::calculators::utils::read_input;

pub fn run_menu() {
    loop {
        println!("\n--- Finance & Investment Calculators ---");
        println!("1. ROI Calculator");
        println!("0. Back");

        let choice = read_input("Select an option: ");
        match choice as i32 {
            1 => roi_cli(),
            0 => break,
            _ => println!("Invalid choice."),
        }
    }
}

/// CLI handler for the ROI calculator
fn roi_cli() {
    println!("\n--- ROI Calculator ---");
    let initial = read_input("Enter initial investment: ");
    let final_val = read_input("Enter final value: ");

    match calculate_roi(initial, final_val) {
        Ok(roi) => println!("ROI: {:.2}%", roi),
        Err(e) => println!("Error: {}", e),
    }
}

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
