const EARTH_RADIUS_M: f64 = 6_371_000.0;

pub fn horizon_distance(height_m: f64) -> Result<f64, String> {
    if height_m < 0.0 {
        return Err("Height cannot be negative".to_string());
    }

    let distance_m = (2.0 * EARTH_RADIUS_M * height_m + height_m.powi(2)).sqrt();
    Ok(distance_m / 1000.0) // convert to km
}

pub fn run() {
    println!("\n--- Horizon Distance (Earth Curvature) Calculator ---");
    use crate::calculators::utils::read_input;

    let height = read_input("Enter observer height above Earth's surface (meters): ");

    match horizon_distance(height) {
        Ok(distance) => println!("Distance to horizon: {:.2} km", distance),
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_horizon_distance() {
        let d = horizon_distance(10.0).unwrap();
        assert!((d - 11.3).abs() < 0.5);
    }
}
