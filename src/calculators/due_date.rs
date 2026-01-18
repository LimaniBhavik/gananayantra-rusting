use crate::calculators::utils::read_string;
use chrono::{NaiveDate, Duration};

pub fn run() {
    println!("\n--- Due Date Calculator (Naegele's Rule) ---");
    let lmp_str = read_string("Enter first day of last menstrual period (YYYY-MM-DD): ");
    
    if let Ok(lmp) = NaiveDate::parse_from_str(&lmp_str, "%Y-%m-%d") {
        let due_date = lmp + Duration::days(280);
        println!("Your estimated due date is: {}", due_date.format("%A, %B %d, %Y"));
    } else {
        println!("Invalid date format. Please use YYYY-MM-DD.");
    }
}
