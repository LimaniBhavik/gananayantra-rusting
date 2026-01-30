pub fn calculate_wind_chill(temp_f: f64, wind_mph: f64) -> Result<f64, String> {
    // Formula valid for T < 50F and V > 3mph usually
    Ok(35.74 + 0.6215 * temp_f - 35.75 * wind_mph.powf(0.16)
        + 0.4275 * temp_f * wind_mph.powf(0.16))
}

pub fn calculate_heat_index(temp_f: f64, humidity_percent: f64) -> Result<f64, String> {
    let t = temp_f;
    let r = humidity_percent;
    // Rothfusz regression
    Ok(
        -42.379 + 2.049 * t + 10.14 * r - 0.224 * t * r - 0.0068 * t * t - 0.054 * r * r
            + 0.0012 * t * t * r
            + 0.00085 * t * r * r
            - 0.0000019 * t * t * r * r,
    )
}
