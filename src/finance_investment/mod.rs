use crate::calculators::utils::read_input;

pub fn run_menu() {
    loop {
        println!("\n--- Finance & Investment Calculators ---");
        println!("1. ROI Calculator");
        println!("0. Back");

        let choice = read_input("Select an option: ");
        match choice as i32 {
            1 => roi_calculator(),
            0 => break,
            _ => println!("Invalid choice."),
        }
    }
}

fn roi_calculator() {
    println!("\n--- ROI Calculator ---");
    let initial_investment = read_input("Enter initial investment: ");
    let final_value = read_input("Enter final value: ");

    match calculate_roi(initial_investment, final_value) {
        Ok(roi) => println!("ROI: {:.2}%", roi),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn calculate_roi(initial_investment: f64, final_value: f64) -> Result<f64, String> {
    if initial_investment <= 0.0 {
        return Err("Initial investment must be greater than 0.".to_string());
    }
    if final_value < 0.0 {
        return Err("Final value cannot be negative.".to_string());
    }

    let roi_percentage = ((final_value - initial_investment) / initial_investment) * 100.0;
    Ok(roi_percentage)
}
