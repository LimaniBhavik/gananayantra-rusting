pub fn calculate_basic_tax(income: f64) -> Result<f64, String> {
    if income < 0.0 {
        return Err("Income cannot be negative".into());
    }
    if income <= 11000.0 {
        Ok(income * 0.10)
    } else if income <= 44725.0 {
        Ok(1100.0 + (income - 11000.0) * 0.12)
    } else {
        Ok(5147.0 + (income - 44725.0) * 0.22)
    } // Simplified
}

pub fn marriage_tax_impact(income1: f64, income2: f64) -> Result<(f64, f64, f64), String> {
    let single1 = calculate_basic_tax(income1)?;
    let single2 = calculate_basic_tax(income2)?;
    // Simplified joint calculation
    let joint_income = income1 + income2;
    let joint_tax = if joint_income <= 22000.0 {
        joint_income * 0.10
    } else if joint_income <= 89450.0 {
        2200.0 + (joint_income - 22000.0) * 0.12
    } else {
        10294.0 + (joint_income - 89450.0) * 0.22
    };

    let total_single = single1 + single2;
    Ok((total_single, joint_tax, joint_tax - total_single))
}

pub fn estate_tax(estate_value: f64) -> Result<f64, String> {
    let exemption = 13610000.0;
    if estate_value <= exemption {
        Ok(0.0)
    } else {
        Ok((estate_value - exemption) * 0.40)
    }
}

pub fn paycheck_estimator(
    gross_pay: f64,
    tax_rate_percent: f64,
    deductions: f64,
) -> Result<f64, String> {
    Ok(gross_pay - (gross_pay * tax_rate_percent / 100.0) - deductions)
}

pub fn hra_exemption(
    basic_salary: f64,
    da: f64,
    hra_received: f64,
    rent_paid: f64,
    is_metro: bool,
) -> Result<f64, String> {
    let salary = basic_salary + da;
    let opt1 = hra_received;
    let opt2 = rent_paid - (0.10 * salary);
    let opt3 = if is_metro {
        0.50 * salary
    } else {
        0.40 * salary
    };
    Ok(opt1.min(opt2.max(0.0)).min(opt3))
}

pub fn transport_allowance_exemption(received: f64) -> Result<f64, String> {
    let exempt: f64 = 3200.0 * 12.0;
    Ok(exempt.min(received))
}

pub fn education_allowance_exemption(received: f64, num_children: f64) -> Result<f64, String> {
    let exempt: f64 = 100.0 * num_children.min(2.0) * 12.0;
    Ok(exempt.min(received))
}

pub fn presumptive_income_44ada(gross_receipts: f64) -> Result<f64, String> {
    if gross_receipts > 7500000.0 {
        return Err("Gross receipts exceed limit for 44ADA".into());
    }
    Ok(gross_receipts * 0.50)
}

pub fn amt_tax(total_income: f64, deductions_claimed: f64) -> Result<(f64, f64), String> {
    let adjusted = total_income + deductions_claimed;
    Ok((adjusted, adjusted * 0.185))
}

pub fn partners_remuneration_limit(book_profit: f64) -> Result<f64, String> {
    if book_profit <= 0.0 {
        Ok(150000.0)
    } else if book_profit <= 300000.0 {
        Ok(150000.0f64.max(book_profit * 0.90))
    } else {
        Ok(270000.0 + (book_profit - 300000.0) * 0.60)
    }
}

pub fn capital_gains_indexation(cost: f64, cii_buy: f64, cii_sell: f64) -> Result<f64, String> {
    if cii_buy == 0.0 {
        return Err("CII buy cannot be zero".into());
    }
    Ok(cost * (cii_sell / cii_buy))
}

pub fn residential_status(days_current: f64, days_last_4_years: f64) -> String {
    if days_current >= 182.0 || (days_current >= 60.0 && days_last_4_years >= 365.0) {
        "Resident".to_string()
    } else {
        "Non-Resident".to_string()
    }
}

pub fn tds_late_fees(tds_amount: f64, months_delay: f64) -> Result<(f64, f64), String> {
    let interest = tds_amount * 0.015 * months_delay;
    let fee = 200.0 * months_delay; // Simplified, capped at TDS amount usually
    Ok((interest, fee.min(tds_amount)))
}

pub fn deduction_80c(investment: f64) -> f64 {
    investment.min(150000.0)
}

pub fn deduction_80d(self_premium: f64, parents_premium: f64, parents_senior: bool) -> f64 {
    let limit_parents = if parents_senior { 50000.0 } else { 25000.0 };
    self_premium.min(25000.0) + parents_premium.min(limit_parents)
}

pub fn gratuity_exemption(
    is_gov: bool,
    amount: f64,
    last_salary: f64,
    years_service: f64,
    covered_under_act: bool,
) -> Result<f64, String> {
    if is_gov {
        Ok(amount)
    } else {
        let limit = 2000000.0;
        let stat_limit = if covered_under_act {
            (15.0 / 26.0) * last_salary * years_service
        } else {
            0.5 * last_salary * years_service
        };
        Ok(amount.min(limit).min(stat_limit))
    }
}

pub fn leave_encashment_exemption(
    is_gov: bool,
    amount: f64,
    avg_salary: f64,
    leave_balance_months: f64,
) -> Result<f64, String> {
    if is_gov {
        Ok(amount)
    } else {
        let limit = 2500000.0;
        Ok(amount
            .min(limit)
            .min(avg_salary * 10.0)
            .min(avg_salary * leave_balance_months))
    }
}

pub fn deferred_tax(
    accounting_income: f64,
    taxable_income: f64,
    tax_rate: f64,
) -> Result<f64, String> {
    Ok((accounting_income - taxable_income) * (tax_rate / 100.0))
}
