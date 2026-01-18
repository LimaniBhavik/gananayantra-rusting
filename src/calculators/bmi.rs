use crate::calculators::utils::read_input;

pub fn run() {
    println!("\n--- BMI Calculator ---");
    let weight = read_input("Enter weight (kg): ");
    let height = read_input("Enter height (cm): ") / 100.0;

    if height > 0.0 {
        let bmi = weight / (height * height);
        println!("Your BMI is: {:.2}", bmi);
        
        let category = if bmi < 18.5 {
            "Underweight"
        } else if bmi < 25.0 {
            "Normal weight"
        } else if bmi < 30.0 {
            "Overweight"
        } else {
            "Obese"
        };
        println!("WHO Category: {}", category);
    }
}
