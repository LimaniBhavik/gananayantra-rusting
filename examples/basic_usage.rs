use gananayantra_rusting::finance::tvm::future_value;

fn main() {
    println!("--- Gananayantra Library Usage Example ---");
    
    let pv = 10000.0;
    let rate = 0.08;
    let years = 5;

    match future_value(pv, rate, years) {
        Ok(fv) => {
            println!("Present Value: ${:.2}", pv);
            println!("Annual Rate: {}%", rate * 100.0);
            println!("Years: {}", years);
            println!("Future Value: ${:.2}", fv);
        }
        Err(e) => println!("Error calculating future value: {}", e),
    }
}
