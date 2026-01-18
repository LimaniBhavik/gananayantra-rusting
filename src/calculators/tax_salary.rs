use crate::calculators::utils::read_input;

pub fn run_menu() {
    loop {
        println!("\n--- Tax and Salary Industry Calculators ---");
        println!("1. Income Tax Calculator (Basic)");
        println!("2. Salary Calculator");
        println!("3. Marriage Tax Calculator (Penalty/Bonus)");
        println!("4. Estate Tax Calculator");
        println!("5. Take-Home-Paycheck Calculator");
        println!("0. Back");
        let choice = read_input("Select an option: ");

        match choice as i32 {
            1 => income_tax_calc(),
            2 => salary_calc(),
            3 => marriage_tax_calc(),
            4 => estate_tax_calc(),
            5 => paycheck_calc(),
            0 => break,
            _ => println!("Invalid choice."),
        }
    }
}

fn income_tax_calc() {
    println!("\n--- Income Tax Calculator (Simplified) ---");
    let income = read_input("Annual Gross Income: ");
    
    // Simplified progressive tax brackets for estimation
    let tax = if income <= 11000.0 {
        income * 0.10
    } else if income <= 44725.0 {
        1100.0 + (income - 11000.0) * 0.12
    } else if income <= 95375.0 {
        5147.0 + (income - 44725.0) * 0.22
    } else {
        16290.0 + (income - 95375.0) * 0.24
    };

    println!("Estimated Federal Income Tax: {:.2}", tax);
    println!("Effective Tax Rate: {:.2}%", (tax / income) * 100.0);
}

fn salary_calc() {
    println!("\n--- Salary Calculator ---");
    let amount = read_input("Enter Amount: ");
    println!("1. Hourly");
    println!("2. Weekly");
    println!("3. Monthly");
    println!("4. Annual");
    let unit = read_input("Select Unit: ");

    let annual = match unit as i32 {
        1 => amount * 40.0 * 52.0,
        2 => amount * 52.0,
        3 => amount * 12.0,
        4 => amount,
        _ => {
            println!("Invalid unit.");
            return;
        }
    };

    println!("\nEquivalent Earnings:");
    println!("Hourly (40h/week): {:.2}", annual / 52.0 / 40.0);
    println!("Weekly: {:.2}", annual / 52.0);
    println!("Monthly: {:.2}", annual / 12.0);
    println!("Annual: {:.2}", annual);
}

fn marriage_tax_calc() {
    println!("\n--- Marriage Tax Calculator ---");
    let income1 = read_input("Spouse 1 Income: ");
    let income2 = read_input("Spouse 2 Income: ");

    let single_tax1 = calculate_basic_tax(income1);
    let single_tax2 = calculate_basic_tax(income2);
    let joint_tax = calculate_joint_tax(income1 + income2);

    let total_single = single_tax1 + single_tax2;
    let diff = joint_tax - total_single;

    println!("Total Tax if Single: {:.2}", total_single);
    println!("Tax if Filing Jointly: {:.2}", joint_tax);
    if diff > 0.0 {
        println!("Marriage Penalty: {:.2}", diff);
    } else {
        println!("Marriage Bonus (Savings): {:.2}", diff.abs());
    }
}

fn calculate_basic_tax(income: f64) -> f64 {
    if income <= 11000.0 { income * 0.10 }
    else if income <= 44725.0 { 1100.0 + (income - 11000.0) * 0.12 }
    else { 5147.0 + (income - 44725.0) * 0.22 }
}

fn calculate_joint_tax(income: f64) -> f64 {
    // Joint brackets are typically wider
    if income <= 22000.0 { income * 0.10 }
    else if income <= 89450.0 { 2200.0 + (income - 22000.0) * 0.12 }
    else { 10294.0 + (income - 89450.0) * 0.22 }
}

fn estate_tax_calc() {
    println!("\n--- Estate Tax Calculator ---");
    let estate_value = read_input("Total Estate Value: ");
    let exemption = 13610000.0; // 2024 Federal Exemption

    if estate_value <= exemption {
        println!("Estate value is within the federal exemption limit (${:.0}).", exemption);
        println!("Estimated Federal Estate Tax: 0.00");
    } else {
        let taxable = estate_value - exemption;
        let tax = taxable * 0.40; // Flat 40% top rate for estimation
        println!("Taxable Estate Amount: {:.2}", taxable);
        println!("Estimated Federal Estate Tax (40%): {:.2}", tax);
    }
}

fn paycheck_calc() {
    println!("\n--- Take-Home-Paycheck Calculator ---");
    let gross_pay = read_input("Gross Pay (per period): ");
    let tax_rate = read_input("Estimated Tax Withholding (%): ");
    let deductions = read_input("Other Deductions (Health, 401k, etc): ");

    let tax = gross_pay * (tax_rate / 100.0);
    let net_pay = gross_pay - tax - deductions;

    println!("Gross Pay: {:.2}", gross_pay);
    println!("Estimated Taxes: {:.2}", tax);
    println!("Other Deductions: {:.2}", deductions);
    println!("Net Take-Home Pay: {:.2}", net_pay);
}
