/// Calculates Degrees of Freedom (Grübler's formula for planar mechanisms).
///
/// # Arguments
/// * `num_links` - Number of links (including ground)
/// * `num_joints` - Number of joints
/// * `joint_freedom` - Degrees of freedom per joint (typically 1)
///
/// # Returns
/// * DOF
pub fn calculate_dof_planar(num_links: f64, num_joints: f64, joint_freedom: f64) -> Result<f64, String> {
    // F = 3*(N-1) - 2*J - ... (Simplified)
    Ok(3.0 * (num_links - 1.0) - 2.0 * num_joints * joint_freedom)
}

/// Calculates simple trapezoidal motion profile time.
///
/// # Arguments
/// * `distance` - Total distance
/// * `max_velocity` - Max velocity
/// * `acceleration` - Acceleration rate
///
/// # Returns
/// * Total time
pub fn motion_profile_time(distance: f64, max_velocity: f64, acceleration: f64) -> Result<f64, String> {
    if max_velocity <= 0.0 || acceleration <= 0.0 { return Err("Invalid kinematics".into()); }

    let t_accel = max_velocity / acceleration;
    let d_accel = 0.5 * acceleration * t_accel.powi(2);

    if 2.0 * d_accel > distance {
        // Triangular profile (never reaches max velocity)
        Ok(2.0 * (distance / acceleration).sqrt())
    } else {
        // Trapezoidal profile
        let d_cruise = distance - 2.0 * d_accel;
        let t_cruise = d_cruise / max_velocity;
        Ok(2.0 * t_accel + t_cruise)
    }
}
