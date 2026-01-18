mod calculators;

use std::io::{self, Write};

fn main() {
    loop {
        println!("\n=== Gananayantra-Rusting Calculator ===");
        println!("1. Fitness & Health");
        println!("2. Financial (Loan)");
        println!("3. Math");
        println!("4. Advertising (CPM)");
        println!("5. E-Commerce");
        println!("6. Auto Industry");
        println!("7. Investment Industry");
        println!("0. Exit");
        print!("Select a category: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => health_menu(),
            "2" => calculators::financial::run_menu(),
            "3" => calculators::math::run(),
            "4" => calculators::advertising::run(),
            "5" => calculators::ecommerce::run(),
            "6" => calculators::auto::run_menu(),
            "7" => calculators::investment_ind::run_menu(),
            "0" => break,
            _ => println!("Invalid choice."),
        }
    }
}

fn health_menu() {
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
        println!("0. Back");
        print!("Select an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => calculators::bmi::run(),
            "2" => calculators::calorie::run(),
            "3" => calculators::body_fat::run(),
            "4" => calculators::bmr::run(),
            "5" => calculators::ideal_weight::run(),
            "6" => calculators::pace::run(),
            "7" => calculators::pregnancy::run(),
            "8" => calculators::conception::run(),
            "9" => calculators::due_date::run(),
            "0" => break,
            _ => println!("Invalid choice."),
        }
    }
}
