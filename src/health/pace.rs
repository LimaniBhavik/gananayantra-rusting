pub struct PaceResult {
    pub pace_minutes: f64,
    pub pace_seconds: f64,
    pub speed_kmh: f64,
}

pub fn calculate_pace(distance_km: f64, hours: f64, minutes: f64, seconds: f64) -> Result<PaceResult, String> {
    if distance_km <= 0.0 {
        return Err("Distance must be greater than zero".into());
    }
    let total_seconds = hours * 3600.0 + minutes * 60.0 + seconds;
    if total_seconds <= 0.0 {
        return Err("Time must be greater than zero".into());
    }

    let pace_seconds_per_km = total_seconds / distance_km;
    let p_min = (pace_seconds_per_km / 60.0).floor();
    let p_sec = (pace_seconds_per_km % 60.0).round();
    let speed = distance_km / (total_seconds / 3600.0);

    Ok(PaceResult {
        pace_minutes: p_min,
        pace_seconds: p_sec,
        speed_kmh: speed,
    })
}
