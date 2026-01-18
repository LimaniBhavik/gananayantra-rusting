use crate::calculators::utils::read_input;

pub fn run_menu() {
    loop {
        println!("\n--- Financial Calculators ---");
        println!("1. Business Loan Calculator");
        println!("2. Mortgage Calculator");
        println!("3. Auto Loan Calculator");
        println!("4. Interest Calculator (Simple)");
        println!("5. Compound Interest Calculator");
        println!("6. Retirement Calculator");
        println!("7. Investment Calculator");
        println!("8. Inflation Calculator");
        println!("9. Income Tax Calculator (Simple Est.)");
        println!("10. Salary Calculator");
        println!("11. Sales Tax Calculator");
        println!("0. Back");
        let choice = read_input("Select an option: ");

        match choice as i32 {
            1 => business_loan(),
            2 => mortgage(),
            3 => auto_loan(),
            4 => simple_interest(),
            5 => compound_interest(),
            6 => retirement(),
            7 => investment(),
            8 => inflation(),
            9 => income_tax(),
            10 => salary(),
            11 => sales_tax(),
            0 => break,
            _ => println!("Invalid choice."),
        }
    }
}

fn business_loan() {
    println!("\n--- Business Loan Calculator ---");
    let loan_amount = read_input("Enter loan amount: ");
    let annual_interest_rate = read_input("Enter annual interest rate (%): ");
    let loan_term_years = read_input("Enter loan term (years): ");

    let monthly_rate = (annual_interest_rate / 100.0) / 12.0;
    let num_payments = loan_term_years * 12.0;

    if monthly_rate > 0.0 {
        let monthly_payment = loan_amount * (monthly_rate * (1.0 + monthly_rate).powf(num_payments)) / ((1.0 + monthly_rate).powf(num_payments) - 1.0);
        println!("Monthly Payment: {:.2}", monthly_payment);
        println!("Total Payment: {:.2}", monthly_payment * num_payments);
        println!("Total Interest: {:.2}", (monthly_payment * num_payments) - loan_amount);
    } else {
        let monthly_payment = loan_amount / num_payments;
        println!("Monthly Payment: {:.2}", monthly_payment);
        println!("Total Payment: {:.2}", loan_amount);
    }
}

fn mortgage() {
    println!("\n--- Mortgage Calculator ---");
    let home_price = read_input("Home Price: ");
    let down_payment = read_input("Down Payment: ");
    let loan_term = read_input("Loan Term (years): ");
    let interest_rate = read_input("Interest Rate (%): ");

    let loan_amount = home_price - down_payment;
    let monthly_rate = (interest_rate / 100.0) / 12.0;
    let num_payments = loan_term * 12.0;

    if monthly_rate > 0.0 {
        let monthly_payment = loan_amount * (monthly_rate * (1.0 + monthly_rate).powf(num_payments)) / ((1.0 + monthly_rate).powf(num_payments) - 1.0);
        println!("Monthly Payment (P&I): {:.2}", monthly_payment);
        println!("Total Cost: {:.2}", monthly_payment * num_payments + down_payment);
    } else {
        println!("Monthly Payment: {:.2}", loan_amount / num_payments);
    }
}

fn auto_loan() {
    println!("\n--- Auto Loan Calculator ---");
    let price = read_input("Auto Price: ");
    let down_payment = read_input("Down Payment: ");
    let trade_in = read_input("Trade-in Value: ");
    let term = read_input("Term (months): ");
    let rate = read_input("Interest Rate (%): ");

    let loan_amount = price - down_payment - trade_in;
    let monthly_rate = (rate / 100.0) / 12.0;

    if monthly_rate > 0.0 {
        let payment = loan_amount * (monthly_rate * (1.0 + monthly_rate).powf(term)) / ((1.0 + monthly_rate).powf(term) - 1.0);
        println!("Monthly Payment: {:.2}", payment);
        println!("Total Interest: {:.2}", (payment * term) - loan_amount);
    } else {
        println!("Monthly Payment: {:.2}", loan_amount / term);
    }
}

