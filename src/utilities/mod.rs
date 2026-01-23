pub mod percentage;

use crate::calculators::utils::read_input;

pub fn run_menu() {
    loop {
        println!("\n--- Utilities (New) ---");
        println!("1. Percentage Calculator");
        println!("0. Back");

        let choice = read_input("Select an option: ");
        match choice as i32 {
            1 => percentage::run(),
            0 => break,
            _ => println!("Invalid choice."),
        }
    }
}
