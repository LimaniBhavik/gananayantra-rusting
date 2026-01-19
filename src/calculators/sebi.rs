use crate::calculators::utils::read_input;

pub fn run_menu() {
    loop {
        println!("\n--- SEBI Investor Tools ---");
        println!("1. SEBI Net Worth Calculator");
        println!("0. Back");
        let choice = read_input("Select an option: ");

        match choice as i32 {
            1 => net_worth_calc(),
            0 => break,
            _ => println!("Invalid choice."),
        }
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
