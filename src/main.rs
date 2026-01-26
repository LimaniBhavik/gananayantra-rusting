use gananayantra::health;
use gananayantra::finance;
use gananayantra::math;
use gananayantra::auto;
use gananayantra::finance_investment;
use gananayantra::energy;
use gananayantra::geo;
use gananayantra::water;
use gananayantra::space;
use gananayantra::utilities;
use gananayantra::specialized;
use gananayantra::utility_lifestyle;

use std::io::{self, Write};

fn main() {
    loop {
        println!("\n=== Gananayantra-Rusting Calculator ===");
        println!("1. Fitness & Health");
        println!("2. Finance & Investment");
        println!("3. Math");
        println!("4. Advertising (CPM)");
        println!("5. E-Commerce");
        println!("6. Auto Industry");
        println!("7. Investment Industry (Internal)");
        println!("8. Retirement Industry (Internal)");
        println!("9. Tax and Salary Industry (Internal)");
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
        println!("24. Network & CCTV Tools (Internal)");
        println!("25. Finance & Investment (New)");
        println!("26. Energy");
        println!("27. Geo");
        println!("28. Water");
        println!("29. Space");
        println!("30. Utilities (New)");
        println!("31. Specialized Industries (Root)");
        println!("32. Utility & Lifestyle (Root)");
        println!("0. Exit");
        print!("Select a category: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => health::run_menu(),
            "2" => finance::run_menu(),
            "3" => math::run_menu(),
            "4" => finance::advertising::run(),
            "5" => finance::ecommerce::run(),
            "6" => auto::run_menu(),
            "7" => finance::investment_ind::run_menu(),
            "8" => finance::retirement_ind::run_menu(),
            "9" => finance::tax_salary::run_menu(),
            "10" => health::health_expanded::run_menu(),
            "11" => math::advanced_math::run_menu(),
            "12" => math::statistics::run_menu(),
            "13" => math::geometry::run_menu(),
            "14" => specialized::date_time::run_menu(),
            "15" => specialized::building::run_menu(),
            "16" => specialized::science::run_menu(),
            "17" => specialized::electronics::run_menu(),
            "18" => utility_lifestyle::internet::run_menu(),
            "19" => utility_lifestyle::utility::run_menu(),
            "20" => utility_lifestyle::weather::run_menu(),
            "21" => utility_lifestyle::transport::run_menu(),
            "22" => utility_lifestyle::entertainment::run_menu(),
            "23" => finance::sebi::run_menu(),
            "24" => specialized::network_cctv::run_menu(),
            "25" => finance_investment::run_menu(),
            "26" => energy::run_menu(),
            "27" => geo::run_menu(),
            "28" => water::run_menu(),
            "29" => space::run_menu(),
            "30" => utilities::run_menu(),
            "31" => specialized::run_menu(),
            "32" => utility_lifestyle::run_menu(),
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
            "1" => health::bmi::run(),
            "2" => health::calorie::run(),
            "3" => health::body_fat::run(),
            "4" => health::bmr::run(),
            "5" => health::ideal_weight::run(),
            "6" => health::pace::run(),
            "7" => health::pregnancy::run(),
            "8" => health::conception::run(),
            "9" => health::due_date::run(),
            "0" => break,
            _ => println!("Invalid choice."),
        }
    }
}
