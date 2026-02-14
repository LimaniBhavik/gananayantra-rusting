/// Generates a simple random password.
///
/// **Warning:** This function uses a Linear Congruential Generator (LCG) seeded with the provided seed.
/// It is **not cryptographically secure** and should not be used for sensitive security applications.
///
/// # Arguments
/// * `length` - Length of the password
/// * `seed` - Seed for deterministic generation
pub fn generate_password(length: usize, seed: u64) -> Result<String, String> {
    if length == 0 {
        return Err("Length must be greater than zero".into());
    }
    let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()";
    let mut current_seed = seed as u128;
    let mut pass = String::with_capacity(length);
    for _ in 0..length {
        // LCG constants (mixed)
        current_seed = (current_seed.wrapping_mul(1103515245).wrapping_add(12345)) & 0x7fffffff;
        let idx = (current_seed % chars.len() as u128) as usize;
        pass.push(chars.chars().nth(idx).unwrap());
    }
    Ok(pass)
}

pub fn calculate_transfer_time(size_gb: f64, speed_mbps: f64) -> Result<f64, String> {
    if size_gb < 0.0 || speed_mbps <= 0.0 {
        return Err("Invalid inputs".into());
    }
    Ok((size_gb * 8192.0) / speed_mbps)
}
