use crate::calculators::utils::read_input;
use std::io::{self, Write};

/// Calculates Body Mass Index (BMI)
/// 
/// # Arguments
/// * `weight_kg` - Weight in kilograms
/// * `height_m` - Height in meters
pub fn calculate_bmi(weight_kg: f64, height_m: f64) -> Result<f64, String> {
    if weight_kg <= 0.0 || height_m <= 0.0 {
        return Err("Weight and height must be positive values".to_string());
    }

    Ok(weight_kg / (height_m * height_m))
}

pub fn run() {
    println!("\n--- BMI Calculator ---");
    let weight = read_input("Enter weight (kg): ");
    let height = read_input("Enter height (cm): ") / 100.0;

    match calculate_bmi(weight, height) {
        Ok(bmi) => {
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
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bmi() {
        let result = calculate_bmi(70.0, 1.75).unwrap();
        assert!((result - 22.86).abs() < 0.01);
    }
}
