pub mod auto_internal;

use crate::calculators::utils::read_input;

pub fn run_menu() {
    loop {
        println!("\n--- Auto Industry ---");
        println!("1. Auto Calculators");
        println!("0. Back");

        let choice = read_input("Select an option: ");
        match choice as i32 {
            1 => auto_internal::run_menu(),
            0 => break,
            _ => println!("Invalid choice."),
        }
    }
}
