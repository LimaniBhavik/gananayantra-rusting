use crate::calculators::utils::read_input;
use chrono::{Local, NaiveDate, Datelike};

pub fn run_menu() {
    loop {
        println!("\n--- Date and Time Calculators ---");
        println!("1. Age Calculator");
        println!("2. Date Duration (Day Counter)");
        println!("3. Time Unit Converter");
        println!("4. Day of the Week");
        println!("0. Back");
        let choice = read_input("Select an option: ");

        match choice as i32 {
            1 => age_calc(),
            2 => day_counter(),
            3 => time_converter(),
            4 => day_of_week(),
            0 => break,
            _ => println!("Invalid choice."),
        }
    }
}

fn age_calc() {
    let y = read_input("Birth Year: ") as i32;
    let m = read_input("Birth Month (1-12): ") as u32;
    let d = read_input("Birth Day: ") as u32;
    let birth = NaiveDate::from_ymd_opt(y, m, d).unwrap();
    let now = Local::now().date_naive();
    let years = now.year() - birth.year();
    println!("Age: approximately {} years", years);
}

fn day_counter() {
    println!("Enter Start Date:");
    let y1 = read_input("Year: ") as i32;
    let m1 = read_input("Month: ") as u32;
    let d1 = read_input("Day: ") as u32;
    println!("Enter End Date:");
    let y2 = read_input("Year: ") as i32;
    let m2 = read_input("Month: ") as u32;
    let d2 = read_input("Day: ") as u32;
    
    let d1 = NaiveDate::from_ymd_opt(y1, m1, d1).unwrap();
    let d2 = NaiveDate::from_ymd_opt(y2, m2, d2).unwrap();
    println!("Difference: {} days", (d2 - d1).num_days());
}

fn time_converter() {
    let hours = read_input("Hours: ");
    println!("Minutes: {:.2}", hours * 60.0);
    println!("Seconds: {:.2}", hours * 3600.0);
}

fn day_of_week() {
    let y = read_input("Year: ") as i32;
    let m = read_input("Month: ") as u32;
    let d = read_input("Day: ") as u32;
    let date = NaiveDate::from_ymd_opt(y, m, d).unwrap();
    println!("Day of the week: {:?}", date.weekday());
}
