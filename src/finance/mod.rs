//! Finance-related calculators including
//! ROI, time value of money, and compound interest.

pub mod roi;
pub mod tvm;
pub mod compound_interest;
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
        println!("2. TVM (Future Value) Calculator");
        println!("3. Compound Interest Calculator");
        println!("4. Financial (Loan)");
        println!("5. Investment Industry");
        println!("6. Retirement Industry");
        println!("7. Tax and Salary Industry");
        println!("8. SEBI Investor Tools");
        println!("9. Advertising (CPM)");
        println!("10. E-Commerce");
        println!("0. Back");

        let choice = read_input("Select an option: ");
        match choice as i32 {
            1 => roi::run(),
            2 => tvm::run(),
            3 => compound_interest::run(),
            4 => financial::run_menu(),
            5 => investment_ind::run_menu(),
            6 => retirement_ind::run_menu(),
            7 => tax_salary::run_menu(),
            8 => sebi::run_menu(),
            9 => advertising::run(),
            10 => ecommerce::run(),
            0 => break,
            _ => println!("Invalid choice."),
        }
    }
}
