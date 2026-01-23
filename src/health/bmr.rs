use crate::calculators::utils::read_input;

pub fn run() {
    println!("\n--- BMR Calculator (Mifflin-St Jeor Equation) ---");
    let gender = crate::calculators::utils::read_string("Enter gender (m/f): ");
    let weight = read_input("Enter weight (kg): ");
    let height = read_input("Enter height (cm): ");
    let age = read_input("Enter age: ");

    let bmr = if gender.to_lowercase() == "m" {
        10.0 * weight + 6.25 * height - 5.0 * age + 5.0
    } else {
        10.0 * weight + 6.25 * height - 5.0 * age - 161.0
    };

    println!("Your BMR is: {:.2} kcal/day", bmr);
}
