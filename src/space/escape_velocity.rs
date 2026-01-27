const G: f64 = 6.67430e-11;        // m^3 kg^-1 s^-2
const EARTH_MASS: f64 = 5.972e24; // kg
const EARTH_RADIUS: f64 = 6_371_000.0; // meters

/// Calculates escape velocity from Earth at a given altitude.
///
/// # Arguments
/// * `altitude_m` - Altitude above Earth's surface in meters
///
/// # Returns
/// * Escape velocity in meters per second (m/s)
pub fn escape_velocity(altitude_m: f64) -> Result<f64, String> {
    if altitude_m < 0.0 {
        return Err("Altitude cannot be negative".into());
    }

    let radius = EARTH_RADIUS + altitude_m;
    let velocity = (2.0 * G * EARTH_MASS / radius).sqrt();

    Ok(velocity)
}

pub fn run() {
    println!("\n--- Escape Velocity Calculator ---");
    use crate::calculators::utils::read_input;

    let altitude_km = read_input("Enter altitude above Earth in km: ");
    let altitude_m = altitude_km * 1000.0;

    match escape_velocity(altitude_m) {
        Ok(v) => {
            println!("Escape Velocity: {:.2} m/s", v);
            println!("Escape Velocity: {:.2} km/s", v / 1000.0);
        },
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_velocity_surface() {
        let v = escape_velocity(0.0).unwrap();
        assert!((v - 11186.0).abs() < 200.0);
    }

    #[test]
    fn test_invalid_altitude() {
        assert!(escape_velocity(-10.0).is_err());
    }
}
