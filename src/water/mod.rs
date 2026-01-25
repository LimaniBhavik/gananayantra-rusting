pub mod pressure_at_depth;

use crate::calculators::utils::read_input;

pub fn run_menu() {
    loop {
        println!("\n--- Water ---");
        println!("1. Water Pressure at Depth Calculator");
        println!("0. Back");

        let choice = read_input("Select an option: ");
        match choice as i32 {
            1 => pressure_at_depth::run(),
            0 => break,
            _ => println!("Invalid choice."),
        }
    }
}
