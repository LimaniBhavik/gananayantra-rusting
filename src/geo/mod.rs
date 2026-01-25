pub mod earth_distance;
pub mod horizon_distance;

use crate::calculators::utils::read_input;

pub fn run_menu() {
    loop {
        println!("\n--- Geo ---");
        println!("1. Earth Distance (Haversine) Calculator");
        println!("2. Horizon Distance (Earth Curvature) Calculator");
        println!("0. Back");

        let choice = read_input("Select an option: ");
        match choice as i32 {
            1 => earth_distance::run(),
            2 => horizon_distance::run(),
            0 => break,
            _ => println!("Invalid choice."),
        }
    }
}
