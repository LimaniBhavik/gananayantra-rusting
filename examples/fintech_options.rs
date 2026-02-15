use gananayantra::finance::options::{calculate_call_option_price, calculate_greeks};

fn main() {
    println!("--- Fintech: Options & Greeks ---");

    let s = 100.0;   // Stock Price
    let k = 100.0;   // Strike Price
    let t = 1.0;     // Time to maturity (years)
    let r = 0.05;    // Risk-free rate (5%)
    let sigma = 0.2; // Volatility (20%)

    match calculate_call_option_price(s, k, t, r, sigma) {
        Ok(price) => println!("Call Option Price (Black-Scholes): ${:.2}", price),
        Err(e) => println!("Error: {}", e),
    }

    match calculate_greeks(s, k, t, r, sigma) {
        Ok(greeks) => {
            println!("Delta: {:.4}", greeks.delta);
            println!("Gamma: {:.4}", greeks.gamma);
            println!("Theta: {:.4}", greeks.theta);
        },
        Err(e) => println!("Error: {}", e),
    }
}
