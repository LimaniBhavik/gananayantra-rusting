/// Generic result for loan calculations
#[derive(Debug, PartialEq)]
pub struct LoanResult {
    pub monthly_payment: f64,
    pub total_payment: f64,
    pub total_interest: f64,
}

/// Helper to calculate PMT (payment)
/// r: monthly rate (decimal)
/// n: number of months
/// p: principal
fn calculate_pmt(r: f64, n: f64, p: f64) -> f64 {
    if r == 0.0 {
        p / n
    } else {
        p * (r * (1.0 + r).powf(n)) / ((1.0 + r).powf(n) - 1.0)
    }
}

pub fn business_loan(amount: f64, annual_rate: f64, years: f64) -> Result<LoanResult, String> {
    if amount < 0.0 || annual_rate < 0.0 || years <= 0.0 {
        return Err("Invalid input values".into());
    }
    let monthly_rate = (annual_rate / 100.0) / 12.0;
    let num_payments = years * 12.0;
    let payment = calculate_pmt(monthly_rate, num_payments, amount);
    let total_payment = payment * num_payments;

    Ok(LoanResult {
        monthly_payment: payment,
        total_payment,
        total_interest: total_payment - amount,
    })
}

pub fn mortgage(
    home_price: f64,
    down_payment: f64,
    years: f64,
    annual_rate: f64,
) -> Result<LoanResult, String> {
    if home_price < 0.0 || down_payment < 0.0 || years <= 0.0 || annual_rate < 0.0 {
        return Err("Invalid input values".into());
    }
    let loan_amount = home_price - down_payment;
    let monthly_rate = (annual_rate / 100.0) / 12.0;
    let num_payments = years * 12.0;
    let payment = calculate_pmt(monthly_rate, num_payments, loan_amount);
    let total_payment = payment * num_payments;

    Ok(LoanResult {
        monthly_payment: payment,
        total_payment: total_payment + down_payment, // Total cost including down payment
        total_interest: total_payment - loan_amount,
    })
}

pub fn simple_interest(principal: f64, annual_rate: f64, years: f64) -> Result<(f64, f64), String> {
    if principal < 0.0 || annual_rate < 0.0 || years < 0.0 {
        return Err("Invalid input values".into());
    }
    let interest = (principal * annual_rate * years) / 100.0;
    Ok((interest, principal + interest))
}

pub fn retirement_savings(
    current_savings: f64,
    monthly_contribution: f64,
    current_age: f64,
    retirement_age: f64,
    annual_return_rate: f64,
) -> Result<f64, String> {
    if retirement_age <= current_age {
        return Err("Retirement age must be greater than current age".into());
    }
    let months = (retirement_age - current_age) * 12.0;
    let monthly_rate = (annual_return_rate / 100.0) / 12.0;

    let mut balance = current_savings;
    for _ in 0..(months as i32) {
        balance = balance * (1.0 + monthly_rate) + monthly_contribution;
    }
    Ok(balance)
}

pub fn investment_growth(
    principal: f64,
    monthly_contribution: f64,
    years: f64,
    annual_return_rate: f64,
) -> Result<f64, String> {
    if years < 0.0 {
        return Err("Years cannot be negative".into());
    }
    let monthly_rate = (annual_return_rate / 100.0) / 12.0;
    let months = years * 12.0;
    let mut balance = principal;
    for _ in 0..(months as i32) {
        balance = balance * (1.0 + monthly_rate) + monthly_contribution;
    }
    Ok(balance)
}

pub fn inflation_impact(amount: f64, inflation_rate: f64, years: f64) -> Result<f64, String> {
    if years < 0.0 {
        return Err("Years cannot be negative".into());
    }
    // Future value required to match current purchasing power, OR future purchasing power of current amount?
    // The original code was: amount * (1.0 + rate/100)^years. This is "Future Value of Amount" if grown by inflation.
    // Or "Cost in future".
    Ok(amount * (1.0 + inflation_rate / 100.0).powf(years))
}

pub fn estimate_income_tax(annual_income: f64) -> Result<(f64, f64, f64), String> {
    if annual_income < 0.0 {
        return Err("Income cannot be negative".into());
    }
    let tax_rate = if annual_income < 50000.0 {
        10.0
    } else if annual_income < 100000.0 {
        20.0
    } else {
        30.0
    };
    let tax = annual_income * (tax_rate / 100.0);
    Ok((tax_rate, tax, annual_income - tax))
}

