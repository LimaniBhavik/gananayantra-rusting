pub mod math_basic;
pub mod advanced_math;
pub mod statistics;
pub mod geometry;

use crate::calculators::utils::read_input;

pub fn run_menu() {
    loop {
        println!("\n--- Math & Statistics ---");
        println!("1. Basic Math");
        println!("2. Advanced Math");
        println!("3. Statistics");
        println!("4. Geometry");
        println!("0. Back");

        let choice = read_input("Select an option: ");
        match choice as i32 {
            1 => math_basic::run(),
            2 => advanced_math::run_menu(),
            3 => statistics::run_menu(),
            4 => geometry::run_menu(),
            0 => break,
            _ => println!("Invalid choice."),
        }
    }
}
