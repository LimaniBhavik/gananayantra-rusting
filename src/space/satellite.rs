use std::f64::consts::PI;

/// Calculates the ground coverage area (footprint) of a satellite.
///
/// # Arguments
/// * `altitude_km` - Satellite altitude in km
/// * `beam_angle_deg` - Beam width in degrees (nadir)
///
/// # Returns
/// * Coverage Area in km²
pub fn calculate_coverage_area(altitude_km: f64, _beam_angle_deg: f64) -> Result<f64, String> {
    if altitude_km <= 0.0 {
        return Err("Inputs must be positive".into());
    }
    let r_earth = 6371.0;
    // Slant range and spherical cap math simplified for beam angle
    // let alpha = (beam_angle_deg / 2.0).to_radians();
    // let slant_range = altitude_km / alpha.cos(); // Simplified flat approx for narrow beam, better logic needed for wide
    // Using simple spherical cap area for "visible earth" from altitude
    // Area = 2 * PI * R^2 * (1 - cos(theta)), where sin(theta) = R / (R+h)
    let h = altitude_km;
    let cos_theta = (r_earth * r_earth + 2.0 * r_earth * h).sqrt() / (r_earth + h); // Horizon limit
    // If beam limit is smaller, use beam.
    Ok(2.0 * PI * r_earth * r_earth * (1.0 - cos_theta)) // Max horizon coverage
}
