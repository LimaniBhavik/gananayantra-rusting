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
        println!("12. Amortization Calculator");
        println!("13. Mortgage Payoff Calculator");
        println!("14. House Affordability Calculator");
        println!("15. Rent Calculator");
        println!("16. Debt-to-Income (DTI) Ratio Calculator");
        println!("17. Real Estate Investment Calculator");
        println!("18. Refinance Calculator");
        println!("19. Rental Property Calculator");
        println!("20. APR Calculator");
        println!("21. FHA Loan Calculator");
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
            12 => amortization(),
            13 => mortgage_payoff(),
            14 => house_affordability(),
            15 => rent_calc(),
            16 => dti_ratio(),
            17 => real_estate_investment(),
            18 => refinance(),
            19 => rental_property(),
            20 => apr_calc(),
            21 => fha_loan(),
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

fn amortization() {
    println!("\n--- Amortization Calculator ---");
    let loan_amount = read_input("Loan Amount: ");
    let interest_rate = read_input("Annual Interest Rate (%): ");
    let loan_term = read_input("Loan Term (years): ");

    let monthly_rate = (interest_rate / 100.0) / 12.0;
    let num_payments = (loan_term * 12.0) as i32;

    if monthly_rate > 0.0 {
        let monthly_payment = loan_amount * (monthly_rate * (1.0 + monthly_rate).powf(num_payments as f64)) / ((1.0 + monthly_rate).powf(num_payments as f64) - 1.0);
        println!("{:<10} {:<15} {:<15} {:<15}", "Month", "Interest", "Principal", "Remaining");
        let mut balance = loan_amount;
        for i in 1..=num_payments {
            let interest = balance * monthly_rate;
            let principal = monthly_payment - interest;
            balance -= principal;
            if i % 12 == 0 || i == num_payments {
                println!("{:<10} {:<15.2} {:<15.2} {:<15.2}", i, interest, principal, balance.max(0.0));
            }
        }
    }
}

fn mortgage_payoff() {
    println!("\n--- Mortgage Payoff Calculator ---");
    let balance = read_input("Current Balance: ");
    let rate = read_input("Interest Rate (%): ");
    let current_payment = read_input("Current Monthly Payment: ");
    let extra_payment = read_input("Extra Monthly Payment: ");

    let monthly_rate = (rate / 100.0) / 12.0;
    let mut balance_standard = balance;
    let mut balance_extra = balance;
    let mut months_standard = 0;
    let mut months_extra = 0;

    while balance_standard > 0.0 && months_standard < 600 {
        balance_standard = balance_standard * (1.0 + monthly_rate) - current_payment;
        months_standard += 1;
    }

    while balance_extra > 0.0 && months_extra < 600 {
        balance_extra = balance_extra * (1.0 + monthly_rate) - (current_payment + extra_payment);
        months_extra += 1;
    }

    println!("Standard payoff: {} months ({:.1} years)", months_standard, months_standard as f64 / 12.0);
    println!("Extra payoff: {} months ({:.1} years)", months_extra, months_extra as f64 / 12.0);
    println!("Time saved: {} months", months_standard - months_extra);
}

fn house_affordability() {
    println!("\n--- House Affordability Calculator ---");
    let annual_income = read_input("Annual Gross Income: ");
    let monthly_debts = read_input("Monthly Debts (loans, etc.): ");
    let down_payment = read_input("Down Payment: ");
    
    let monthly_income = annual_income / 12.0;
    // Conservative 28% rule
    let max_monthly_payment = (monthly_income * 0.28).min(monthly_income * 0.36 - monthly_debts);
    
    println!("Based on the 28/36 rule:");
    println!("Max Monthly Mortgage Payment: {:.2}", max_monthly_payment.max(0.0));
    println!("Estimated Affordable House Price (w/ $0 down): {:.2}", max_monthly_payment * 150.0); // Rough estimate
    println!("Total Buying Power (inc. down payment): {:.2}", max_monthly_payment * 150.0 + down_payment);
}

fn rent_calc() {
    println!("\n--- Rent Calculator ---");
    let annual_income = read_input("Annual Gross Income: ");
    let monthly_rent = annual_income / 12.0 * 0.30;
    println!("Recommended maximum monthly rent (30% rule): {:.2}", monthly_rent);
}

fn dti_ratio() {
    println!("\n--- Debt-to-Income (DTI) Ratio Calculator ---");
    let monthly_income = read_input("Monthly Gross Income: ");
    let monthly_debt = read_input("Total Monthly Debt Payments: ");
    
    if monthly_income > 0.0 {
        let dti = (monthly_debt / monthly_income) * 100.0;
        println!("Your DTI Ratio is: {:.2}%", dti);
        if dti <= 36.0 {
            println!("Status: Excellent (Conventional limit is 36%)");
        } else if dti <= 43.0 {
            println!("Status: Good (FHA limit is typically 43%)");
        } else {
            println!("Status: High (May be difficult to qualify for some loans)");
        }
    }
}

