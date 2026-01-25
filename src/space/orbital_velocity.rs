const G: f64 = 6.67430e-11;        // Gravitational constant (m^3 kg^-1 s^-2)
const EARTH_MASS: f64 = 5.972e24; // kg
const EARTH_RADIUS: f64 = 6_371_000.0; // meters

pub fn orbital_velocity(
    altitude_m: f64,
) -> Result<f64, String> {
    if altitude_m < 0.0 {
        return Err("Altitude cannot be negative".to_string());
    }

    let orbital_radius = EARTH_RADIUS + altitude_m;
    let velocity = (G * EARTH_MASS / orbital_radius).sqrt();

    Ok(velocity)
}

pub fn run() {
    println!("\n--- Orbital Velocity Calculator ---");
    use crate::calculators::utils::read_input;

    let altitude_km = read_input("Enter altitude above Earth in km: ");
    let altitude_m = altitude_km * 1000.0;

    match orbital_velocity(altitude_m) {
        Ok(v) => {
            println!("Orbital Velocity: {:.2} m/s", v);
            println!("Orbital Velocity: {:.2} km/s", v / 1000.0);
        },
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orbital_velocity() {
        let v = orbital_velocity(400_000.0).unwrap();
        assert!((v - 7670.0).abs() < 100.0);
    }
}
