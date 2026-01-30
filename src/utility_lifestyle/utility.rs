pub fn calculate_gpa(grades: &[(f64, f64)]) -> Result<f64, String> {
    if grades.is_empty() {
        return Err("No courses provided".into());
    }
    let mut total_points = 0.0;
    let mut total_credits = 0.0;
    for (grade, credits) in grades {
        total_points += grade * credits;
        total_credits += credits;
    }
    if total_credits == 0.0 {
        return Err("Total credits cannot be zero".into());
    }
    Ok(total_points / total_credits)
}

pub fn calculate_tip(bill: f64, tip_percent: f64, split_between: f64) -> Result<(f64, f64, f64), String> {
    if split_between <= 0.0 {
        return Err("Number of people must be positive".into());
    }
    let tip = bill * (tip_percent / 100.0);
    let total = bill + tip;
    Ok((tip, total, total / split_between))
}

pub fn convert_shoe_size_us_to_uk_eu(us_size: f64) -> (f64, f64) {
    (us_size - 0.5, us_size + 33.0)
}

pub fn calculate_bedtime(wake_hour: i32, cycles: i32) -> Result<String, String> {
    if !(0..=24).contains(&wake_hour) {
        return Err("Invalid hour".into());
    }
    let total_min = cycles * 90;
    let mut bed_h = (wake_hour - (total_min / 60)) % 24;
    if bed_h < 0 { bed_h += 24; }
    let bed_m = (60 - (total_min % 60)) % 60;
    Ok(format!("{:02}:{:02}", bed_h, bed_m))
}
