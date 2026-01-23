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
        println!("1. Financial (Loan)");
        println!("2. Investment Industry");
        println!("3. Retirement Industry");
        println!("4. Tax and Salary Industry");
        println!("5. SEBI Investor Tools");
        println!("6. Advertising (CPM)");
        println!("7. E-Commerce");
        println!("0. Back");

        let choice = read_input("Select an option: ");
        match choice as i32 {
            1 => financial::run_menu(),
            2 => investment_ind::run_menu(),
            3 => retirement_ind::run_menu(),
            4 => tax_salary::run_menu(),
            5 => sebi::run_menu(),
            6 => advertising::run(),
            7 => ecommerce::run(),
            0 => break,
            _ => println!("Invalid choice."),
        }
    }
}
