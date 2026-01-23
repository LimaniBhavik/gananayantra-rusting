use crate::calculators::utils::read_input;

pub fn run() {
    println!("\n--- Ideal Weight Calculator (Devine Formula) ---");
    let gender = crate::calculators::utils::read_string("Enter gender (m/f): ");
    let height_cm = read_input("Enter height (cm): ");
    let height_in = height_cm / 2.54;
    
    if height_in > 60.0 {
        let base_weight = if gender.to_lowercase() == "m" { 50.0 } else { 45.5 };
        let ideal = base_weight + 2.3 * (height_in - 60.0);
        println!("Your ideal body weight is approximately: {:.1} kg", ideal);
    } else {
        println!("Formula is applicable for heights over 5 feet (152.4 cm).");
    }
}
