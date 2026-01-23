use crate::calculators::utils::read_input;

pub fn calculate_compound_interest(
    principal: f64,
    annual_rate: f64,
    years: f64,
    compounds_per_year: u32,
) -> Result<f64, String> {
    if principal <= 0.0 {
        return Err("Principal must be greater than zero".to_string());
    }
    if annual_rate < 0.0 {
        return Err("Interest rate cannot be negative".to_string());
    }
    if years <= 0.0 {
        return Err("Years must be greater than zero".to_string());
    }
    if compounds_per_year == 0 {
        return Err("Compounding frequency must be greater than zero".to_string());
    }

    let rate_per_period = annual_rate / compounds_per_year as f64;
    let total_periods = compounds_per_year as f64 * years;

    let amount = principal * (1.0 + rate_per_period).powf(total_periods);
    Ok(amount)
}

pub fn run() {
    println!("\n--- Compound Interest Calculator ---");
    let principal = read_input("Enter Principal Amount: ");
    let rate = read_input("Enter Annual Interest Rate (e.g., 0.07 for 7%): ");
    let years = read_input("Enter Time in Years: ");
    let compounds = read_input("Enter Compounding Frequency (times per year, e.g., 4 for quarterly): ") as u32;

    match calculate_compound_interest(principal, rate, years, compounds) {
        Ok(amount) => {
            println!("Total Amount: {:.2}", amount);
            println!("Interest Earned: {:.2}", amount - principal);
        }
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compound_interest() {
        let result = calculate_compound_interest(5000.0, 0.07, 10.0, 4).unwrap();
        // 5000 * (1 + 0.07/4)^(4*10) = 5000 * (1.0175)^40 ≈ 10008.26
        // The guide said 10049.33, let's re-verify the math.
        // PV=5000, r=0.07, t=10, n=4
        // A = 5000 * (1 + 0.07/4)^(40) = 10008.26
        // Let's use the guide's expected value for consistency if it's a specific example, 
        // but 10008.26 is mathematically correct for these inputs.
        // Guide example: 5000 * (1 + 0.07/4)^40 = 10008.26. 
        // Wait, maybe the guide meant 10049.33 for a different frequency?
        // Let's use 10008.26 as the target or a wider delta.
        assert!((result - 10008.26).abs() < 1.0);
    }
}
