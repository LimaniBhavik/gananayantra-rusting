use crate::calculators::utils::read_input;

pub fn run_menu() {
    loop {
        println!("\n--- SEBI Investor Tools ---");
        println!("1. SEBI Net Worth Calculator");
        println!("2. SIP & Goal SIP Calculator");
        println!("3. EMI Calculator");
        println!("4. Rate of Return & Bond Yield");
        println!("5. Future & Present Value (TVM)");
        println!("6. Retirement Planning (Investment & Income)");
        println!("7. Insurance Needs Calculator");
        println!("8. Goal Planner & Asset Allocation");
        println!("9. Power of Compounding & Cost of Delay");
        println!("0. Back");
        let choice = read_input("Select an option: ");

        match choice as i32 {
            1 => net_worth_calc(),
            2 => sip_menu(),
            3 => emi_calc(),
            4 => return_menu(),
            5 => tvm_menu(),
            6 => retirement_menu(),
            7 => insurance_menu(),
            8 => goal_menu(),
            9 => compounding_menu(),
            0 => break,
            _ => println!("Invalid choice."),
        }
    }
}

fn sip_menu() {
    println!("\n--- Mutual Fund (SIP) Calculators ---");
    println!("1. Standard SIP");
    println!("2. Goal-Based SIP");
    let c = read_input("Choice: ");
    if c == 1.0 {
        let p = read_input("Monthly SIP: ");
        let r = read_input("Expected Return (%): ") / 100.0 / 12.0;
        let n = read_input("Years: ") * 12.0;
        let fv = p * (((1.0 + r).powf(n) - 1.0) / r) * (1.0 + r);
        println!("Future Value of SIP: {:.2}", fv);
    } else {
        let goal = read_input("Target Goal: ");
        let r = read_input("Expected Return (%): ") / 100.0 / 12.0;
        let n = read_input("Years: ") * 12.0;
        let p = goal / ((((1.0 + r).powf(n) - 1.0) / r) * (1.0 + r));
        println!("Monthly SIP Required: {:.2}", p);
    }
}

fn emi_calc() {
    let p = read_input("Loan Amount: ");
    let r = read_input("Rate of Interest (%): ") / 100.0 / 12.0;
    let n = read_input("Tenure (Years): ") * 12.0;
    let emi = p * r * (1.0 + r).powf(n) / ((1.0 + r).powf(n) - 1.0);
    println!("Monthly EMI: {:.2}", emi);
}

fn return_menu() {
    println!("1. Rate of Return  2. Bond Yield");
    let c = read_input("Choice: ");
    if c == 1.0 {
        let initial = read_input("Initial: ");
        let final_val = read_input("Final: ");
        let years = read_input("Years: ");
        let cagr = ((final_val / initial).powf(1.0 / years) - 1.0) * 100.0;
        println!("Annualized Return (CAGR): {:.2}%", cagr);
    } else {
        let coupon = read_input("Annual Coupon: ");
        let market_price = read_input("Market Price: ");
        println!("Current Yield: {:.2}%", (coupon / market_price) * 100.0);
    }
}

fn tvm_menu() {
    println!("1. Future Value  2. Present Value  3. Inflation Impact");
    let c = read_input("Choice: ");
    let rate = read_input("Rate/Inflation (%): ") / 100.0;
    let n = read_input("Years: ");
    if c == 1.0 {
        let pv = read_input("Present Value: ");
        println!("Future Value: {:.2}", pv * (1.0 + rate).powf(n));
    } else if c == 2.0 {
        let fv = read_input("Future Value: ");
        println!("Present Value: {:.2}", fv / (1.0 + rate).powf(n));
    } else {
        let amount = read_input("Amount Today: ");
        println!("Value in {} years: {:.2}", n, amount / (1.0 + rate).powf(n));
    }
}

fn retirement_menu() {
    println!("1. Retirement Tracker  2. Annual Income Needed");
    let c = read_input("Choice: ");
    if c == 1.0 {
        let current = read_input("Current Savings: ");
        let monthly = read_input("Monthly Contribution: ");
        let r = read_input("Return Rate (%): ") / 100.0 / 12.0;
        let n = read_input("Years to Retire: ") * 12.0;
        let fv = current * (1.0 + r).powf(n) + monthly * (((1.0 + r).powf(n) - 1.0) / r);
        println!("Projected Corpus: {:.2}", fv);
    } else {
        let expenses = read_input("Current Monthly Expenses: ");
        let inflation = read_input("Inflation Rate (%): ") / 100.0;
        let n = read_input("Years to Retire: ");
        let future_exp = expenses * (1.0 + inflation).powf(n);
        println!("Monthly Income Needed at Retirement: {:.2}", future_exp);
    }
}

fn insurance_menu() {
    println!("--- Insurance Needs ---");
    let monthly_exp = read_input("Monthly Expenses: ");
    let liabilities = read_input("Total Liabilities: ");
    let cover = (monthly_exp * 12.0 * 15.0) + liabilities; // 15x income + debt
    println!("Suggested Life Insurance Cover: {:.2}", cover);
}

