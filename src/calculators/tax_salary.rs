use crate::calculators::utils::read_input;

pub fn run_menu() {
    loop {
        println!("\n--- Tax and Salary Industry Calculators ---");
        println!("1. Income Tax Calculator (Basic)");
        println!("2. Salary Calculator");
        println!("3. Marriage Tax Calculator (Penalty/Bonus)");
        println!("4. Estate Tax Calculator");
        println!("5. Take-Home-Paycheck Calculator");
        println!("6. HRA Exemption Calculator");
        println!("7. Transport & Education Allowance");
        println!("8. Presumptive Taxation (44ADA)");
        println!("9. Old vs New Regime Comparison");
        println!("10. AMT (Alternate Minimum Tax)");
        println!("11. Partners Remuneration (40b)");
        println!("12. Capital Gains Indexation");
        println!("13. Residential Status Calculator");
        println!("14. TDS & Late Fees/Interest");
        println!("15. Section 80G Donation");
        println!("0. Back");
        let choice = read_input("Select an option: ");

        match choice as i32 {
            1 => income_tax_calc(),
            2 => salary_calc(),
            3 => marriage_tax_calc(),
            4 => estate_tax_calc(),
            5 => paycheck_calc(),
            6 => hra_calculator(),
            7 => allowance_calculator(),
            8 => presumptive_tax_44ada(),
            9 => tax_regime_comparison(),
            10 => amt_calculator(),
            11 => partners_remuneration(),
            12 => capital_gains_indexation(),
            13 => residential_status(),
            14 => tds_and_late_fees(),
            15 => deduction_80g(),
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

fn hra_calculator() {
    println!("\n--- HRA Exemption Calculator (India) ---");
    let basic_salary = read_input("Basic Salary (Annual): ");
    let da = read_input("DA (if part of retirement benefits): ");
    let hra_received = read_input("HRA Received: ");
    let rent_paid = read_input("Actual Rent Paid: ");
    let is_metro = read_input("Living in Metro City? (1 for Yes, 0 for No): ");

    let salary = basic_salary + da;
    let opt1 = hra_received;
    let opt2 = rent_paid - (0.10 * salary);
    let opt3 = if is_metro == 1.0 { 0.50 * salary } else { 0.40 * salary };

    let exemption = opt1.min(opt2.max(0.0)).min(opt3);
    println!("Exempt HRA: {:.2}", exemption);
    println!("Taxable HRA: {:.2}", hra_received - exemption);
}

fn allowance_calculator() {
    println!("\n--- Transport & Education Allowance ---");
    println!("1. Transport Allowance (Blind/Handicapped only for exempt)");
    println!("2. Children Education Allowance");
    println!("3. Hostel Allowance");
    let choice = read_input("Choice: ");

    match choice as i32 {
        1 => {
            let received = read_input("Allowance Received: ");
            let exempt: f64 = 3200.0 * 12.0; // Standard for handicapped
            println!("Exempt: {:.2}, Taxable: {:.2}", exempt.min(received), (received - exempt).max(0.0));
        }
        2 => {
            let received = read_input("Allowance Received: ");
            let kids = read_input("Number of kids (Max 2 for exempt): ");
            let exempt: f64 = 100.0 * kids.min(2.0) * 12.0;
            println!("Exempt: {:.2}, Taxable: {:.2}", exempt.min(received), (received - exempt).max(0.0));
        }
        3 => {
            let received = read_input("Allowance Received: ");
            let kids = read_input("Number of kids (Max 2 for exempt): ");
            let exempt: f64 = 300.0 * kids.min(2.0) * 12.0;
            println!("Exempt: {:.2}, Taxable: {:.2}", exempt.min(received), (received - exempt).max(0.0));
        }
        _ => println!("Invalid choice."),
    }
}

fn presumptive_tax_44ada() {
    println!("\n--- Presumptive Taxation u/s 44ADA ---");
    let gross_receipts = read_input("Total Gross Receipts: ");
    if gross_receipts > 7500000.0 {
        println!("Limit exceeded (75L). Presumptive scheme may not apply.");
    }
    let income = gross_receipts * 0.50;
    println!("Presumptive Income (50%): {:.2}", income);
}

fn tax_regime_comparison() {
    println!("\n--- Old vs New Tax Regime (India FY 24-25) ---");
    let income = read_input("Total Taxable Income: ");
    let deductions = read_input("Total Deductions (80C, etc. - only for Old): ");

    let old_taxable = (income - deductions - 50000.0).max(0.0); // Standard deduction
    let old_tax = if old_taxable <= 250000.0 { 0.0 }
        else if old_taxable <= 500000.0 { (old_taxable - 250000.0) * 0.05 }
        else if old_taxable <= 1000000.0 { 12500.0 + (old_taxable - 500000.0) * 0.20 }
        else { 112500.0 + (old_taxable - 1000000.0) * 0.30 };

    let new_taxable = (income - 75000.0).max(0.0); // New Standard deduction
    let new_tax = if new_taxable <= 300000.0 { 0.0 }
        else if new_taxable <= 700000.0 { (new_taxable - 300000.0) * 0.05 }
        else if new_taxable <= 1000000.0 { 20000.0 + (new_taxable - 700000.0) * 0.10 }
        else if new_taxable <= 1200000.0 { 50000.0 + (new_taxable - 1000000.0) * 0.15 }
        else if new_taxable <= 1500000.0 { 80000.0 + (new_taxable - 1200000.0) * 0.20 }
        else { 140000.0 + (new_taxable - 1500000.0) * 0.30 };

    println!("Old Regime Tax: {:.2}", old_tax);
    println!("New Regime Tax: {:.2}", new_tax);
    if old_tax < new_tax {
        println!("Old Regime is better by {:.2}", new_tax - old_tax);
    } else {
        println!("New Regime is better by {:.2}", old_tax - new_tax);
    }
}

fn amt_calculator() {
    println!("\n--- Alternate Minimum Tax (AMT) ---");
    let total_income = read_input("Regular Total Income: ");
    let deductions = read_input("Deductions claimed u/s 10AA/35AD/Chapter VI-A (80IA-80RRB): ");
    let adjusted_total_income = total_income + deductions;
    let amt_rate = 18.5 / 100.0;
    let amt = adjusted_total_income * amt_rate;
    println!("Adjusted Total Income: {:.2}", adjusted_total_income);
    println!("AMT Payable (18.5%): {:.2}", amt);
}

fn partners_remuneration() {
    println!("\n--- Partners Remuneration u/s 40(b) ---");
    let book_profit = read_input("Book Profit of Firm: ");
    let limit: f64 = if book_profit <= 0.0 {
        150000.0
    } else if book_profit <= 300000.0 {
        let pct_90 = book_profit * 0.90;
        if 150000.0 > pct_90 { 150000.0 } else { pct_90 }
    } else {
        270000.0 + (book_profit - 300000.0) * 0.60
    };
    println!("Max Allowable Remuneration: {:.2}", limit);
}

fn capital_gains_indexation() {
    println!("\n--- Indexed Cost of Acquisition ---");
    let cost = read_input("Cost of Acquisition: ");
    let year_buy = read_input("CII of Purchase Year: ");
    let year_sell = read_input("CII of Sale Year (e.g. 2024 CII = 363): ");
    let indexed_cost = cost * (year_sell / year_buy);
    println!("Indexed Cost: {:.2}", indexed_cost);
}

fn residential_status() {
    println!("\n--- Residential Status Calculator ---");
    let days_current = read_input("Days in India (Current FY): ");
    let days_4yrs = read_input("Days in India (Past 4 FYs): ");
    if days_current >= 182.0 || (days_current >= 60.0 && days_4yrs >= 365.0) {
        println!("Status: Resident");
    } else {
        println!("Status: Non-Resident (NRI)");
    }
}

fn tds_and_late_fees() {
    println!("\n--- TDS & Late Filing Fees ---");
    let amount = read_input("Payment Amount: ");
    let tds_rate = read_input("TDS Rate (%): ");
    let months_delay = read_input("Months delay in deposit: ");
    
    let tds = amount * (tds_rate / 100.0);
    let interest = tds * 0.015 * months_delay;
    let fee = 200.0 * months_delay.min(tds / 200.0); // Simplified Sec 234E
    
    println!("TDS to Deduct: {:.2}", tds);
    println!("Interest u/s 201(1A): {:.2}", interest);
    println!("Late Filing Fee (approx): {:.2}", fee);
}

fn deduction_80g() {
    println!("\n--- Section 80G Deduction ---");
    let donation = read_input("Donation Amount: ");
    println!("1. 100% Exemption (e.g. PM Relief Fund)");
    println!("2. 50% Exemption");
    let choice = read_input("Choice: ");
    let deduction = if choice == 1.0 { donation } else { donation * 0.5 };
    println!("Allowable Deduction: {:.2}", deduction);
}
