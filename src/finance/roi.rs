use crate::calculators::utils::read_input;

pub fn calculate_roi(initial: f64, final_value: f64) -> Result<f64, String> {
    if initial <= 0.0 {
        return Err("Initial investment must be greater than zero".to_string());
    }

    Ok(((final_value - initial) / initial) * 100.0)
}

pub fn run() {
    println!("\n--- ROI Calculator ---");
    let initial = read_input("Enter initial investment: ");
    let final_value = read_input("Enter final value: ");

    match calculate_roi(initial, final_value) {
        Ok(roi) => println!("ROI Result: {:.2}%", roi),
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roi() {
        let result = calculate_roi(1000.0, 1200.0).unwrap();
        assert_eq!(result, 20.0);
    }
}
