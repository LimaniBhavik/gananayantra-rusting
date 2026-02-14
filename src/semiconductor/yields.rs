use std::f64::consts::PI;

/// Calculates the number of Gross Dies per Wafer (DPW).
///
/// # Arguments
/// * `wafer_diameter_mm` - Diameter of the wafer (e.g., 300mm)
/// * `die_area_mm2` - Area of a single die in mm²
///
/// # Returns
/// * Total potential dies on the wafer
pub fn calculate_gross_dies(wafer_diameter_mm: f64, die_area_mm2: f64) -> Result<f64, String> {
    if wafer_diameter_mm <= 0.0 || die_area_mm2 <= 0.0 {
        return Err("Dimensions must be greater than zero".into());
    }
    let radius = wafer_diameter_mm / 2.0;
    let wafer_area = PI * radius.powi(2);
    let dies = (wafer_area / die_area_mm2) - (PI * wafer_diameter_mm) / (2.0 * die_area_mm2.sqrt());
    Ok(dies.floor())
}

/// Calculates Die Yield using the Murphy Model.
///
/// # Arguments
/// * `defect_density` - Defects per cm²
/// * `die_area_mm2` - Area of a single die in mm²
///
/// # Returns
/// * Yield percentage (0.0 to 100.0)
pub fn calculate_die_yield_murphy(defect_density_per_cm2: f64, die_area_mm2: f64) -> Result<f64, String> {
    if defect_density_per_cm2 < 0.0 || die_area_mm2 <= 0.0 {
        return Err("Invalid inputs".into());
    }
    let die_area_cm2 = die_area_mm2 / 100.0;
    let exponent = -defect_density_per_cm2 * die_area_cm2;
    let yield_val = ((1.0 - (-exponent).exp()) / (-exponent).abs()).powi(2);
    Ok(yield_val * 100.0)
}

/// Calculates Net Good Dies per Wafer.
pub fn calculate_net_dies(gross_dies: f64, yield_percent: f64) -> Result<f64, String> {
    if gross_dies < 0.0 || yield_percent < 0.0 || yield_percent > 100.0 {
        return Err("Invalid inputs".into());
    }
    Ok((gross_dies * (yield_percent / 100.0)).floor())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gross_dies() {
        // 300mm wafer, 100mm2 die -> approx 600-700
        let dies = calculate_gross_dies(300.0, 100.0).unwrap();
        assert!(dies > 600.0 && dies < 750.0);
    }
}
