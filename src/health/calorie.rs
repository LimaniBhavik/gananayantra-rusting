use crate::calculators::utils::read_input;

pub fn run() {
    println!("\n--- Calorie Calculator ---");
    println!("First, we need your BMR.");
    let gender = crate::calculators::utils::read_string("Enter gender (m/f): ");
    let weight = read_input("Enter weight (kg): ");
    let height = read_input("Enter height (cm): ");
    let age = read_input("Enter age: ");

    let bmr = if gender.to_lowercase() == "m" {
        10.0 * weight + 6.25 * height - 5.0 * age + 5.0
    } else {
        10.0 * weight + 6.25 * height - 5.0 * age - 161.0
    };

    println!("\nActivity Level:");
    println!("1. Sedentary (little or no exercise)");
    println!("2. Lightly active (1-3 days/week)");
    println!("3. Moderately active (3-5 days/week)");
    println!("4. Very active (6-7 days/week)");
    println!("5. Extra active (very hard exercise/physical job)");
    
    let activity = read_input("Select activity level (1-5): ");
    let factor = match activity as i32 {
        1 => 1.2,
        2 => 1.375,
        3 => 1.55,
        4 => 1.725,
        5 => 1.9,
        _ => 1.2,
    };

    let tdee = bmr * factor;
    println!("\nYour Daily Maintenance Calories: {:.0} kcal", tdee);
    println!("To lose weight (0.5kg/week): {:.0} kcal", tdee - 500.0);
    println!("To gain weight (0.5kg/week): {:.0} kcal", tdee + 500.0);
}
