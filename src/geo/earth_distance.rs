const EARTH_RADIUS_KM: f64 = 6371.0;

/// Calculates the great-circle distance between two points on Earth
/// using the Haversine formula.
///
/// # Arguments
/// * `lat1` - Latitude of first point (degrees)
/// * `lon1` - Longitude of first point (degrees)
/// * `lat2` - Latitude of second point (degrees)
/// * `lon2` - Longitude of second point (degrees)
///
/// # Returns
/// * Distance in kilometers
pub fn haversine_distance(
    lat1: f64,
    lon1: f64,
    lat2: f64,
    lon2: f64,
) -> Result<f64, String> {
    if !( -90.0..=90.0 ).contains(&lat1) || !( -90.0..=90.0 ).contains(&lat2) {
        return Err("Latitude must be between -90 and 90 degrees".into());
    }
    if !( -180.0..=180.0 ).contains(&lon1) || !( -180.0..=180.0 ).contains(&lon2) {
        return Err("Longitude must be between -180 and 180 degrees".into());
    }

    let lat1_rad = lat1.to_radians();
    let lat2_rad = lat2.to_radians();
    let dlat = (lat2 - lat1).to_radians();
    let dlon = (lon2 - lon1).to_radians();

    let a = (dlat / 2.0).sin().powi(2)
        + lat1_rad.cos() * lat2_rad.cos()
        * (dlon / 2.0).sin().powi(2);

    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    Ok(EARTH_RADIUS_KM * c)
}

pub fn run() {
    println!("\n--- Earth Distance (Haversine) Calculator ---");
    use crate::calculators::utils::read_input;

    let lat1 = read_input("Enter Latitude 1 (degrees): ");
    let lon1 = read_input("Enter Longitude 1 (degrees): ");
    let lat2 = read_input("Enter Latitude 2 (degrees): ");
    let lon2 = read_input("Enter Longitude 2 (degrees): ");

    match haversine_distance(lat1, lon1, lat2, lon2) {
        Ok(distance) => println!("Distance: {:.2} km", distance),
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equator_distance() {
        let d = haversine_distance(0.0, 0.0, 0.0, 1.0).unwrap();
        assert!((d - 111.0).abs() < 2.0);
    }

    #[test]
    fn test_invalid_latitude() {
        assert!(haversine_distance(100.0, 0.0, 0.0, 0.0).is_err());
    }
}
