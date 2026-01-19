use crate::calculators::utils::read_input;

pub fn run_menu() {
    loop {
        println!("\n--- Retirement Industry Calculators ---");
        println!("1. Retirement Calculator");
        println!("2. 401K Calculator");
        println!("3. Pension Calculator");
        println!("4. Social Security Calculator");
        println!("5. Annuity Calculator");
        println!("6. Annuity Payout Calculator");
        println!("7. Roth IRA Calculator");
        println!("8. IRA Calculator");
        println!("9. RMD Calculator");
        println!("0. Back");
        let choice = read_input("Select an option: ");

        match choice as i32 {
            1 => retirement_calc(),
            2 => k401_calc(),
            3 => pension_calc(),
            4 => social_security_calc(),
            5 => annuity_calc(),
            6 => annuity_payout_calc(),
            7 => roth_ira_calc(),
            8 => ira_calc(),
            9 => rmd_calc(),
            0 => break,
            _ => println!("Invalid choice."),
        }
    }
}

fn retirement_calc() {
    println!("\n--- Retirement Calculator ---");
    let current_age = read_input("Current Age: ");
    let retire_age = read_input("Planned Retirement Age: ");
    let current_savings = read_input("Current Savings: ");
    let monthly_contribution = read_input("Monthly Contribution: ");
    let annual_return = read_input("Annual Return (%): ");

    let years = retire_age - current_age;
    let r = (annual_return / 100.0) / 12.0;
    let n = years * 12.0;

    let total = current_savings * (1.0 + r).powf(n) + monthly_contribution * (((1.0 + r).powf(n) - 1.0) / r);
    println!("Estimated Savings at Retirement: {:.2}", total);
}

fn k401_calc() {
    println!("\n--- 401K Calculator ---");
    let salary = read_input("Annual Salary: ");
    let contribution_percent = read_input("Contribution (%): ");
    let employer_match = read_input("Employer Match (% of salary): ");
    let match_limit = read_input("Employer Match Limit (% of contribution): ");
    let current_balance = read_input("Current Balance: ");
    let years = read_input("Years to retirement: ");
    let annual_return = read_input("Annual Return (%): ");

    let annual_contribution = salary * (contribution_percent / 100.0);
    let annual_match = (salary * (employer_match / 100.0)).min(annual_contribution * (match_limit / 100.0));
    let total_annual = annual_contribution + annual_match;

    let r = annual_return / 100.0;
    let mut balance = current_balance;
    for _ in 0..(years as i32) {
        balance = (balance + total_annual) * (1.0 + r);
    }

    println!("Annual Total Contribution: {:.2}", total_annual);
    println!("Projected 401K Balance: {:.2}", balance);
}

fn pension_calc() {
    println!("\n--- Pension Calculator (Simplified) ---");
    let final_salary = read_input("Expected Final Salary: ");
    let years_service = read_input("Years of Service: ");
    let multiplier = read_input("Pension Multiplier (% per year, e.g., 2%): ");

    let annual_pension = final_salary * (years_service * (multiplier / 100.0));
    println!("Estimated Annual Pension: {:.2}", annual_pension);
    println!("Estimated Monthly Pension: {:.2}", annual_pension / 12.0);
}

fn social_security_calc() {
    println!("\n--- Social Security Estimate (Very Rough) ---");
    let current_annual_income = read_input("Current Annual Income: ");
    // Very simplified: roughly 40% of income for average earners
    let estimate = current_annual_income * 0.40;
    println!("Rough Annual Estimate: {:.2}", estimate);
    println!("Rough Monthly Estimate: {:.2}", estimate / 12.0);
}

fn annuity_calc() {
    println!("\n--- Annuity Calculator (Accumulation) ---");
    let contribution = read_input("Monthly Contribution: ");
    let rate = read_input("Annual Interest Rate (%): ");
    let years = read_input("Years: ");

    let r = (rate / 100.0) / 12.0;
    let n = years * 12.0;
    let total = contribution * (((1.0 + r).powf(n) - 1.0) / r);

    println!("Total Accumulated Value: {:.2}", total);
}

fn annuity_payout_calc() {
    println!("\n--- Annuity Payout Calculator ---");
    let principal = read_input("Annuity Principal: ");
    let rate = read_input("Annual Interest Rate (%): ");
    let years = read_input("Payout Years: ");

    let r = (rate / 100.0) / 12.0;
    let n = years * 12.0;
    let monthly_payout = principal * (r * (1.0 + r).powf(n)) / ((1.0 + r).powf(n) - 1.0);

    println!("Monthly Payout Amount: {:.2}", monthly_payout);
}

fn roth_ira_calc() {
    println!("\n--- Roth IRA Calculator ---");
    let initial = read_input("Current Roth IRA Balance: ");
    let annual_contribution = read_input("Annual Contribution: ");
    let years = read_input("Years until Withdrawal: ");
    let rate = read_input("Annual Return (%): ");

    let r = rate / 100.0;
    let mut balance = initial;
    for _ in 0..(years as i32) {
        balance = (balance + annual_contribution) * (1.0 + r);
    }

    println!("Projected Roth IRA Balance (Tax-Free): {:.2}", balance);
}

fn ira_calc() {
    println!("\n--- Traditional IRA Calculator ---");
    let initial = read_input("Current IRA Balance: ");
    let annual_contribution = read_input("Annual Contribution: ");
    let years = read_input("Years until Withdrawal: ");
    let rate = read_input("Annual Return (%): ");
    let tax_rate = read_input("Expected Tax Rate at Withdrawal (%): ");

    let r = rate / 100.0;
    let mut balance = initial;
    for _ in 0..(years as i32) {
        balance = (balance + annual_contribution) * (1.0 + r);
    }

    let tax = balance * (tax_rate / 100.0);
    println!("Projected Pre-Tax Balance: {:.2}", balance);
    println!("Estimated Tax: {:.2}", tax);
    println!("Projected After-Tax Balance: {:.2}", balance - tax);
}

fn rmd_calc() {
    println!("\n--- RMD Calculator (Simplified) ---");
    let balance = read_input("IRA/401K Balance: ");
    let age = read_input("Current Age: ");

    // Very simplified RMD factor (roughly 1/27.4 at age 72)
    let factor = match age as i32 {
        72 => 27.4,
        73 => 26.5,
        74 => 25.5,
        75 => 24.6,
        _ if age > 75.0 => 20.0, // Conservative placeholder
        _ => 0.0,
    };

    if factor > 0.0 {
        println!("Required Minimum Distribution (RMD): {:.2}", balance / factor);
    } else {
        println!("RMD typically starts at age 72 or 73.");
    }
}
