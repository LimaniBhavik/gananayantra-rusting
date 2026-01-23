pub mod internet;
pub mod utility;
pub mod weather;
pub mod transport;
pub mod entertainment;

use crate::calculators::utils::read_input;

pub fn run_menu() {
    loop {
        println!("\n--- Utility & Lifestyle ---");
        println!("1. Internet Tools");
        println!("2. Everyday Utility");
        println!("3. Weather");
        println!("4. Transportation");
        println!("5. Entertainment");
        println!("0. Back");

        let choice = read_input("Select an option: ");
        match choice as i32 {
            1 => internet::run_menu(),
            2 => utility::run_menu(),
            3 => weather::run_menu(),
            4 => transport::run_menu(),
            5 => entertainment::run_menu(),
            0 => break,
            _ => println!("Invalid choice."),
        }
    }
}
