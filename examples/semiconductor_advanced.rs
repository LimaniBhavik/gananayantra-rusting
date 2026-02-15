use gananayantra::semiconductor::lithography::{calculate_resolution, calculate_dof};
use gananayantra::semiconductor::moore::predict_transistor_count;

fn main() {
    println!("--- Advanced Semiconductor Examples ---");

    // Lithography (EUV)
    let wavelength = 13.5; // nm
    let na = 0.33;
    let k1 = 0.3;
    match calculate_resolution(wavelength, na, k1) {
        Ok(res) => println!("EUV Resolution (NA=0.33): {:.2} nm", res),
        Err(e) => println!("Error: {}", e),
    }

    // Moore's Law
    let current_transistors = 50_000_000_000.0; // 50 Billion
    let years = 4.0;
    match predict_transistor_count(current_transistors, years) {
        Ok(count) => println!("Transistor Count in 4 years: {:.2e}", count),
        Err(e) => println!("Error: {}", e),
    }
}
