pub mod roi;
pub mod financial;
pub mod investment_ind;
pub mod retirement_ind;
pub mod tax_salary;
pub mod sebi;
pub mod advertising;
pub mod ecommerce;

use crate::calculators::utils::read_input;

pub fn run_menu() {
    loop {
        println!("\n--- Finance & Investment ---");
        println!("1. ROI Calculator");
        println!("2. Financial (Loan)");
        println!("3. Investment Industry");
        println!("4. Retirement Industry");
        println!("5. Tax and Salary Industry");
        println!("6. SEBI Investor Tools");
        println!("7. Advertising (CPM)");
        println!("8. E-Commerce");
        println!("0. Back");

        let choice = read_input("Select an option: ");
        match choice as i32 {
            1 => roi::run(),
            2 => financial::run_menu(),
            3 => investment_ind::run_menu(),
            4 => retirement_ind::run_menu(),
            5 => tax_salary::run_menu(),
            6 => sebi::run_menu(),
            7 => advertising::run(),
            8 => ecommerce::run(),
            0 => break,
            _ => println!("Invalid choice."),
        }
    }
}
