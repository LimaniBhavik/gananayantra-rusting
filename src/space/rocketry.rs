/// Calculates Delta-V using the Tsiolkovsky Rocket Equation.
///
/// # Arguments
/// * `isp` - Specific Impulse in seconds
/// * `wet_mass` - Initial mass (with fuel)
/// * `dry_mass` - Final mass (without fuel)
///
/// # Returns
/// * Delta-V in m/s
pub fn calculate_delta_v(isp: f64, wet_mass: f64, dry_mass: f64) -> Result<f64, String> {
    if isp <= 0.0 || wet_mass <= 0.0 || dry_mass <= 0.0 {
        return Err("Inputs must be positive".into());
    }
    if dry_mass >= wet_mass {
        return Err("Dry mass must be less than wet mass".into());
    }
    let g0 = 9.80665;
    Ok(isp * g0 * (wet_mass / dry_mass).ln())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delta_v() {
        let dv = calculate_delta_v(300.0, 1000.0, 100.0).unwrap();
        // 300 * 9.8 * ln(10) ~ 2940 * 2.3 = 6769
        assert!(dv > 6700.0 && dv < 6800.0);
    }
}
