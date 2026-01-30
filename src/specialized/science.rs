pub fn calculate_density(mass: f64, volume: f64) -> Result<f64, String> {
    if volume <= 0.0 {
        return Err("Volume must be greater than zero".into());
    }
    Ok(mass / volume)
}

pub fn calculate_speed(distance: f64, time: f64) -> Result<f64, String> {
    if time <= 0.0 {
        return Err("Time must be greater than zero".into());
    }
    Ok(distance / time)
}

pub fn decimal_to_roman(mut num: i32) -> Result<String, String> {
    if num <= 0 || num > 3999 {
        return Err("Number must be between 1 and 3999".into());
    }
    let vals = [
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];
    let mut res = String::new();
    for (v, s) in vals {
        while num >= v {
            res.push_str(s);
            num -= v;
        }
    }
    Ok(res)
}

pub fn gdp_growth_rate(old_gdp: f64, new_gdp: f64) -> Result<f64, String> {
    if old_gdp == 0.0 {
        return Err("Old GDP cannot be zero".into());
    }
    Ok(((new_gdp - old_gdp) / old_gdp) * 100.0)
}
