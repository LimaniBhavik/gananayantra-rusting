/// Basic arithmetic operations.
pub fn add(a: f64, b: f64) -> f64 { a + b }
pub fn sub(a: f64, b: f64) -> f64 { a - b }
pub fn mul(a: f64, b: f64) -> f64 { a * b }
pub fn div(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero".into())
    } else {
        Ok(a / b)
    }
}
