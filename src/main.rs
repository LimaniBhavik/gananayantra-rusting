mod calculators;
mod finance_investment;

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
        println!("8. Retirement Industry");
        println!("9. Tax and Salary Industry");
        println!("10. Fitness & Health (Expanded)");
        println!("11. Advanced Math");
        println!("12. Statistics");
        println!("13. Geometry");
        println!("14. Date and Time");
        println!("15. Housing and Building");
        println!("16. Science and Measurements");
        println!("17. Electronics");
        println!("18. Internet Tools");
        println!("19. Everyday Utility");
        println!("20. Weather");
        println!("21. Transportation");
        println!("22. Entertainment");
        println!("23. SEBI Investor Tools");
        println!("24. Network & CCTV Tools");
        println!("25. Finance & Investment (New)");
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
            "8" => calculators::retirement_ind::run_menu(),
            "9" => calculators::tax_salary::run_menu(),
            "10" => calculators::health_expanded::run_menu(),
            "11" => calculators::advanced_math::run_menu(),
            "12" => calculators::statistics::run_menu(),
            "13" => calculators::geometry::run_menu(),
            "14" => calculators::date_time::run_menu(),
            "15" => calculators::building::run_menu(),
            "16" => calculators::science::run_menu(),
            "17" => calculators::electronics::run_menu(),
            "18" => calculators::internet::run_menu(),
            "19" => calculators::utility::run_menu(),
            "20" => calculators::weather::run_menu(),
            "21" => calculators::transport::run_menu(),
            "22" => calculators::entertainment::run_menu(),
            "23" => calculators::sebi::run_menu(),
            "24" => calculators::network_cctv::run_menu(),
            "25" => finance_investment::run_menu(),
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
