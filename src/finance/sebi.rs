pub struct NetWorth {
    pub total_assets: f64,
    pub total_liabilities: f64,
    pub net_worth: f64,
    pub debt_to_asset_ratio: f64,
}

pub fn calculate_net_worth(assets: f64, liabilities: f64) -> Result<NetWorth, String> {
    if assets < 0.0 || liabilities < 0.0 {
        return Err("Values must be positive".into());
    }
    let net_worth = assets - liabilities;
    let dti = if assets > 0.0 {
        liabilities / assets
    } else {
        0.0
    };
    Ok(NetWorth {
        total_assets: assets,
        total_liabilities: liabilities,
        net_worth,
        debt_to_asset_ratio: dti,
    })
}

pub fn sip_future_value(
    monthly_investment: f64,
    expected_return_percent: f64,
    years: f64,
) -> Result<f64, String> {
    let r = expected_return_percent / 100.0 / 12.0;
    let n = years * 12.0;
    Ok(monthly_investment * (((1.0 + r).powf(n) - 1.0) / r) * (1.0 + r))
}

pub fn sip_required_monthly(
    goal_amount: f64,
    expected_return_percent: f64,
    years: f64,
) -> Result<f64, String> {
    let r = expected_return_percent / 100.0 / 12.0;
    let n = years * 12.0;
    Ok(goal_amount / ((((1.0 + r).powf(n) - 1.0) / r) * (1.0 + r)))
}

pub fn loan_emi(principal: f64, rate_percent: f64, years: f64) -> Result<f64, String> {
    let r = rate_percent / 100.0 / 12.0;
    let n = years * 12.0;
    Ok(principal * r * (1.0 + r).powf(n) / ((1.0 + r).powf(n) - 1.0))
}

pub fn cagr(initial_value: f64, final_value: f64, years: f64) -> Result<f64, String> {
    if initial_value <= 0.0 || years <= 0.0 {
        return Err("Invalid inputs".into());
    }
    Ok(((final_value / initial_value).powf(1.0 / years) - 1.0) * 100.0)
}

pub fn future_inflation_cost(amount: f64, inflation_rate: f64, years: f64) -> Result<f64, String> {
    Ok(amount / (1.0 + inflation_rate / 100.0).powf(years))
}

pub fn retirement_corpus_needed(
    current_expenses: f64,
    inflation_rate: f64,
    years_to_retire: f64,
) -> Result<f64, String> {
    Ok(current_expenses * (1.0 + inflation_rate / 100.0).powf(years_to_retire))
}

pub fn insurance_needs(monthly_expenses: f64, liabilities: f64) -> Result<f64, String> {
    Ok((monthly_expenses * 12.0 * 15.0) + liabilities)
}

pub fn asset_allocation(age: f64) -> Result<(f64, f64), String> {
    let equity = (100.0 - age).max(0.0);
    Ok((equity, 100.0 - equity))
}

pub fn cost_of_delay(monthly_sip: f64, years: f64, rate_diff: f64) -> Result<f64, String> {
    // Calculates difference in corpus if return is 1% less (or rate_diff less)
    // Actually the original code hardcoded 12% vs 11%.
    // I will generalize it.
    let r1: f64 = 12.0 / 100.0 / 12.0;
    let r2 = (12.0 - rate_diff) / 100.0 / 12.0;
    let n = years * 12.0;

    let fv1 = monthly_sip * (((1.0 + r1).powf(n) - 1.0) / r1);
    let fv2 = monthly_sip * (((1.0 + r2).powf(n) - 1.0) / r2);
    Ok(fv1 - fv2)
}
