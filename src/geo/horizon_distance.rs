const EARTH_RADIUS_M: f64 = 6_371_000.0;

/// Calculates the distance to the horizon accounting for Earth’s curvature.
///
/// # Arguments
/// * `height_m` - Observer height above Earth surface in meters
///
/// # Returns
/// * Distance to the horizon in kilometers
pub fn horizon_distance(height_m: f64) -> Result<f64, String> {
    if height_m < 0.0 {
        return Err("Height cannot be negative".into());
    }

    let distance_m =
        (2.0 * EARTH_RADIUS_M * height_m + height_m.powi(2)).sqrt();

    Ok(distance_m / 1000.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_horizon_distance_10m() {
        let d = horizon_distance(10.0).unwrap();
        assert!((d - 11.3).abs() < 0.5);
    }

    #[test]
    fn test_invalid_height() {
        assert!(horizon_distance(-5.0).is_err());
    }
}
