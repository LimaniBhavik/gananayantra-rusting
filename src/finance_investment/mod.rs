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
    if initial_investment <= 0.0 {
        println!("Error: Initial investment must be greater than 0.");
        return;
    }
    let final_value = read_input("Enter final value: ");
    if final_value < 0.0 {
        println!("Error: Final value cannot be negative.");
        return;
    }

    let roi_percentage = ((final_value - initial_investment) / initial_investment) * 100.0;
    println!("ROI: {:.2}%", roi_percentage);
}