pub struct SalaryBreakdown {
    pub hourly: f64,
    pub monthly: f64,
    pub annual: f64,
}

pub fn convert_salary(amount: f64, unit: &str) -> Result<SalaryBreakdown, String> {
    if amount < 0.0 {
        return Err("Amount cannot be negative".into());
    }
    match unit.to_lowercase().as_str() {
        "hourly" => Ok(SalaryBreakdown {
            hourly: amount,
            monthly: amount * 40.0 * 4.33,
            annual: amount * 40.0 * 52.0,
        }),
        "monthly" => Ok(SalaryBreakdown {
            hourly: amount / (40.0 * 4.33),
            monthly: amount,
            annual: amount * 12.0,
        }),
        "annual" | "annually" => Ok(SalaryBreakdown {
            hourly: amount / (40.0 * 52.0),
            monthly: amount / 12.0,
            annual: amount,
        }),
        _ => Err("Invalid unit. Use 'hourly', 'monthly', or 'annual'".into()),
    }
}

pub fn sales_tax(net_price: f64, tax_rate: f64) -> Result<(f64, f64), String> {
    if net_price < 0.0 || tax_rate < 0.0 {
        return Err("Inputs must be positive".into());
    }
    let tax = net_price * (tax_rate / 100.0);
    Ok((tax, net_price + tax))
}

#[derive(Debug, PartialEq)]
pub struct AmortizationRow {
    pub month: i32,
    pub interest: f64,
    pub principal: f64,
    pub remaining_balance: f64,
}

pub fn amortization_schedule(
    loan_amount: f64,
    annual_rate: f64,
    years: f64,
) -> Result<Vec<AmortizationRow>, String> {
    if loan_amount < 0.0 || annual_rate < 0.0 || years <= 0.0 {
        return Err("Invalid inputs".into());
    }
    let monthly_rate = (annual_rate / 100.0) / 12.0;
    let num_payments = (years * 12.0) as i32;
    let monthly_payment = calculate_pmt(monthly_rate, num_payments as f64, loan_amount);

    let mut schedule = Vec::new();
    let mut balance = loan_amount;

    for i in 1..=num_payments {
        let interest = balance * monthly_rate;
        let principal = monthly_payment - interest;
        balance -= principal;
        if balance < 0.0001 {
            balance = 0.0;
        } // Float precision fix

        schedule.push(AmortizationRow {
            month: i,
            interest,
            principal,
            remaining_balance: balance,
        });
    }
    Ok(schedule)
}

pub fn mortgage_payoff(
    current_balance: f64,
    annual_rate: f64,
    current_payment: f64,
    extra_payment: f64,
) -> Result<(i32, i32, i32), String> {
    if current_balance < 0.0 || annual_rate < 0.0 {
        return Err("Invalid inputs".into());
    }

    let monthly_rate = (annual_rate / 100.0) / 12.0;

    let calc_months = |mut balance: f64, payment: f64| -> i32 {
        let mut months = 0;
        if payment <= balance * monthly_rate {
            return 9999;
        } // Will never pay off
        while balance > 0.0 {
            balance = balance * (1.0 + monthly_rate) - payment;
            months += 1;
            if months > 1200 {
                break;
            } // Safety limit
        }
        months
    };

    let months_standard = calc_months(current_balance, current_payment);
    let months_extra = calc_months(current_balance, current_payment + extra_payment);

    Ok((
        months_standard,
        months_extra,
        months_standard - months_extra,
    ))
}

pub fn house_affordability(
    annual_income: f64,
    monthly_debts: f64,
    down_payment: f64,
) -> Result<(f64, f64, f64), String> {
    if annual_income < 0.0 {
        return Err("Income cannot be negative".into());
    }
    let monthly_income = annual_income / 12.0;
    let max_monthly_payment = (monthly_income * 0.28)
        .min(monthly_income * 0.36 - monthly_debts)
        .max(0.0);
    let estimated_price = max_monthly_payment * 150.0; // Rough estimate rule
    Ok((
        max_monthly_payment,
        estimated_price,
        estimated_price + down_payment,
    ))
}

pub fn recommended_rent(annual_income: f64) -> Result<f64, String> {
    if annual_income < 0.0 {
        return Err("Income cannot be negative".into());
    }
    Ok((annual_income / 12.0) * 0.30)
}