fn simple_interest() {
    let p = read_input("Principal Amount: ");
    let r = read_input("Interest Rate (% per year): ");
    let t = read_input("Time (years): ");
    let interest = (p * r * t) / 100.0;
    println!("Simple Interest: {:.2}", interest);
    println!("Total Amount: {:.2}", p + interest);
}

fn compound_interest() {
    let p = read_input("Principal Amount: ");
    let r = read_input("Annual Interest Rate (%): ");
    let t = read_input("Time (years): ");
    let n = read_input("Compounds per year (e.g., 12 for monthly): ");
    
    let rate = r / 100.0;
    let amount = p * (1.0 + rate / n).powf(n * t);
    println!("Future Value: {:.2}", amount);
    println!("Compound Interest: {:.2}", amount - p);
}

fn retirement() {
    let age = read_input("Current Age: ");
    let retire_age = read_input("Retirement Age: ");
    let current_savings = read_input("Current Savings: ");
    let monthly_contribution = read_input("Monthly Contribution: ");
    let rate = read_input("Expected Return Rate (%): ");

    let years = retire_age - age;
    let months = years * 12.0;
    let r = (rate / 100.0) / 12.0;

    let mut balance = current_savings;
    for _ in 0..(months as i32) {
        balance = balance * (1.0 + r) + monthly_contribution;
    }
    println!("Estimated Savings at Retirement: {:.2}", balance);
}

fn investment() {
    let principal = read_input("Starting Investment: ");
    let contribution = read_input("Monthly Contribution: ");
    let years = read_input("Years to grow: ");
    let rate = read_input("Annual Return (%): ");

    let r = (rate / 100.0) / 12.0;
    let months = years * 12.0;
    let mut balance = principal;
    for _ in 0..(months as i32) {
        balance = balance * (1.0 + r) + contribution;
    }
    println!("Future Value: {:.2}", balance);
}

fn inflation() {
    let amount = read_input("Current Amount: ");
    let rate = read_input("Average Inflation Rate (%): ");
    let years = read_input("Years in future: ");
    
    let future_value = amount * (1.0 + rate / 100.0).powf(years);
    println!("Future Value (purchasing power equivalent): {:.2}", future_value);
}

fn income_tax() {
    println!("(Note: This is a very simplified model)");
    let income = read_input("Annual Income: ");
    let tax_rate = if income < 50000.0 { 10.0 } else if income < 100000.0 { 20.0 } else { 30.0 };
    let tax = income * (tax_rate / 100.0);
    println!("Estimated Tax ({:.0}%): {:.2}", tax_rate, tax);
    println!("Net Income: {:.2}", income - tax);
}

fn salary() {
    let rate = read_input("Enter Salary Amount: ");
    println!("1. Hourly");
    println!("2. Monthly");
    println!("3. Annually");
    let unit = read_input("Select unit (1-3): ");

    match unit as i32 {
        1 => {
            println!("Monthly: {:.2}", rate * 40.0 * 4.33);
            println!("Annually: {:.2}", rate * 40.0 * 52.0);
        }
        2 => {
            println!("Hourly (est): {:.2}", rate / (40.0 * 4.33));
            println!("Annually: {:.2}", rate * 12.0);
        }
        3 => {
            println!("Hourly (est): {:.2}", rate / (40.0 * 52.0));
            println!("Monthly: {:.2}", rate / 12.0);
        }
        _ => println!("Invalid selection."),
    }
}

fn sales_tax() {
    let amount = read_input("Net Price: ");
    let rate = read_input("Tax Rate (%): ");
    let tax = amount * (rate / 100.0);
    println!("Tax Amount: {:.2}", tax);
    println!("Total Price: {:.2}", amount + tax);
}
