use std::time::{SystemTime, UNIX_EPOCH};

/// Generates a simple random password.
///
/// **Warning:** This function uses a Linear Congruential Generator (LCG) seeded with the system time.
/// It is **not cryptographically secure** and should not be used for sensitive security applications.
pub fn generate_password(length: usize) -> Result<String, String> {
    if length == 0 {
        return Err("Length must be greater than zero".into());
    }
    let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()";
    let mut seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros();
    let mut pass = String::with_capacity(length);
    for _ in 0..length {
        // LCG constants (mixed)
        seed = (seed.wrapping_mul(1103515245).wrapping_add(12345)) & 0x7fffffff;
        let idx = (seed % chars.len() as u128) as usize;
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
