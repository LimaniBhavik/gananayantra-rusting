pub fn retirement_corpus(
    current_savings: f64,
    monthly_contribution: f64,
    annual_return: f64,
    years: f64,
) -> Result<f64, String> {
    let r = (annual_return / 100.0) / 12.0;
    let n = years * 12.0;
    Ok(current_savings * (1.0 + r).powf(n) + monthly_contribution * (((1.0 + r).powf(n) - 1.0) / r))
}

pub fn k401_balance(
    salary: f64,
    contribution_percent: f64,
    employer_match_percent: f64,
    match_limit_percent: f64,
    current_balance: f64,
    annual_return: f64,
    years: f64
) -> Result<(f64, f64), String> {
    let annual_contribution = salary * (contribution_percent / 100.0);
    let annual_match = (salary * (employer_match_percent / 100.0)).min(annual_contribution * (match_limit_percent / 100.0));
    let total_annual = annual_contribution + annual_match;

    let r = annual_return / 100.0;
    let mut balance = current_balance;
    for _ in 0..(years as i32) {
        balance = (balance + total_annual) * (1.0 + r);
    }
    Ok((total_annual, balance))
}

pub fn estimate_pension(final_salary: f64, years_service: f64, multiplier_percent: f64) -> Result<f64, String> {
    Ok(final_salary * years_service * (multiplier_percent / 100.0))
}

pub fn estimate_social_security(current_income: f64) -> Result<f64, String> {
    Ok(current_income * 0.40) // Very rough estimate
}

pub fn annuity_accumulation(monthly_contribution: f64, annual_rate: f64, years: f64) -> Result<f64, String> {
    let r = (annual_rate / 100.0) / 12.0;
    let n = years * 12.0;
    Ok(monthly_contribution * (((1.0 + r).powf(n) - 1.0) / r))
}

pub fn annuity_payout(principal: f64, annual_rate: f64, years: f64) -> Result<f64, String> {
    let r = (annual_rate / 100.0) / 12.0;
    let n = years * 12.0;
    Ok(principal * (r * (1.0 + r).powf(n)) / ((1.0 + r).powf(n) - 1.0))
}

pub fn roth_ira_balance(current_balance: f64, annual_contribution: f64, annual_return: f64, years: f64) -> Result<f64, String> {
    let r = annual_return / 100.0;
    let mut balance = current_balance;
    for _ in 0..(years as i32) {
        balance = (balance + annual_contribution) * (1.0 + r);
    }
    Ok(balance)
}

pub fn traditional_ira_balance(current_balance: f64, annual_contribution: f64, annual_return: f64, years: f64, tax_rate_at_withdrawal: f64) -> Result<(f64, f64, f64), String> {
    let r = annual_return / 100.0;
    let mut balance = current_balance;
    for _ in 0..(years as i32) {
        balance = (balance + annual_contribution) * (1.0 + r);
    }
    let tax = balance * (tax_rate_at_withdrawal / 100.0);
    Ok((balance, tax, balance - tax))
}

pub fn rmd_estimate(balance: f64, age: f64) -> Result<f64, String> {
    // Simplified RMD table
    let factor = match age as i32 {
        72 => 27.4,
        73 => 26.5,
        74 => 25.5,
        75 => 24.6,
        _ if age > 75.0 => 20.0,
        _ => 0.0,
    };
    if factor == 0.0 {
        Ok(0.0)
    } else {
        Ok(balance / factor)
    }
}
