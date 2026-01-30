pub fn calculate_fuel_cost(distance: f64, mpg: f64, price_per_gallon: f64) -> Result<f64, String> {
    if mpg <= 0.0 {
        return Err("MPG must be greater than zero".into());
    }
    Ok((distance / mpg) * price_per_gallon)
}

pub fn calculate_mpg(miles: f64, gallons: f64) -> Result<f64, String> {
    if gallons <= 0.0 {
        return Err("Gallons must be greater than zero".into());
    }
    Ok(miles / gallons)
}

pub fn calculate_horsepower(torque: f64, rpm: f64) -> Result<f64, String> {
    Ok((torque * rpm) / 5252.0)
}
