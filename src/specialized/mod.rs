pub mod date_time;
pub mod building;
pub mod science;
pub mod electronics;
pub mod network_cctv;

use crate::calculators::utils::read_input;

pub fn run_menu() {
    loop {
        println!("\n--- Specialized Industries ---");
        println!("1. Date & Time");
        println!("2. Housing & Building");
        println!("3. Science & Measurements");
        println!("4. Electronics");
        println!("5. Network & CCTV Tools");
        println!("0. Back");

        let choice = read_input("Select an option: ");
        match choice as i32 {
            1 => date_time::run_menu(),
            2 => building::run_menu(),
            3 => science::run_menu(),
            4 => electronics::run_menu(),
            5 => network_cctv::run_menu(),
            0 => break,
            _ => println!("Invalid choice."),
        }
    }
}
