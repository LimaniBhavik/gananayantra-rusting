use crate::calculators::utils::read_input;

pub fn run_menu() {
    loop {
        println!("\n--- Auto Industry Calculators ---");
        println!("1. Auto Loan Calculator");
        println!("2. Cash Back vs. Low Interest");
        println!("3. Auto Lease Calculator");
        println!("0. Back");
        let choice = read_input("Select an option: ");

        match choice as i32 {
            1 => auto_loan(),
            2 => cash_back_vs_interest(),
            3 => auto_lease(),
            0 => break,
            _ => println!("Invalid choice."),
        }
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
        println!("Total Cost: {:.2}", payment * term + down_payment + trade_in);
    } else {
        println!("Monthly Payment: {:.2}", loan_amount / term);
    }
}

fn cash_back_vs_interest() {
    println!("\n--- Cash Back vs. Low Interest ---");
    let price = read_input("Auto Price: ");
    let cash_back = read_input("Cash Back Amount: ");
    let interest_rate_with_cash = read_input("Interest Rate with Cash Back (%): ");
    let low_interest_rate = read_input("Low Interest Rate (%): ");
    let term = read_input("Loan Term (months): ");

    // Option 1: Cash Back
    let loan1 = price - cash_back;
    let r1 = (interest_rate_with_cash / 100.0) / 12.0;
    let p1 = if r1 > 0.0 {
        loan1 * (r1 * (1.0 + r1).powf(term)) / ((1.0 + r1).powf(term) - 1.0)
    } else {
        loan1 / term
    };
    let total1 = p1 * term;

    // Option 2: Low Interest
    let loan2 = price;
    let r2 = (low_interest_rate / 100.0) / 12.0;
    let p2 = if r2 > 0.0 {
        loan2 * (r2 * (1.0 + r2).powf(term)) / ((1.0 + r2).powf(term) - 1.0)
    } else {
        loan2 / term
    };
    let total2 = p2 * term;

    println!("\nResults over {} months:", term);
    println!("Option 1 (Cash Back): Monthly ${:.2}, Total ${:.2}", p1, total1);
    println!("Option 2 (Low Interest): Monthly ${:.2}, Total ${:.2}", p2, total2);
    
    if total1 < total2 {
        println!("Winner: Cash Back saves you ${:.2}", total2 - total1);
    } else {
        println!("Winner: Low Interest saves you ${:.2}", total1 - total2);
    }
}

fn auto_lease() {
    println!("\n--- Auto Lease Calculator ---");
    let msrp = read_input("MSRP: ");
    let capitalized_cost = read_input("Negotiated Cap Cost: ");
    let residual_value = read_input("Residual Value (at end of lease): ");
    let money_factor = read_input("Money Factor (e.g., 0.00125): ");
    let term = read_input("Term (months): ");
    let tax_rate = read_input("Sales Tax Rate (%): ");

    let monthly_depreciation = (capitalized_cost - residual_value) / term;
    let monthly_finance = (capitalized_cost + residual_value) * money_factor;
    let base_payment = monthly_depreciation + monthly_finance;
    let tax = base_payment * (tax_rate / 100.0);

    println!("\nMonthly Depreciation: {:.2}", monthly_depreciation);
    println!("Monthly Finance Fee: {:.2}", monthly_finance);
    println!("Base Monthly Payment: {:.2}", base_payment);
    println!("Total Monthly with Tax: {:.2}", base_payment + tax);
}
