pub fn calculate_cpm(cost: f64, impressions: f64) -> Result<f64, String> {
    if impressions <= 0.0 {
        return Err("Impressions must be greater than zero".into());
    }
    Ok((cost / impressions) * 1000.0)
}

pub fn calculate_ad_cost(cpm: f64, impressions: f64) -> Result<f64, String> {
    if cpm < 0.0 || impressions < 0.0 {
        return Err("Values must be non-negative".into());
    }
    Ok((cpm * impressions) / 1000.0)
}

pub fn calculate_impressions(cost: f64, cpm: f64) -> Result<f64, String> {
    if cpm <= 0.0 {
        return Err("CPM must be greater than zero".into());
    }
    Ok((cost / cpm) * 1000.0)
}
