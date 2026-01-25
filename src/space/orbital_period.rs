const G: f64 = 6.67430e-11;        // m^3 kg^-1 s^-2
const EARTH_MASS: f64 = 5.972e24; // kg
const EARTH_RADIUS: f64 = 6_371_000.0; // meters
const PI: f64 = std::f64::consts::PI;

pub fn orbital_period(altitude_m: f64) -> Result<f64, String> {
    if altitude_m < 0.0 {
        return Err("Altitude cannot be negative".to_string());
    }

    let r = EARTH_RADIUS + altitude_m;
    let period_seconds = 2.0 * PI * (r.powi(3) / (G * EARTH_MASS)).sqrt();

    Ok(period_seconds / 60.0) // convert to minutes
}

pub fn run() {
    println!("\n--- Satellite Orbital Period Calculator ---");
    use crate::calculators::utils::read_input;

    let altitude = read_input("Enter satellite altitude above Earth (meters): ");

    match orbital_period(altitude) {
        Ok(period) => println!("Orbital Period: {:.2} minutes", period),
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orbital_period() {
        let t = orbital_period(400_000.0).unwrap();
        assert!((t - 92.0).abs() < 3.0);
    }
}
