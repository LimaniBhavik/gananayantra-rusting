const G: f64 = 6.67430e-11;        // m^3 kg^-1 s^-2
const EARTH_MASS: f64 = 5.972e24; // kg
const EARTH_RADIUS: f64 = 6_371_000.0; // meters

/// Calculates orbital velocity for a circular orbit around Earth.
///
/// # Arguments
/// * `altitude_m` - Altitude above Earth's surface in meters
///
/// # Returns
/// * Orbital velocity in meters per second (m/s)
pub fn orbital_velocity(
    altitude_m: f64,
) -> Result<f64, String> {
    if altitude_m < 0.0 {
        return Err("Altitude cannot be negative".into());
    }

    let orbital_radius = EARTH_RADIUS + altitude_m;
    let velocity = (G * EARTH_MASS / orbital_radius).sqrt();

    Ok(velocity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orbital_velocity_leo() {
        let v = orbital_velocity(400_000.0).unwrap();
        assert!((v - 7670.0).abs() < 150.0);
    }

    #[test]
    fn test_invalid_altitude() {
        assert!(orbital_velocity(-1.0).is_err());
    }
}