fn real_estate_investment() {
    println!("\n--- Real Estate Investment Calculator ---");
    let price = read_input("Property Purchase Price: ");
    let monthly_rent = read_input("Expected Monthly Rent: ");
    let annual_expenses = read_input("Total Annual Expenses (Tax, Ins, Maint): ");
    
    let annual_revenue = monthly_rent * 12.0;
    let net_operating_income = annual_revenue - annual_expenses;
    let cap_rate = (net_operating_income / price) * 100.0;
    
    println!("Annual Gross Revenue: {:.2}", annual_revenue);
    println!("Net Operating Income (NOI): {:.2}", net_operating_income);
    println!("Capitalization Rate (Cap Rate): {:.2}%", cap_rate);
}

fn refinance() {
    println!("\n--- Refinance Calculator ---");
    let current_balance = read_input("Current Loan Balance: ");
    let current_rate = read_input("Current Interest Rate (%): ");
    let remaining_term = read_input("Remaining Term (years): ");
    let new_rate = read_input("New Interest Rate (%): ");
    let new_term = read_input("New Term (years): ");
    let costs = read_input("Refinancing Costs (closing, etc.): ");

    let r_old = (current_rate / 100.0) / 12.0;
    let n_old = remaining_term * 12.0;
    let p_old = current_balance * (r_old * (1.0 + r_old).powf(n_old)) / ((1.0 + r_old).powf(n_old) - 1.0);

    let r_new = (new_rate / 100.0) / 12.0;
    let n_new = new_term * 12.0;
    let p_new = current_balance * (r_new * (1.0 + r_new).powf(n_new)) / ((1.0 + r_new).powf(n_new) - 1.0);

    let monthly_savings = p_old - p_new;
    println!("Current Monthly Payment: {:.2}", p_old);
    println!("New Monthly Payment: {:.2}", p_new);
    println!("Monthly Savings: {:.2}", monthly_savings);
    if monthly_savings > 0.0 {
        println!("Break-even Point: {:.1} months", costs / monthly_savings);
    }
}

fn rental_property() {
    println!("\n--- Rental Property Calculator ---");
    let price = read_input("Purchase Price: ");
    let rent = read_input("Monthly Rent: ");
    let expenses = read_input("Monthly Expenses (Tax, Ins, Maint): ");
    let down_payment = read_input("Down Payment: ");

    let monthly_cash_flow = rent - expenses;
    let annual_cash_flow = monthly_cash_flow * 12.0;
    let coc_return = (annual_cash_flow / down_payment) * 100.0;

    println!("Monthly Cash Flow: {:.2}", monthly_cash_flow);
    println!("Annual Cash Flow: {:.2}", annual_cash_flow);
    println!("Cash-on-Cash Return: {:.2}%", coc_return);
}

fn apr_calc() {
    println!("\n--- APR Calculator (Simplified) ---");
    let loan_amount = read_input("Loan Amount: ");
    let costs = read_input("Upfront Costs/Fees: ");
    let rate = read_input("Interest Rate (%): ");
    let term = read_input("Term (years): ");

    let r = (rate / 100.0) / 12.0;
    let n = term * 12.0;
    let payment = loan_amount * (r * (1.0 + r).powf(n)) / ((1.0 + r).powf(n) - 1.0);
    
    // Iterative approach to find APR (simplified)
    let net_loan = loan_amount - costs;
    let mut apr = rate;
    for _ in 0..10 {
        let r_apr = (apr / 100.0) / 12.0;
        let p_apr = net_loan * (r_apr * (1.0 + r_apr).powf(n)) / ((1.0 + r_apr).powf(n) - 1.0);
        if p_apr < payment { apr += 0.1; } else { apr -= 0.05; }
    }

    println!("Nominal Rate: {:.2}%", rate);
    println!("Estimated APR: {:.2}%", apr);
}

fn fha_loan() {
    println!("\n--- FHA Loan Calculator (Simplified) ---");
    let price = read_input("Home Price: ");
    let down_payment = price * 0.035; // Standard 3.5%
    let loan_amount = price - down_payment;
    let mip = (loan_amount * 0.0055) / 12.0; // Typical 0.55% MIP

    println!("FHA Minimum Down Payment (3.5%): {:.2}", down_payment);
    println!("Base Loan Amount: {:.2}", loan_amount);
    println!("Estimated Monthly MIP: {:.2}", mip);
}
