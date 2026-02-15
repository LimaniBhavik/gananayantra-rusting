/// Calculates Inference Latency (simplified queuing model).
///
/// # Arguments
/// * `processing_time_ms` - Model forward pass time
/// * `queue_depth` - Average request queue depth
/// * `batch_size` - Batch size used
///
/// # Returns
/// * Latency in ms
pub fn predict_latency(processing_time_ms: f64, queue_depth: f64, batch_size: f64) -> Result<f64, String> {
    if batch_size <= 0.0 {
        return Err("Batch size must be > 0".into());
    }
    // Latency = Processing + Queuing Delay
    Ok(processing_time_ms + (queue_depth * processing_time_ms / batch_size))
}

/// Throughput Predictor (Tokens/sec or Requests/sec).
///
/// # Arguments
/// * `batch_size` - Batch size
/// * `latency_ms` - Latency per batch
///
/// # Returns
/// * Throughput in units per second
pub fn calculate_throughput(batch_size: f64, latency_ms: f64) -> Result<f64, String> {
    if latency_ms <= 0.0 {
        return Err("Latency must be > 0".into());
    }
    Ok((batch_size * 1000.0) / latency_ms)
}