pub fn dti_ratio(monthly_income: f64, monthly_debt: f64) -> Result<(f64, String), String> {
    if monthly_income <= 0.0 {
        return Err("Income must be greater than zero".into());
    }
    let dti = (monthly_debt / monthly_income) * 100.0;
    let status = if dti <= 36.0 {
        "Excellent"
    } else if dti <= 43.0 {
        "Good"
    } else {
        "High"
    };
    Ok((dti, status.to_string()))
}

pub fn real_estate_investment(
    price: f64,
    monthly_rent: f64,
    annual_expenses: f64,
) -> Result<(f64, f64, f64), String> {
    let annual_revenue = monthly_rent * 12.0;
    let noi = annual_revenue - annual_expenses;
    if price <= 0.0 {
        return Err("Price must be > 0".into());
    }
    let cap_rate = (noi / price) * 100.0;
    Ok((annual_revenue, noi, cap_rate))
}

pub fn refinance_savings(
    balance: f64,
    current_rate: f64,
    remaining_years: f64,
    new_rate: f64,
    new_years: f64,
    costs: f64,
) -> Result<(f64, f64, f64, f64), String> {
    let monthly_rate_old = (current_rate / 100.0) / 12.0;
    let n_old = remaining_years * 12.0;
    let p_old = calculate_pmt(monthly_rate_old, n_old, balance);

    let monthly_rate_new = (new_rate / 100.0) / 12.0;
    let n_new = new_years * 12.0;
    let p_new = calculate_pmt(monthly_rate_new, n_new, balance);

    let monthly_savings = p_old - p_new;
    let break_even_months = if monthly_savings > 0.0 {
        costs / monthly_savings
    } else {
        0.0
    };

    Ok((p_old, p_new, monthly_savings, break_even_months))
}

pub fn rental_property_return(
    monthly_rent: f64,
    monthly_expenses: f64,
    down_payment: f64,
) -> Result<(f64, f64, f64), String> {
    let monthly_cash_flow = monthly_rent - monthly_expenses;
    let annual_cash_flow = monthly_cash_flow * 12.0;
    if down_payment <= 0.0 {
        return Err("Down payment must be > 0".into());
    }
    let coc_return = (annual_cash_flow / down_payment) * 100.0;
    Ok((monthly_cash_flow, annual_cash_flow, coc_return))
}

pub fn estimate_apr(
    loan_amount: f64,
    costs: f64,
    nominal_rate: f64,
    years: f64,
) -> Result<f64, String> {
    // Simplified iterative approximation
    let net_loan = loan_amount - costs;
    let monthly_rate = (nominal_rate / 100.0) / 12.0;
    let n = years * 12.0;
    let target_payment = calculate_pmt(monthly_rate, n, loan_amount);

    let mut apr = nominal_rate;
    for _ in 0..20 {
        let r_apr = (apr / 100.0) / 12.0;
        let p_apr = calculate_pmt(r_apr, n, net_loan);
        if p_apr < target_payment {
            apr += 0.05;
        } else {
            apr -= 0.025;
        }
    }
    Ok(apr)
}

pub fn fha_loan_details(home_price: f64) -> Result<(f64, f64, f64), String> {
    let down_payment = home_price * 0.035;
    let loan_amount = home_price - down_payment;
    let monthly_mip = (loan_amount * 0.0055) / 12.0;
    Ok((down_payment, loan_amount, monthly_mip))
}

pub fn va_mortgage_details(
    price: f64,
    rate: f64,
    years: f64,
    funding_fee_percent: f64,
) -> Result<(f64, f64), String> {
    let funding_fee = price * (funding_fee_percent / 100.0);
    let total_loan = price + funding_fee;
    let monthly_rate = (rate / 100.0) / 12.0;
    let payment = calculate_pmt(monthly_rate, years * 12.0, total_loan);
    Ok((total_loan, payment))
}

pub fn home_equity_available(
    market_value: f64,
    mortgage_balance: f64,
    max_ltv_percent: f64,
) -> Result<(f64, f64), String> {
    let max_loan = market_value * (max_ltv_percent / 100.0);
    let available = (max_loan - mortgage_balance).max(0.0);
    Ok((max_loan, available))
}

pub fn heloc_interest_only(amount: f64, rate: f64) -> Result<f64, String> {
    Ok(amount * (rate / 100.0) / 12.0)
}

