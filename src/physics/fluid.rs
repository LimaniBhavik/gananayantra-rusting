/// Calculates Reynolds Number to predict fluid flow regime (laminar vs turbulent).
///
/// # Arguments
/// * `velocity` - Fluid velocity in m/s
/// * `characteristic_length` - Diameter of pipe or length in m
/// * `kinematic_viscosity` - Fluid kinematic viscosity in m²/s (e.g., water ~1e-6 at 20C)
///
/// # Returns
/// * Reynolds Number (dimensionless)
pub fn calculate_reynolds_number(
    velocity: f64,
    characteristic_length: f64,
    kinematic_viscosity: f64,
) -> Result<f64, String> {
    if kinematic_viscosity <= 0.0 {
        return Err("Viscosity must be greater than zero".into());
    }
    if characteristic_length < 0.0 {
        return Err("Length cannot be negative".into());
    }
    Ok((velocity * characteristic_length) / kinematic_viscosity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reynolds() {
        let re = calculate_reynolds_number(10.0, 0.5, 1e-6).unwrap();
        assert_eq!(re, 5_000_000.0);
    }
}
