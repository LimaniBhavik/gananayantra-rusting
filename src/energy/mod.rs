pub mod power_consumption;
pub mod electricity_cost;

use crate::calculators::utils::read_input;

pub fn run_menu() {
    loop {
        println!("\n--- Energy ---");
        println!("1. Power Consumption Calculator");
        println!("2. Electricity Cost Calculator");
        println!("0. Back");

        let choice = read_input("Select an option: ");
        match choice as i32 {
            1 => power_consumption::run(),
            2 => electricity_cost::run(),
            0 => break,
            _ => println!("Invalid choice."),
        }
    }
}
