/// Calculates Edge Inference Latency including Network Overhead.
///
/// # Arguments
/// * `model_inference_ms` - Model inference time in milliseconds
/// * `network_overhead_ms` - Additional network latency (if cloud-assisted or transmitting result) in milliseconds
///
/// # Returns
/// * Total latency in milliseconds
pub fn edge_inference_latency(
    model_inference_ms: f64,
    network_overhead_ms: f64
) -> Result<f64, String> {
    if model_inference_ms < 0.0 || network_overhead_ms < 0.0 {
        return Err("Latencies cannot be negative".into());
    }
    Ok(model_inference_ms + network_overhead_ms)
}

/// Calculates Memory Saving from Quantization.
///
/// # Arguments
/// * `original_bits` - Original bit width (e.g., 32 for FP32)
/// * `quantized_bits` - Quantized bit width (e.g., 8 for INT8)
///
/// # Returns
/// * Percentage memory saved (0.0 to 1.0)
pub fn quantization_memory_saving(
    original_bits: f64,
    quantized_bits: f64
) -> Result<f64, String> {
    if original_bits <= 0.0 {
        return Err("Original bits must be positive".into());
    }
    if quantized_bits <= 0.0 {
        return Err("Quantized bits must be positive".into());
    }
    if quantized_bits >= original_bits {
        return Ok(0.0); // No saving or negative saving not typical for this metric logic
    }

    Ok((original_bits - quantized_bits) / original_bits)
}
