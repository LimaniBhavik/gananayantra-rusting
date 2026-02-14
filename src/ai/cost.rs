/// Calculates the estimated cost to train an LLM based on FLOPS and GPU pricing.
///
/// # Arguments
/// * `model_params_billions` - Number of parameters in billions
/// * `tokens_billions` - Number of training tokens in billions
/// * `gpu_hourly_cost` - Cost per GPU hour
/// * `gpu_flops_tera` - Achievable FLOPS per GPU in TFLOPS
///
/// # Returns
/// * Estimated Training Cost
pub fn calculate_training_cost(
    model_params_billions: f64,
    tokens_billions: f64,
    gpu_hourly_cost: f64,
    gpu_flops_tera: f64,
) -> Result<f64, String> {
    if model_params_billions <= 0.0 || tokens_billions <= 0.0 || gpu_hourly_cost < 0.0 || gpu_flops_tera <= 0.0 {
        return Err("Invalid inputs".into());
    }

    // Approximation: 6 * P * T FLOPS total
    let total_flops = 6.0 * (model_params_billions * 1e9) * (tokens_billions * 1e9);
    let gpu_flops_per_sec = gpu_flops_tera * 1e12;
    let gpu_seconds = total_flops / gpu_flops_per_sec;
    let gpu_hours = gpu_seconds / 3600.0;

    Ok(gpu_hours * gpu_hourly_cost)
}

/// Calculates inference cost for 1M tokens.
pub fn calculate_inference_cost(
    price_per_1k_input: f64,
    price_per_1k_output: f64,
    input_ratio: f64
) -> Result<f64, String> {
    if price_per_1k_input < 0.0 || price_per_1k_output < 0.0 {
        return Err("Prices cannot be negative".into());
    }
    let output_ratio = 1.0 - input_ratio;
    let cost = (1000.0 * input_ratio * price_per_1k_input) + (1000.0 * output_ratio * price_per_1k_output);
    Ok(cost)
}
