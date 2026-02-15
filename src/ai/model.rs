/// Calculates approximate memory usage for a model inference.
///
/// # Arguments
/// * `params_billions` - Number of parameters in billions
/// * `precision_bits` - Precision (e.g., 32 for FP32, 16 for FP16, 8 for INT8)
/// * `context_window` - Context length
/// * `hidden_size` - Hidden layer size (for KV cache estimation)
/// * `layers` - Number of layers
///
/// # Returns
/// * Memory required in GB
pub fn calculate_model_memory(
    params_billions: f64,
    precision_bits: f64,
    context_window: f64,
    hidden_size: f64,
    layers: f64
) -> Result<f64, String> {
    if params_billions <= 0.0 || precision_bits <= 0.0 {
        return Err("Inputs must be positive".into());
    }
    // Weights
    let weights_gb = (params_billions * 1e9 * (precision_bits / 8.0)) / 1e9;

    // KV Cache (Simplified: 2 * layers * hidden * context * precision / 8)
    // 2 for K and V.
    let kv_cache_bytes = 2.0 * layers * hidden_size * context_window * (precision_bits / 8.0);
    let kv_cache_gb = kv_cache_bytes / 1e9;

    Ok(weights_gb + kv_cache_gb)
}

/// Calculates Attention Mechanism Complexity (FLOPS per token).
///
/// # Arguments
/// * `seq_len` - Sequence length
/// * `hidden_size` - Model dimension
///
/// # Returns
/// * Complexity score (FLOPS)
pub fn attention_complexity(seq_len: f64, hidden_size: f64) -> Result<f64, String> {
    if seq_len <= 0.0 {
        return Err("Sequence length must be positive".into());
    }
    // Standard Attention: 4 * N * d^2 + 2 * N^2 * d
    Ok(4.0 * seq_len * hidden_size.powi(2) + 2.0 * seq_len.powi(2) * hidden_size)
}

/// Estimates model size reduction via quantization.
pub fn quantization_saving(original_bits: f64, target_bits: f64) -> Result<f64, String> {
    if original_bits <= 0.0 || target_bits <= 0.0 {
        return Err("Bits must be positive".into());
    }
    let reduction = 1.0 - (target_bits / original_bits);
    Ok(reduction * 100.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory() {
        // 7B param, 16-bit -> ~14GB weights + KV
        let mem = calculate_model_memory(7.0, 16.0, 2048.0, 4096.0, 32.0).unwrap();
        assert!(mem > 14.0 && mem < 16.0);
    }
}
