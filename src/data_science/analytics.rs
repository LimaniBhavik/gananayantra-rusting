/// Calculates Data Quality Score (0-100) based on missing values and duplicates.
pub fn data_quality_score(total_rows: f64, missing_rows: f64, duplicate_rows: f64) -> Result<f64, String> {
    if total_rows <= 0.0 { return Err("Total rows must be > 0".into()); }
    let valid_ratio = (total_rows - missing_rows - duplicate_rows) / total_rows;
    Ok(valid_ratio * 100.0)
}

/// Simple Correlation Matrix calculator (Pearson) for 2 arrays.
pub fn correlation_coefficient(x: &[f64], y: &[f64]) -> Result<f64, String> {
    if x.len() != y.len() || x.is_empty() { return Err("Arrays must have same non-zero length".into()); }
    let n = x.len() as f64;
    let mean_x = x.iter().sum::<f64>() / n;
    let mean_y = y.iter().sum::<f64>() / n;

    let mut num = 0.0;
    let mut den_x = 0.0;
    let mut den_y = 0.0;

    for i in 0..x.len() {
        let dx = x[i] - mean_x;
        let dy = y[i] - mean_y;
        num += dx * dy;
        den_x += dx * dx;
        den_y += dy * dy;
    }

    if den_x == 0.0 || den_y == 0.0 { return Ok(0.0); }
    Ok(num / (den_x.sqrt() * den_y.sqrt()))
}
