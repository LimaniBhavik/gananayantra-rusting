use crate::calculators::utils::read_input;

/// Calculates density given mass and volume.
///
/// # Arguments
/// * `mass` - Mass in kg (must be >= 0)
/// * `volume` - Volume in m³ (must be > 0)
///
/// # Returns
/// * Density in kg/m³
pub fn calculate_density(mass: f64, volume: f64) -> Result<f64, String> {
    if mass < 0.0 {
        return Err("Mass cannot be negative".into());
    }
    if volume <= 0.0 {
        return Err("Volume must be greater than zero".into());
    }
    Ok(mass / volume)
}

/// Calculates speed given distance and time.
///
/// # Arguments
/// * `distance` - Distance (any unit)
/// * `time` - Time (same unit basis, must be > 0)
///
/// # Returns
/// * Speed
pub fn calculate_speed(distance: f64, time: f64) -> Result<f64, String> {
    if time <= 0.0 {
        return Err("Time must be greater than zero".into());
    }
    Ok(distance / time)
}

/// Converts a decimal integer to Roman numerals.
///
/// # Arguments
/// * `n` - Number to convert (1 to 3999)
///
/// # Returns
/// * Roman numeral string
pub fn decimal_to_roman(n: u32) -> Result<String, String> {
    if n == 0 || n > 3999 {
        return Err("Number must be between 1 and 3999".into());
    }

    let values = [
        (1000, "M"), (900, "CM"), (500, "D"), (400, "CD"),
        (100, "C"), (90, "XC"), (50, "L"), (40, "XL"),
        (10, "X"), (9, "IX"), (5, "V"), (4, "IV"),
        (1, "I"),
    ];

    let mut num = n;
    let mut result = String::new();

    for (val, sym) in values {
        while num >= val {
            result.push_str(sym);
            num -= val;
        }
    }
    Ok(result)
}

/// Calculates GDP growth rate percentage.
///
/// # Arguments
/// * `old_gdp` - Previous GDP (must be > 0)
/// * `new_gdp` - Current GDP
///
/// # Returns
/// * Growth rate percentage
pub fn calculate_gdp_growth(old_gdp: f64, new_gdp: f64) -> Result<f64, String> {
    if old_gdp <= 0.0 {
        return Err("Old GDP must be greater than zero".into());
    }
    Ok(((new_gdp - old_gdp) / old_gdp) * 100.0)
}

pub fn run_menu() {
    loop {
        println!("\n--- Science and Measurements ---");
        println!("1. Density Calculator");
        println!("2. Speed/Distance/Time");
        println!("3. Roman Numeral Converter (Decimal to Roman)");
        println!("4. GDP Growth Calculator");
        println!("0. Back");
        let choice = read_input("Select an option: ");

        match choice as i32 {
            1 => run_density(),
            2 => run_speed(),
            3 => run_roman(),
            4 => run_gdp(),
            0 => break,
            _ => println!("Invalid choice."),
        }
    }
}

fn run_density() {
    let mass = read_input("Mass (kg): ");
    let vol = read_input("Volume (m³): ");
    match calculate_density(mass, vol) {
        Ok(d) => println!("Density: {:.2} kg/m³", d),
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_density() {
        assert!((calculate_density(100.0, 2.0).unwrap() - 50.0).abs() < 1e-10);
    }

    #[test]
    fn test_density_invalid() {
        assert!(calculate_density(100.0, 0.0).is_err());
        assert!(calculate_density(-10.0, 5.0).is_err());
    }

    #[test]
    fn test_calculate_speed() {
        assert!((calculate_speed(100.0, 2.0).unwrap() - 50.0).abs() < 1e-10);
    }

    #[test]
    fn test_speed_invalid() {
        assert!(calculate_speed(100.0, 0.0).is_err());
        assert!(calculate_speed(100.0, -5.0).is_err());
    }

    #[test]
    fn test_decimal_to_roman() {
        assert_eq!(decimal_to_roman(1).unwrap(), "I");
        assert_eq!(decimal_to_roman(4).unwrap(), "IV");
        assert_eq!(decimal_to_roman(9).unwrap(), "IX");
        assert_eq!(decimal_to_roman(58).unwrap(), "LVIII");
        assert_eq!(decimal_to_roman(1994).unwrap(), "MCMXCIV");
        assert_eq!(decimal_to_roman(3999).unwrap(), "MMMCMXCIX");
    }

    #[test]
    fn test_roman_invalid() {
        assert!(decimal_to_roman(0).is_err());
        assert!(decimal_to_roman(4000).is_err());
    }

    #[test]
    fn test_calculate_gdp_growth() {
        assert!((calculate_gdp_growth(100.0, 110.0).unwrap() - 10.0).abs() < 1e-10);
        assert!((calculate_gdp_growth(100.0, 90.0).unwrap() + 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_gdp_invalid() {
        assert!(calculate_gdp_growth(0.0, 100.0).is_err());
        assert!(calculate_gdp_growth(-100.0, 100.0).is_err());
    }
}

fn run_speed() {
    let dist = read_input("Distance: ");
    let time = read_input("Time: ");
    match calculate_speed(dist, time) {
        Ok(s) => println!("Speed: {:.2}", s),
        Err(e) => println!("Error: {}", e),
    }
}

fn run_roman() {
    let num = read_input("Number (1-3999): ");
    if num.fract() != 0.0 {
        println!("Error: Please enter a whole number.");
        return;
    }
    match decimal_to_roman(num as u32) {
        Ok(s) => println!("Roman Numeral: {}", s),
        Err(e) => println!("Error: {}", e),
    }
}

fn run_gdp() {
    let old = read_input("Old GDP: ");
    let new = read_input("New GDP: ");
    match calculate_gdp_growth(old, new) {
        Ok(g) => println!("Growth Rate: {:.2}%", g),
        Err(e) => println!("Error: {}", e),
    }
}
