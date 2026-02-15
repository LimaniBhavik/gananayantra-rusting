/// Calculates the cost of LLM tokens per million.
///
/// # Arguments
/// * `tokens` - Number of tokens
/// * `price_per_million` - Cost per 1M tokens
///
/// # Returns
/// * Total Cost
pub fn calculate_token_cost(tokens: f64, price_per_million: f64) -> Result<f64, String> {
    if tokens < 0.0 || price_per_million < 0.0 {
        return Err("Inputs must be non-negative".into());
    }
    Ok((tokens / 1_000_000.0) * price_per_million)
}

/// Calculates the efficiency of a model's context window.
///
/// # Arguments
/// * `context_window` - Context window size (tokens)
/// * `actual_usage` - Average tokens used per request
///
/// # Returns
/// * Efficiency (0.0 to 1.0)
pub fn context_window_efficiency(context_window: f64, actual_usage: f64) -> Result<f64, String> {
    if context_window <= 0.0 {
        return Err("Context window must be positive".into());
    }
    Ok((actual_usage / context_window).min(1.0).max(0.0))
}

/// Calculates Parameter Efficiency (Parameters / Size in GB).
pub fn parameter_efficiency(params_billions: f64, size_gb: f64) -> Result<f64, String> {
    if size_gb <= 0.0 { return Err("Size must be positive".into()); }
    Ok(params_billions / size_gb)
}

/// Estimates memory usage optimization potential via quantization.
pub fn memory_usage_optimization(current_memory_gb: f64, quantization_ratio: f64) -> Result<f64, String> {
    if current_memory_gb < 0.0 { return Err("Memory must be positive".into()); }
    Ok(current_memory_gb * (1.0 - quantization_ratio))
}
