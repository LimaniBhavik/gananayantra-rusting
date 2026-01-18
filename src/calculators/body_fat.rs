use crate::calculators::utils::read_input;

pub fn run() {
    println!("\n--- Body Fat Calculator (U.S. Navy Method) ---");
    let gender = crate::calculators::utils::read_string("Enter gender (m/f): ");
    let weight = read_input("Enter weight (kg): ");
    let height = read_input("Enter height (cm): ");
    let neck = read_input("Enter neck circumference (cm): ");
    let waist = read_input("Enter waist circumference (cm): ");

    let bf = if gender.to_lowercase() == "m" {
        495.0 / (1.0324 - 0.19077 * (waist - neck).log10() + 0.15456 * height.log10()) - 450.0
    } else {
        let hip = read_input("Enter hip circumference (cm): ");
        495.0 / (1.29579 - 0.35004 * (waist + hip - neck).log10() + 0.22100 * height.log10()) - 450.0
    };

    println!("Estimated Body Fat: {:.1}%", bf);
}
