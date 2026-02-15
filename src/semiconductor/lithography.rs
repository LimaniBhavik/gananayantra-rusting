/// Calculates Minimum Resolvable Feature Size (Resolution) using Rayleigh's criterion.
///
/// # Arguments
/// * `wavelength_nm` - Wavelength of light (e.g., 13.5nm for EUV, 193nm for DUV)
/// * `numerical_aperture` - NA of the lens system
/// * `k1_factor` - Process factor (typically 0.25 to 0.8)
///
/// # Returns
/// * Resolution (Critical Dimension) in nm
pub fn calculate_resolution(wavelength_nm: f64, numerical_aperture: f64, k1_factor: f64) -> Result<f64, String> {
    if wavelength_nm <= 0.0 || numerical_aperture <= 0.0 || k1_factor <= 0.0 {
        return Err("All factors must be positive".into());
    }
    Ok(k1_factor * (wavelength_nm / numerical_aperture))
}

/// Calculates Depth of Focus (DOF).
///
/// # Arguments
/// * `wavelength_nm` - Wavelength of light
/// * `numerical_aperture` - NA of the lens system
/// * `k2_factor` - Process factor
///
/// # Returns
/// * DOF in nm
pub fn calculate_dof(wavelength_nm: f64, numerical_aperture: f64, k2_factor: f64) -> Result<f64, String> {
    if wavelength_nm <= 0.0 || numerical_aperture <= 0.0 || k2_factor <= 0.0 {
        return Err("All factors must be positive".into());
    }
    Ok(k2_factor * (wavelength_nm / numerical_aperture.powi(2)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euv_resolution() {
        // EUV: 13.5nm, NA 0.33, k1 0.3 -> ~12.27nm
        let res = calculate_resolution(13.5, 0.33, 0.3).unwrap();
        assert!((res - 12.27).abs() < 0.1);
    }
}
