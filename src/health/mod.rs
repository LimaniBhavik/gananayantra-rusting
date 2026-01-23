pub mod bmi;
pub mod calorie;
pub mod body_fat;
pub mod bmr;
pub mod ideal_weight;
pub mod pace;
pub mod pregnancy;
pub mod conception;
pub mod due_date;
pub mod health_expanded;

use crate::calculators::utils::read_input;
use std::io::{self, Write};

pub fn run_menu() {
    loop {
        println!("\n--- Fitness & Health ---");
        println!("1. BMI Calculator");
        println!("2. Calorie Calculator");
        println!("3. Body Fat Calculator");
        println!("4. BMR Calculator");
        println!("5. Ideal Weight Calculator");
        println!("6. Pace Calculator");
        println!("7. Pregnancy Calculator");
        println!("8. Pregnancy Conception Calculator");
        println!("9. Due Date Calculator");
        println!("10. Fitness & Health (Expanded)");
        println!("0. Back");
        print!("Select an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => bmi::run(),
            "2" => calorie::run(),
            "3" => body_fat::run(),
            "4" => bmr::run(),
            "5" => ideal_weight::run(),
            "6" => pace::run(),
            "7" => pregnancy::run(),
            "8" => conception::run(),
            "9" => due_date::run(),
            "10" => health_expanded::run_menu(),
            "0" => break,
            _ => println!("Invalid choice."),
        }
    }
}
