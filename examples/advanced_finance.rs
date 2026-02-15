use gananayantra::finance::quant::{calculate_sharpe_ratio, calculate_var_parametric, monte_carlo_simulation};

fn main() {
    println!("--- Advanced Quantitative Finance ---");

    // Sharpe Ratio
    let ret = 0.12;
    let rf = 0.04;
    let vol = 0.15;
    match calculate_sharpe_ratio(ret, rf, vol) {
        Ok(sr) => println!("Sharpe Ratio: {:.2}", sr),
        Err(e) => println!("Error: {}", e),
    }

    // VaR
    let portfolio = 1_000_000.0;
    let conf = 0.95;
    match calculate_var_parametric(portfolio, vol, conf) {
        Ok(var) => println!("Value at Risk (95%): ${:.2}", var),
        Err(e) => println!("Error: {}", e),
    }

    // Monte Carlo
    println!("Running Monte Carlo Simulation (1000 paths)...");
    let seed = 42; // Deterministic seed for example
    match monte_carlo_simulation(100.0, 0.08, 0.20, 10, 1000, seed) {
        Ok(paths) => {
            let avg_final: f64 = paths.iter().sum::<f64>() / paths.len() as f64;
            println!("Average Portfolio Value after 10 years: ${:.2}", avg_final);
        },
        Err(e) => println!("Error: {}", e),
    }
}
