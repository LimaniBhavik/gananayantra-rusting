use crate::calculators::utils::read_input;

pub fn run_menu() {
    loop {
        println!("\n--- Investment Industry Calculators ---");
        println!("1. Interest Calculator");
        println!("2. Investment Calculator");
        println!("3. Finance Calculator");
        println!("4. Compound Interest Calculator");
        println!("5. Interest Rate Calculator");
        println!("0. Back");
        let choice = read_input("Select an option: ");

        match choice as i32 {
            1 => interest_calculator(),
            2 => investment_calculator(),
            3 => finance_calculator(),
            4 => compound_interest_calculator(),
            5 => interest_rate_calculator(),
            0 => break,
            _ => println!("Invalid choice."),
        }
    }
}

fn interest_calculator() {
    println!("\n--- Interest Calculator ---");
    let principal = read_input("Principal Amount: ");
    let rate = read_input("Annual Interest Rate (%): ");
    let time = read_input("Time (years): ");
    
    let interest = (principal * rate * time) / 100.0;
    println!("Simple Interest: {:.2}", interest);
    println!("Total Amount: {:.2}", principal + interest);
}

fn investment_calculator() {
    println!("\n--- Investment Calculator ---");
    let starting_amount = read_input("Starting Amount: ");
    let rate = read_input("Annual Return Rate (%): ");
    let years = read_input("Years to Invest: ");
    let additional_contribution = read_input("Monthly Contribution: ");

    let mut balance = starting_amount;
    let monthly_rate = (rate / 100.0) / 12.0;
    let months = years * 12.0;

    for _ in 0..(months as i32) {
        balance += additional_contribution;
        balance *= 1.0 + monthly_rate;
    }

    println!("Total Investment Value: {:.2}", balance);
    println!("Total Contributions: {:.2}", starting_amount + (additional_contribution * months));
}

fn finance_calculator() {
    println!("\n--- Finance Calculator (NPV) ---");
    let initial_investment = read_input("Initial Investment: ");
    let discount_rate = read_input("Discount Rate (%): ");
    let years = read_input("Number of Years: ");

    let mut npv = -initial_investment;
    let r = discount_rate / 100.0;

    for t in 1..=(years as i32) {
        println!("Enter expected cash flow for Year {}: ", t);
        let cash_flow = read_input("> ");
        npv += cash_flow / (1.0 + r).powi(t);
    }

    println!("Net Present Value (NPV): {:.2}", npv);
}

fn compound_interest_calculator() {
    println!("\n--- Compound Interest Calculator ---");
    let principal = read_input("Principal Amount: ");
    let rate = read_input("Annual Interest Rate (%): ");
    let years = read_input("Number of Years: ");
    let times_compounded = read_input("Times compounded per year (e.g., 12 for monthly): ");

    let r = rate / 100.0;
    let n = times_compounded;
    let t = years;
    
    let amount = principal * (1.0 + r / n).powf(n * t);
    println!("Future Value: {:.2}", amount);
    println!("Total Interest Earned: {:.2}", amount - principal);
}

fn interest_rate_calculator() {
    println!("\n--- Interest Rate Calculator ---");
    let principal = read_input("Principal Amount: ");
    let interest_earned = read_input("Total Interest Earned: ");
    let time = read_input("Time (years): ");

    let rate = (interest_earned / (principal * time)) * 100.0;
    println!("Calculated Annual Simple Interest Rate: {:.2}%", rate);
}
