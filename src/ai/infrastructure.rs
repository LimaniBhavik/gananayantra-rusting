/// Calculates Cloud Compute Cost.
pub fn cloud_compute_cost(instances: f64, hours: f64, hourly_rate: f64) -> Result<f64, String> {
    if instances < 0.0 || hours < 0.0 || hourly_rate < 0.0 {
        return Err("Inputs cannot be negative".into());
    }
    Ok(instances * hours * hourly_rate)
}

/// Estimates Data Storage Requirements.
///
/// # Arguments
/// * `dataset_size_gb` - Base dataset size
/// * `redundancy_factor` - Replication factor (e.g., 3 for typical cloud storage)
/// * `growth_rate` - Monthly growth rate (0.0 - 1.0)
/// * `months` - Projection period
///
/// # Returns
/// * Projected Storage in GB
pub fn data_storage_projection(dataset_size_gb: f64, redundancy_factor: f64, growth_rate: f64, months: f64) -> Result<f64, String> {
    if dataset_size_gb < 0.0 { return Err("Dataset size must be positive".into()); }
    let future_size = dataset_size_gb * (1.0 + growth_rate).powf(months);
    Ok(future_size * redundancy_factor)
}

/// Predicts Bandwidth Usage.
///
/// # Arguments
/// * `daily_requests` - Number of API requests per day
/// * `avg_response_size_kb` - Average size per response in KB
///
/// # Returns
/// * Monthly Bandwidth in GB
pub fn predict_bandwidth_usage(daily_requests: f64, avg_response_size_kb: f64) -> Result<f64, String> {
    let daily_gb = (daily_requests * avg_response_size_kb) / (1024.0 * 1024.0);
    Ok(daily_gb * 30.0)
}

/// Estimates Cooling Requirements (BTU/hr) based on Power Consumption.
pub fn cooling_requirements_btu(power_watts: f64) -> Result<f64, String> {
    // 1 Watt approx 3.412 BTU/hr
    if power_watts < 0.0 { return Err("Power must be positive".into()); }
    Ok(power_watts * 3.412)
}

/// Calculates Storage IOPS requirements.
pub fn storage_iops(throughput_mbps: f64, block_size_kb: f64) -> Result<f64, String> {
    if block_size_kb <= 0.0 { return Err("Block size must be > 0".into()); }
    // IOPS = (Throughput in MB/s * 1024) / Block Size in KB
    Ok((throughput_mbps * 1024.0) / block_size_kb)
}