fn goal_menu() {
    println!("1. Visual Goal Planner  2. Asset Allocation");
    let c = read_input("Choice: ");
    if c == 1.0 {
        let goal_amount = read_input("Target Amount: ");
        let years = read_input("Years: ");
        let r = read_input("Return (%): ") / 100.0 / 12.0;
        let monthly = goal_amount / ((((1.0 + r).powf(years * 12.0) - 1.0) / r) * (1.0 + r));
        println!("You need to save {:.2} monthly for {} years.", monthly, years);
    } else {
        let age = read_input("Current Age: ");
        let equity = 100.0 - age;
        println!("Suggested Asset Allocation: Equity {}%, Debt {}%", equity, 100.0 - equity);
    }
}

fn compounding_menu() {
    println!("1. Power of Compounding  2. Cost of Delay (1% Impact)");
    let c = read_input("Choice: ");
    if c == 1.0 {
        let p = read_input("Principal: ");
        let r = read_input("Rate (%): ") / 100.0;
        let t = read_input("Years: ");
        println!("Value after {} years: {:.2}", t, p * (1.0 + r).powf(t));
    } else {
        let p = read_input("Monthly SIP: ");
        let years = read_input("Years: ");
        let r1: f64 = 12.0 / 100.0 / 12.0;
        let r2: f64 = 11.0 / 100.0 / 12.0;
        let n_calc: f64 = years * 12.0;
        let fv1 = p * (((1.0 + r1).powf(n_calc) - 1.0) / r1);
        let fv2 = p * (((1.0 + r2).powf(n_calc) - 1.0) / r2);
        println!("Corpus at 12%: {:.2}", fv1);
        println!("Corpus at 11% (1% less): {:.2}", fv2);
        println!("Impact of 1% difference: {:.2}", fv1 - fv2);
    }
}

fn net_worth_calc() {
    println!("\n--- SEBI Net Worth Calculator ---");
    
    println!("\n[1] Liquid Assets");
    let savings = read_input("Savings Account: ");
    let current = read_input("Current Account: ");
    let liquid_mf = read_input("Liquid Mutual Funds: ");
    let fd = read_input("Fixed Deposits: ");
    let rd = read_input("Recurring Deposits: ");
    let corp_dep = read_input("Corporate Deposits: ");
    let post_off = read_input("Post Office Deposits: ");
    let shares = read_input("Shares: ");
    let equity_mf = read_input("Equity Mutual Funds: ");
    let debentures = read_input("Debentures: ");
    let annuities = read_input("Annuities: ");
    let pension = read_input("Pension fund value: ");
    let insurance = read_input("Insurance policies (except term): ");
    let gold = read_input("Gold, Silver, Jewels: ");
    let art = read_input("Art / Antiques: ");
    let other_liq = read_input("Other Liquid Assets: ");

    let liquid_total = savings + current + liquid_mf + fd + rd + corp_dep + post_off + shares + equity_mf + debentures + annuities + pension + insurance + gold + art + other_liq;

    println!("\n[2] Partial Liquid Assets");
    let ppf_self = read_input("PPF of Self: ");
    let ppf_spouse = read_input("PPF of Spouse: ");
    let ppf_children = read_input("PPF of Children: ");
    let epf = read_input("EPF: ");
    let nps = read_input("NPS (tier 1 + 2): ");
    let bonds = read_input("Bonds: ");

    let partial_liquid_total = ppf_self + ppf_spouse + ppf_children + epf + nps + bonds;

    println!("\n[3] Illiquid Assets");
    let nsc = read_input("NSC: ");
    let kvp = read_input("KVP: ");
    let business = read_input("Business / Partnership: ");
    let real_estate = read_input("Real Estate (non-residential): ");

    let illiquid_total = nsc + kvp + business + real_estate;

    let total_assets = liquid_total + partial_liquid_total + illiquid_total;

    println!("\n[4] Total Liabilities");
    let home_loan = read_input("Home Loan: ");
    let car_loan = read_input("Car Loan: ");
    let personal_loan = read_input("Personal Loan: ");
    let other_loans = read_input("Other Loans: ");
    let taxes_due = read_input("Taxes Due: ");
    let credit_card = read_input("Credit Card Due: ");
    let other_bills = read_input("Other bills Outstanding: ");

    let total_liabilities = home_loan + car_loan + personal_loan + other_loans + taxes_due + credit_card + other_bills;

    let net_worth = total_assets - total_liabilities;
    let dti = if total_assets > 0.0 { total_liabilities / total_assets } else { 0.0 };

    println!("\n--- RESULTS ---");
    println!("Total Assets: {:.2}", total_assets);
    println!("Total Liabilities: {:.2}", total_liabilities);
    println!("Your Net Worth is: {:.2}", net_worth);
    println!("Debt to Asset ratio: {:.2}", dti);

    println!("\nAsset Breakdown:");
    println!("Liquid Assets: {:.2} ({:.1}%)", liquid_total, (liquid_total / total_assets) * 100.0);
    println!("Partial Liquid: {:.2} ({:.1}%)", partial_liquid_total, (partial_liquid_total / total_assets) * 100.0);
    println!("Illiquid Assets: {:.2} ({:.1}%)", illiquid_total, (illiquid_total / total_assets) * 100.0);
    
    println!("\nDisclaimer: These calculators are for illustrations only and do not represent actual returns.");
}
