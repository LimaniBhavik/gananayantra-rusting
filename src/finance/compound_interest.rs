/// Calculates the final amount using compound interest.
///
/// # Arguments
/// * `principal` - Initial amount invested
/// * `annual_rate` - Annual interest rate (decimal, e.g. 0.07 for 7%)
/// * `years` - Duration of investment in years
/// * `compounds_per_year` - Number of compounding periods per year
///
/// # Returns
/// * Final accumulated amount
pub fn compound_interest(
    principal: f64,
    annual_rate: f64,
    years: f64,
    compounds_per_year: u32,
) -> Result<f64, String> {
    if principal <= 0.0 {
        return Err("Principal must be greater than zero".into());
    }
    if annual_rate < 0.0 {
        return Err("Annual rate cannot be negative".into());
    }
    if years <= 0.0 {
        return Err("Years must be greater than zero".into());
    }
    if compounds_per_year == 0 {
        return Err("Compounds per year must be greater than zero".into());
    }

    let rate_per_period = annual_rate / compounds_per_year as f64;
    let total_periods = compounds_per_year as f64 * years;

    Ok(principal * (1.0 + rate_per_period).powf(total_periods))
}

pub fn run() {
    println!("\n--- Compound Interest Calculator ---");
    use crate::calculators::utils::read_input;
    let principal = read_input("Enter Principal Amount: ");
    let rate = read_input("Enter Annual Interest Rate (e.g., 0.07 for 7%): ");
    let years = read_input("Enter Time in Years: ");
    let compounds = read_input("Enter Compounding Frequency (times per year, e.g., 4 for quarterly): ") as u32;

    match compound_interest(principal, rate, years, compounds) {
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
        let result = compound_interest(5_000.0, 0.07, 10.0, 4).unwrap();
        // A = 5000 * (1 + 0.07/4)^(4*10) = 5000 * (1.0175)^40 = 10008.26
        assert!((result - 10008.26).abs() < 1.0);
    }

    #[test]
    fn test_invalid_principal() {
        assert!(compound_interest(0.0, 0.07, 10.0, 4).is_err());
    }
}
