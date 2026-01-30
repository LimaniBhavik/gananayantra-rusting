pub fn healthy_weight_range(height_cm: f64) -> Result<(f64, f64), String> {
    if height_cm <= 0.0 { return Err("Height must be positive".into()); }
    let h_m = height_cm / 100.0;
    let low = 18.5 * h_m.powi(2);
    let high = 25.0 * h_m.powi(2);
    Ok((low, high))
}

pub fn calories_burned(weight_kg: f64, duration_minutes: f64, met_value: f64) -> Result<f64, String> {
    if weight_kg <= 0.0 || duration_minutes <= 0.0 || met_value <= 0.0 {
        return Err("Values must be positive".into());
    }
    Ok((duration_minutes * met_value * 3.5 * weight_kg) / 200.0)
}

pub fn one_rep_max(weight: f64, reps: f64) -> Result<f64, String> {
    if weight <= 0.0 || reps <= 0.0 { return Err("Values must be positive".into()); }
    Ok(weight * (1.0 + reps / 30.0))
}

pub fn target_heart_rate(age: f64, resting_hr: f64) -> Result<(f64, f64, f64), String> {
    if age <= 0.0 || resting_hr <= 0.0 { return Err("Values must be positive".into()); }
    let max_hr = 220.0 - age;
    let hrr = max_hr - resting_hr;
    let low = hrr * 0.5 + resting_hr;
    let high = hrr * 0.85 + resting_hr;
    Ok((max_hr, low, high))
}

pub fn calculate_macros(tdee: f64, goal_type: i32) -> Result<(f64, f64, f64), String> {
    if tdee <= 0.0 { return Err("TDEE must be positive".into()); }
    let (p, c, f) = match goal_type {
        2 => (0.4, 0.2, 0.4), // Low Carb
        3 => (0.2, 0.6, 0.2), // High Carb
        _ => (0.3, 0.4, 0.3), // Balanced
    };
    Ok(((tdee * p) / 4.0, (tdee * c) / 4.0, (tdee * f) / 9.0))
}

pub fn calculate_tdee(bmr: f64, activity_level: i32) -> Result<f64, String> {
    if bmr <= 0.0 { return Err("BMR must be positive".into()); }
    let factor = match activity_level {
        2 => 1.375, // Light
        3 => 1.55,  // Moderate
        4 => 1.725, // Heavy
        5 => 1.9,   // Athlete
        _ => 1.2,   // Sedentary
    };
    Ok(bmr * factor)
}

pub fn ovulation_period(cycle_length: f64) -> Result<(f64, f64, f64), String> {
    if cycle_length <= 0.0 { return Err("Cycle length must be positive".into()); }
    let ovulation_day = cycle_length - 14.0;
    Ok((ovulation_day, cycle_length - 17.0, cycle_length - 12.0))
}

pub fn calculate_bac(weight_kg: f64, is_male: bool, alcohol_grams: f64, hours_since: f64) -> Result<f64, String> {
    if weight_kg <= 0.0 { return Err("Weight must be positive".into()); }
    let r = if is_male { 0.68 } else { 0.55 };
    let bac = (alcohol_grams / (weight_kg * 1000.0 * r)) * 100.0 - (hours_since * 0.015);
    Ok(bac.max(0.0))
}

pub fn calculate_bsa(height_cm: f64, weight_kg: f64) -> Result<f64, String> {
    if height_cm <= 0.0 || weight_kg <= 0.0 { return Err("Values must be positive".into()); }
    Ok(((height_cm * weight_kg) / 3600.0).sqrt())
}

pub fn lean_body_mass(height_cm: f64, weight_kg: f64, is_male: bool) -> Result<f64, String> {
    if height_cm <= 0.0 || weight_kg <= 0.0 { return Err("Values must be positive".into()); }
    let lbm = if is_male {
        0.407 * weight_kg + 0.267 * height_cm - 19.2
    } else {
        0.252 * weight_kg + 0.473 * height_cm - 48.3
    };
    Ok(lbm)
}
