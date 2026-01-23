use crate::calculators::utils::read_input;

pub fn run_menu() {
    loop {
        println!("\n--- Fitness & Health (Expanded) ---");
        println!("1. Healthy Weight Calculator");
        println!("2. Calories Burned Calculator");
        println!("3. One Rep Max Calculator");
        println!("4. Target Heart Rate Calculator");
        println!("5. Macro Calculator (Protein/Carb/Fat)");
        println!("6. TDEE Calculator");
        println!("7. Ovulation & Period Calculator");
        println!("8. BAC Calculator");
        println!("9. Body Surface Area (BSA)");
        println!("10. Lean Body Mass");
        println!("0. Back");
        let choice = read_input("Select an option: ");

        match choice as i32 {
            1 => healthy_weight(),
            2 => calories_burned(),
            3 => one_rep_max(),
            4 => target_heart_rate(),
            5 => macro_calculator(),
            6 => tdee_calculator(),
            7 => ovulation_period(),
            8 => bac_calculator(),
            9 => bsa_calculator(),
            10 => lean_body_mass(),
            0 => break,
            _ => println!("Invalid choice."),
        }
    }
}

fn healthy_weight() {
    println!("\n--- Healthy Weight Calculator ---");
    let height_cm = read_input("Height (cm): ");
    // BMI 18.5 to 25
    let low = 18.5 * (height_cm / 100.0).powi(2);
    let high = 25.0 * (height_cm / 100.0).powi(2);
    println!("Healthy Weight Range: {:.2}kg - {:.2}kg", low, high);
}

fn calories_burned() {
    println!("\n--- Calories Burned Calculator ---");
    let weight = read_input("Weight (kg): ");
    let duration = read_input("Duration (minutes): ");
    println!("Activity MET (e.g., Running 8mph = 11.5, Walking 3mph = 3.5): ");
    let met = read_input("MET Value: ");
    
    let burned = (duration * met * 3.5 * weight) / 200.0;
    println!("Estimated Calories Burned: {:.2} kcal", burned);
}

fn one_rep_max() {
    println!("\n--- One Rep Max Calculator ---");
    let weight = read_input("Weight Lifted: ");
    let reps = read_input("Reps Performed: ");
    
    // Epley formula
    let orm = weight * (1.0 + reps / 30.0);
    println!("Estimated 1RM: {:.2}", orm);
}

fn target_heart_rate() {
    println!("\n--- Target Heart Rate Calculator ---");
    let age = read_input("Age: ");
    let resting_hr = read_input("Resting Heart Rate: ");
    
    let max_hr = 220.0 - age;
    let hrr = max_hr - resting_hr;
    
    let low = hrr * 0.5 + resting_hr;
    let high = hrr * 0.85 + resting_hr;
    
    println!("Max Heart Rate: {:.0}", max_hr);
    println!("Target Zone (50%-85%): {:.0} - {:.0} bpm", low, high);
}

fn macro_calculator() {
    println!("\n--- Macro Calculator ---");
    let tdee = read_input("Daily Calories (TDEE): ");
    println!("1. Balanced (40/30/30)");
    println!("2. Low Carb (20/40/40)");
    println!("3. High Carb (60/20/20)");
    let goal = read_input("Choice: ");
    
    let (p, c, f) = match goal as i32 {
        2 => (0.4, 0.2, 0.4),
        3 => (0.2, 0.6, 0.2),
        _ => (0.3, 0.4, 0.3),
    };
    
    println!("Protein: {:.0}g", (tdee * p) / 4.0);
    println!("Carbs: {:.0}g", (tdee * c) / 4.0);
    println!("Fats: {:.0}g", (tdee * f) / 9.0);
}

fn tdee_calculator() {
    println!("\n--- TDEE Calculator ---");
    let bmr = read_input("BMR (Basal Metabolic Rate): ");
    println!("Activity Level:");
    println!("1. Sedentary (1.2)");
    println!("2. Light (1.375)");
    println!("3. Moderate (1.55)");
    println!("4. Heavy (1.725)");
    println!("5. Athlete (1.9)");
    let factor = match read_input("Choice: ") as i32 {
        2 => 1.375,
        3 => 1.55,
        4 => 1.725,
        5 => 1.9,
        _ => 1.2,
    };
    println!("Daily TDEE: {:.0} kcal", bmr * factor);
}

fn ovulation_period() {
    println!("\n--- Ovulation & Period Calculator ---");
    let cycle = read_input("Average Cycle Length (days, e.g., 28): ");
    println!("Ovulation typically occurs around day {:.0} of your cycle.", cycle - 14.0);
    println!("Fertile window: Days {:.0} to {:.0}.", cycle - 17.0, cycle - 12.0);
}

fn bac_calculator() {
    println!("\n--- BAC Calculator (Widmark Formula) ---");
    let weight = read_input("Weight (kg): ");
    let gender = read_input("Gender (1 for Male, 2 for Female): ");
    let alcohol_grams = read_input("Alcohol Consumed (grams): ");
    let hours = read_input("Hours since first drink: ");
    
    let r = if gender == 1.0 { 0.68 } else { 0.55 };
    let bac = (alcohol_grams / (weight * 1000.0 * r)) * 100.0 - (hours * 0.015);
    println!("Estimated BAC: {:.3}%", bac.max(0.0));
}

fn bsa_calculator() {
    println!("\n--- Body Surface Area (Mosteller) ---");
    let h = read_input("Height (cm): ");
    let w = read_input("Weight (kg): ");
    let bsa = ((h * w) / 3600.0).sqrt();
    println!("Estimated BSA: {:.2} m²", bsa);
}

fn lean_body_mass() {
    println!("\n--- Lean Body Mass (Boer) ---");
    let h = read_input("Height (cm): ");
    let w = read_input("Weight (kg): ");
    let gender = read_input("Gender (1 for Male, 2 for Female): ");
    
    let lbm = if gender == 1.0 {
        0.407 * w + 0.267 * h - 19.2
    } else {
        0.252 * w + 0.473 * h - 48.3
    };
    println!("Estimated Lean Body Mass: {:.2} kg", lbm);
}
