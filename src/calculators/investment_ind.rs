use crate::calculators::utils::read_input;

pub fn run_menu() {
    loop {
        println!("\n--- Investment Industry Calculators ---");
        println!("1. Interest Calculator");
        println!("2. Investment Calculator");
        println!("3. Finance Calculator");
        println!("4. Compound Interest Calculator");
        println!("5. Interest Rate Calculator");
        println!("6. Savings Calculator");
        println!("7. CD Calculator");
        println!("8. Bond Calculator");
        println!("9. Average Return Calculator");
        println!("10. IRR Calculator (Approx)");
        println!("11. ROI Calculator");
        println!("12. Payback Period Calculator");
        println!("13. Present Value Calculator");
        println!("14. Future Value Calculator");
        println!("0. Back");
        let choice = read_input("Select an option: ");

        match choice as i32 {
            1 => interest_calculator(),
            2 => investment_calculator(),
            3 => finance_calculator(),
            4 => compound_interest_calculator(),
            5 => interest_rate_calculator(),
            6 => savings_calculator(),
            7 => cd_calculator(),
            8 => bond_calculator(),
            9 => average_return_calculator(),
            10 => irr_calculator(),
            11 => roi_calculator(),
            12 => payback_period_calculator(),
            13 => present_value_calculator(),
            14 => future_value_calculator(),
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

fn savings_calculator() {
    println!("\n--- Savings Calculator ---");
    let initial = read_input("Initial Balance: ");
    let monthly = read_input("Monthly Contribution: ");
    let rate = read_input("Annual Interest Rate (%): ");
    let years = read_input("Years: ");

    let r = (rate / 100.0) / 12.0;
    let n = years * 12.0;
    
    let total = initial * (1.0 + r).powf(n) + monthly * (((1.0 + r).powf(n) - 1.0) / r);
    println!("Total Savings: {:.2}", total);
}

fn cd_calculator() {
    println!("\n--- CD Calculator ---");
    let deposit = read_input("Initial Deposit: ");
    let rate = read_input("APY (%): ");
    let months = read_input("Term (months): ");

    let r = rate / 100.0;
    let t = months / 12.0;
    let total = deposit * (1.0 + r).powf(t);

    println!("Total at Maturity: {:.2}", total);
    println!("Total Interest: {:.2}", total - deposit);
}

fn bond_calculator() {
    println!("\n--- Bond Calculator (Current Yield) ---");
    let par_value = read_input("Par Value: ");
    let coupon_rate = read_input("Coupon Rate (%): ");
    let market_price = read_input("Market Price: ");

    let annual_coupon = par_value * (coupon_rate / 100.0);
    let yield_val = (annual_coupon / market_price) * 100.0;

    println!("Annual Coupon Payment: {:.2}", annual_coupon);
    println!("Current Yield: {:.2}%", yield_val);
}

fn average_return_calculator() {
    println!("\n--- Average Return Calculator ---");
    let count = read_input("Number of periods: ");
    let mut total_return = 0.0;

    for i in 1..=(count as i32) {
        println!("Enter return for period {} (%): ", i);
        let r = read_input("> ");
        total_return += r;
    }

    println!("Arithmetic Average Return: {:.2}%", total_return / count);
}

fn irr_calculator() {
    println!("\n--- IRR Calculator (Simplified/Trial & Error) ---");
    let initial = read_input("Initial Investment (positive number): ");
    let cash_flow = read_input("Annual Cash Flow: ");
    let years = read_input("Years: ");

    // Simple estimation for constant cash flows: initial = cf * [1 - (1+r)^-n] / r
    // We'll provide a rough approximation using a common formula for annuity IRR
    let mut guess: f64 = 0.1;
    for _ in 0..100 {
        let f = cash_flow * ((1.0 - (1.0 + guess).powf(-years)) / guess) - initial;
        let df = cash_flow * ((years * (1.0 + guess).powf(-years - 1.0)) / guess - (1.0 - (1.0 + guess).powf(-years)) / (guess * guess));
        guess = guess - f / df;
    }

    println!("Approximate IRR: {:.2}%", guess * 100.0);
}

fn roi_calculator() {
    println!("\n--- ROI Calculator ---");
    let gained = read_input("Amount Gained: ");
    let spent = read_input("Amount Spent: ");

    let roi = ((gained - spent) / spent) * 100.0;
    println!("Return on Investment (ROI): {:.2}%", roi);
}

fn payback_period_calculator() {
    println!("\n--- Payback Period Calculator ---");
    let investment = read_input("Initial Investment: ");
    let annual_cash_flow = read_input("Annual Cash Flow: ");

    let period = investment / annual_cash_flow;
    println!("Payback Period: {:.2} years", period);
}

fn present_value_calculator() {
    println!("\n--- Present Value Calculator ---");
    let future_value = read_input("Future Value: ");
    let rate = read_input("Annual Rate (%): ");
    let periods = read_input("Number of Periods (years): ");

    let pv = future_value / (1.0 + rate / 100.0).powf(periods);
    println!("Present Value: {:.2}", pv);
}

fn future_value_calculator() {
    println!("\n--- Future Value Calculator ---");
    let present_value = read_input("Present Value: ");
    let rate = read_input("Annual Rate (%): ");
    let periods = read_input("Number of Periods (years): ");

    let fv = present_value * (1.0 + rate / 100.0).powf(periods);
    println!("Future Value: {:.2}", fv);
}
