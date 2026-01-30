/// Calculates auto loan payments and costs.
///
/// # Arguments
/// * `price` - Vehicle price
/// * `down_payment` - Down payment amount
/// * `trade_in` - Trade-in value
/// * `term_months` - Loan term in months
/// * `annual_interest_rate` - Annual interest rate (percentage, e.g., 5.0 for 5%)
///
/// # Returns
/// * `AutoLoanResult` struct containing monthly payment, total interest, and total cost.
pub fn calculate_auto_loan(
    price: f64,
    down_payment: f64,
    trade_in: f64,
    term_months: f64,
    annual_interest_rate: f64,
) -> Result<AutoLoanResult, String> {
    if price < 0.0
        || down_payment < 0.0
        || trade_in < 0.0
        || term_months <= 0.0
        || annual_interest_rate < 0.0
    {
        return Err("Inputs must be non-negative, and term must be positive.".into());
    }

    let loan_amount = price - down_payment - trade_in;
    if loan_amount <= 0.0 {
        return Ok(AutoLoanResult {
            monthly_payment: 0.0,
            total_interest: 0.0,
            total_cost: price, // Or just down_payment + trade_in? Technically total cost of car is price.
        });
    }

    let monthly_rate = (annual_interest_rate / 100.0) / 12.0;

    let monthly_payment = if monthly_rate > 0.0 {
        loan_amount * (monthly_rate * (1.0 + monthly_rate).powf(term_months))
            / ((1.0 + monthly_rate).powf(term_months) - 1.0)
    } else {
        loan_amount / term_months
    };

    let total_payment = monthly_payment * term_months;
    let total_interest = total_payment - loan_amount;
    let total_cost = total_payment + down_payment + trade_in;

    Ok(AutoLoanResult {
        monthly_payment,
        total_interest,
        total_cost,
    })
}

#[derive(Debug, PartialEq)]
pub struct AutoLoanResult {
    pub monthly_payment: f64,
    pub total_interest: f64,
    pub total_cost: f64,
}

/// Compares "Cash Back" vs "Low Interest" auto financing options.
///
/// # Returns
/// * `CashBackComparison` struct with details for both options.
pub fn compare_cash_back_vs_low_interest(
    price: f64,
    cash_back: f64,
    rate_with_cash_back: f64,
    low_interest_rate: f64,
    term_months: f64,
) -> Result<CashBackComparison, String> {
    if price < 0.0 || term_months <= 0.0 {
        return Err("Invalid input values.".into());
    }

    // Option 1: Cash Back
    let loan1 = price - cash_back;
    let r1 = (rate_with_cash_back / 100.0) / 12.0;
    let p1 = if r1 > 0.0 {
        loan1 * (r1 * (1.0 + r1).powf(term_months)) / ((1.0 + r1).powf(term_months) - 1.0)
    } else {
        loan1 / term_months
    };
    let total1 = p1 * term_months;

    // Option 2: Low Interest
    let loan2 = price;
    let r2 = (low_interest_rate / 100.0) / 12.0;
    let p2 = if r2 > 0.0 {
        loan2 * (r2 * (1.0 + r2).powf(term_months)) / ((1.0 + r2).powf(term_months) - 1.0)
    } else {
        loan2 / term_months
    };
    let total2 = p2 * term_months;

    Ok(CashBackComparison {
        cash_back_option: LoanOption {
            monthly_payment: p1,
            total_cost: total1,
        },
        low_interest_option: LoanOption {
            monthly_payment: p2,
            total_cost: total2,
        },
        savings_amount: (total1 - total2).abs(),
        better_option: if total1 < total2 {
            "Cash Back"
        } else {
            "Low Interest"
        }
        .to_string(),
    })
}

#[derive(Debug, PartialEq)]
pub struct CashBackComparison {
    pub cash_back_option: LoanOption,
    pub low_interest_option: LoanOption,
    pub savings_amount: f64,
    pub better_option: String,
}

#[derive(Debug, PartialEq)]
pub struct LoanOption {
    pub monthly_payment: f64,
    pub total_cost: f64,
}

/// Calculates auto lease payments.
pub fn calculate_auto_lease(
    capitalized_cost: f64,
    residual_value: f64,
    money_factor: f64,
    term_months: f64,
    tax_rate: f64,
) -> Result<AutoLeaseResult, String> {
    if term_months <= 0.0 {
        return Err("Term must be positive.".into());
    }

    let monthly_depreciation = (capitalized_cost - residual_value) / term_months;
    let monthly_finance = (capitalized_cost + residual_value) * money_factor;
    let base_payment = monthly_depreciation + monthly_finance;
    let tax = base_payment * (tax_rate / 100.0);
    let total_monthly = base_payment + tax;

    Ok(AutoLeaseResult {
        monthly_depreciation,
        monthly_finance,
        base_payment,
        total_monthly_payment: total_monthly,
    })
}

#[derive(Debug, PartialEq)]
pub struct AutoLeaseResult {
    pub monthly_depreciation: f64,
    pub monthly_finance: f64,
    pub base_payment: f64,
    pub total_monthly_payment: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auto_loan() {
        let res = calculate_auto_loan(30000.0, 5000.0, 0.0, 60.0, 5.0).unwrap();
        // Loan 25000, 5%, 60 months
        // Payment ~ 471.78
        assert!((res.monthly_payment - 471.78).abs() < 0.1);
    }

    #[test]
    fn test_lease() {
        // Cap cost 30000, residual 15000, MF 0.00125 (3%), 36 months, tax 8%
        let res = calculate_auto_lease(30000.0, 15000.0, 0.00125, 36.0, 8.0).unwrap();
        // Dep: 15000/36 = 416.67
        // Fin: 45000 * 0.00125 = 56.25
        // Base: 472.92
        // Tax: 472.92 * 0.08 = 37.83
        // Total: 510.75
        assert!((res.total_monthly_payment - 510.75).abs() < 0.1);
    }
}
