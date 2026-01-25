pub mod orbital_velocity;
pub mod escape_velocity;

use crate::calculators::utils::read_input;

pub fn run_menu() {
    loop {
        println!("\n--- Space ---");
        println!("1. Orbital Velocity Calculator");
        println!("2. Escape Velocity Calculator");
        println!("0. Back");

        let choice = read_input("Select an option: ");
        match choice as i32 {
            1 => orbital_velocity::run(),
            2 => escape_velocity::run(),
            0 => break,
            _ => println!("Invalid choice."),
        }
    }
}
