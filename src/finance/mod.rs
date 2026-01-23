pub mod roi;
pub mod tvm;
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
        println!("3. Financial (Loan)");
        println!("4. Investment Industry");
        println!("5. Retirement Industry");
        println!("6. Tax and Salary Industry");
        println!("7. SEBI Investor Tools");
        println!("8. Advertising (CPM)");
        println!("9. E-Commerce");
        println!("0. Back");

        let choice = read_input("Select an option: ");
        match choice as i32 {
            1 => roi::run(),
            2 => tvm::run(),
            3 => financial::run_menu(),
            4 => investment_ind::run_menu(),
            5 => retirement_ind::run_menu(),
            6 => tax_salary::run_menu(),
            7 => sebi::run_menu(),
            8 => advertising::run(),
            9 => ecommerce::run(),
            0 => break,
            _ => println!("Invalid choice."),
        }
    }
}
