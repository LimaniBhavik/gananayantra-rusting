use crate::calculators::utils::read_string;
use chrono::{NaiveDate, Duration};

pub fn run() {
    println!("\n--- Pregnancy Conception Calculator ---");
    let date_str = read_string("Enter last menstrual period date (YYYY-MM-DD): ");
    
    if let Ok(lmp) = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
        let conception = lmp + Duration::days(14);
        println!("Estimated Conception Date: {}", conception.format("%Y-%m-%d"));
        println!("(Based on a standard 28-day cycle)");
    } else {
        println!("Invalid date format. Please use YYYY-MM-DD.");
    }
}