pub fn down_payment_required(price: f64, percent: f64) -> Result<(f64, f64), String> {
    let down = price * (percent / 100.0);
    Ok((down, price - down))
}

pub fn rent_vs_buy(
    monthly_rent: f64,
    home_price: f64,
    years: f64,
) -> Result<(f64, f64, String), String> {
    let total_rent = monthly_rent * 12.0 * years;
    let total_buy_cost = home_price + (home_price * 0.01 * years) + (home_price * 0.02); // Simplified
    let recommendation = if total_rent < total_buy_cost {
        "Rent"
    } else {
        "Buy"
    }
    .to_string();
    Ok((total_rent, total_buy_cost, recommendation))
}

pub fn credit_card_payoff(
    balance: f64,
    rate: f64,
    monthly_payment: f64,
) -> Result<(f64, f64), String> {
    let r = (rate / 100.0) / 12.0;
    if monthly_payment <= balance * r {
        return Err("Payment too low to cover interest".into());
    }
    let n = -(1.0 - (balance * r / monthly_payment)).ln() / (1.0 + r).ln();
    let total_interest = (monthly_payment * n) - balance;
    Ok((n, total_interest))
}

pub fn debt_consolidation_savings(
    total_debt: f64,
    current_avg_rate: f64,
    new_rate: f64,
    months: f64,
) -> Result<(f64, f64, f64), String> {
    let p1 = calculate_pmt((current_avg_rate / 100.0) / 12.0, months, total_debt);
    let p2 = calculate_pmt((new_rate / 100.0) / 12.0, months, total_debt);
    Ok((p1, p2, (p1 - p2) * months))
}

pub fn student_loan_payment(amount: f64, rate: f64, years: f64) -> Result<(f64, f64), String> {
    let payment = calculate_pmt((rate / 100.0) / 12.0, years * 12.0, amount);
    let total_interest = (payment * years * 12.0) - amount;
    Ok((payment, total_interest))
}

pub fn vat_calculation(amount: f64, rate: f64, add_tax: bool) -> Result<(f64, f64), String> {
    if add_tax {
        let tax = amount * (rate / 100.0);
        Ok((tax, amount + tax))
    } else {
        let base = amount / (1.0 + rate / 100.0);
        Ok((amount - base, base))
    }
}

pub fn straight_line_depreciation(
    cost: f64,
    salvage: f64,
    life_years: f64,
) -> Result<(f64, f64), String> {
    if life_years <= 0.0 {
        return Err("Life must be > 0".into());
    }
    let annual = (cost - salvage) / life_years;
    Ok((annual, annual / 12.0))
}

pub fn general_loan(amount: f64, rate: f64, months: f64) -> Result<(f64, f64), String> {
    let payment = calculate_pmt((rate / 100.0) / 12.0, months, amount);
    Ok((payment, payment * months))
}

pub fn general_lease(
    value: f64,
    residual: f64,
    months: f64,
    money_factor: f64,
) -> Result<f64, String> {
    let dep = (value - residual) / months;
    let finance = (value + residual) * money_factor;
    Ok(dep + finance)
}

pub fn commission(sales: f64, rate: f64) -> Result<f64, String> {
    Ok(sales * (rate / 100.0))
}

pub fn calculate_break_even_point(
    fixed_costs: f64,
    price_per_unit: f64,
    variable_cost_per_unit: f64,
) -> Result<f64, String> {
    let contribution_margin = price_per_unit - variable_cost_per_unit;
    if contribution_margin <= 0.0 {
        return Err("Contribution margin must be positive to break even".into());
    }
    if fixed_costs < 0.0 {
        return Err("Fixed costs cannot be negative".into());
    }
    Ok(fixed_costs / contribution_margin)
}

pub fn calculate_rule_of_72(annual_rate_percent: f64) -> Result<f64, String> {
    if annual_rate_percent <= 0.0 {
        return Err("Rate must be greater than zero".into());
    }
    Ok(72.0 / annual_rate_percent)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_business_loan() {
        let res = business_loan(10000.0, 5.0, 1.0).unwrap();
        // PMT(0.05/12, 12, 10000) = 856.07
        assert!((res.monthly_payment - 856.07).abs() < 0.1);
    }

    #[test]
    fn test_amortization() {
        let schedule = amortization_schedule(10000.0, 5.0, 1.0).unwrap();
        assert_eq!(schedule.len(), 12);
        assert!(schedule.last().unwrap().remaining_balance < 1.0);
    }
}
