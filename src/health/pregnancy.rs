pub fn calculate_pregnancy_progress(weeks: f64, days: f64) -> Result<(f64, f64), String> {
    if weeks < 0.0 || days < 0.0 {
        return Err("Values must be non-negative".into());
    }
    let total_days = weeks * 7.0 + days;
    let remaining = 280.0 - total_days;
    Ok((total_days, remaining))
}
