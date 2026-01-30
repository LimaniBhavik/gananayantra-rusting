use std::time::{SystemTime, UNIX_EPOCH};

pub fn roll_die(sides: u32) -> Result<u32, String> {
    if sides == 0 {
        return Err("Die must have at least one side".into());
    }
    let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros();
    Ok((seed % sides as u128) as u32 + 1)
}

pub fn love_compatibility(name1: &str, name2: &str) -> u32 {
    let combined = format!("{}{}", name1, name2);
    let mut sum: u32 = 0;
    for b in combined.bytes() {
        sum += b as u32;
    }
    sum % 101 // Deterministic based on names
}
