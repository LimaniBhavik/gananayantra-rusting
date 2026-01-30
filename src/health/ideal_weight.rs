pub fn calculate_ideal_weight(height_cm: f64, is_male: bool) -> Result<f64, String> {
    if height_cm <= 0.0 {
        return Err("Height must be positive".into());
    }
    let height_in = height_cm / 2.54;
    if height_in <= 60.0 {
        return Err("Formula is applicable for heights over 5 feet (152.4 cm).".into());
    }
    let base_weight = if is_male { 50.0 } else { 45.5 };
    Ok(base_weight + 2.3 * (height_in - 60.0))
}
