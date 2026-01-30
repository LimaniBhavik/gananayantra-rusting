const GRAVITY: f64 = 9.81;

/// Calculates kinetic energy.
///
/// # Arguments
/// * `mass` - Mass in kg
/// * `velocity` - Velocity in m/s
///
/// # Returns
/// * Kinetic Energy in Joules (J)
pub fn calculate_kinetic_energy(mass: f64, velocity: f64) -> Result<f64, String> {
    if mass < 0.0 {
        return Err("Mass cannot be negative".into());
    }
    Ok(0.5 * mass * velocity.powi(2))
}

/// Calculates gravitational potential energy.
///
/// # Arguments
/// * `mass` - Mass in kg
/// * `height` - Height in m
///
/// # Returns
/// * Potential Energy in Joules (J)
pub fn calculate_potential_energy(mass: f64, height: f64) -> Result<f64, String> {
    if mass < 0.0 {
        return Err("Mass cannot be negative".into());
    }
    Ok(mass * GRAVITY * height)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kinetic() {
        assert_eq!(calculate_kinetic_energy(10.0, 2.0).unwrap(), 20.0);
    }

    #[test]
    fn test_potential() {
        let pe = calculate_potential_energy(10.0, 10.0).unwrap();
        assert!((pe - 981.0).abs() < 0.1);
    }
}
