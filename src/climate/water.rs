/// Calculates Industrial Water Footprint (Blue + Green + Grey).
///
/// # Arguments
/// * `blue_water` - Surface/Groundwater consumed (liters)
/// * `green_water` - Rainwater consumed (liters)
/// * `grey_water` - Freshwater needed to dilute pollutants (liters)
///
/// # Returns
/// * Total Water Footprint in liters
pub fn calculate_industrial_water_footprint(blue_water: f64, green_water: f64, grey_water: f64) -> Result<f64, String> {
    if blue_water < 0.0 || green_water < 0.0 || grey_water < 0.0 {
        return Err("Water volumes cannot be negative".into());
    }
    Ok(blue_water + green_water + grey_water)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_water_footprint() {
        assert_eq!(calculate_industrial_water_footprint(100.0, 50.0, 20.0).unwrap(), 170.0);
    }
}
