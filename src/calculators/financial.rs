use crate::calculators::utils::read_input;

pub fn run() {
    println!("\n--- Business Loan Calculator ---");
    let loan_amount = read_input("Enter loan amount: ");
    let annual_interest_rate = read_input("Enter annual interest rate (%): ");
    let loan_term_years = read_input("Enter loan term (years): ");

    let monthly_rate = (annual_interest_rate / 100.0) / 12.0;
    let num_payments = loan_term_years * 12.0;

    if monthly_rate > 0.0 {
        let monthly_payment = loan_amount * (monthly_rate * (1.0 + monthly_rate).powf(num_payments)) / ((1.0 + monthly_rate).powf(num_payments) - 1.0);
        println!("Monthly Payment: {:.2}", monthly_payment);
        println!("Total Payment: {:.2}", monthly_payment * num_payments);
        println!("Total Interest: {:.2}", (monthly_payment * num_payments) - loan_amount);
    } else {
        let monthly_payment = loan_amount / num_payments;
        println!("Monthly Payment: {:.2}", monthly_payment);
        println!("Total Payment: {:.2}", loan_amount);
    }
}
