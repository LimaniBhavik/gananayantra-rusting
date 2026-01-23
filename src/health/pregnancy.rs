use crate::calculators::utils::read_input;

pub fn run() {
    println!("\n--- Pregnancy Calculator ---");
    let weeks = read_input("Enter current weeks of pregnancy: ");
    let days = read_input("Enter additional days: ");
    
    let total_days = weeks * 7.0 + days;
    let remaining = 280.0 - total_days;
    
    if remaining > 0.0 {
        println!("You are approximately {:.0} weeks and {:.0} days pregnant.", (total_days / 7.0).floor(), total_days % 7.0);
        println!("Estimated days until birth: {:.0}", remaining);
    } else {
        println!("You are past the average 40-week gestation period.");
    }
}
