/// Calculates Dataset Size in GB.
pub fn calculate_dataset_size(num_samples: f64, avg_sample_size_kb: f64) -> Result<f64, String> {
    Ok((num_samples * avg_sample_size_kb) / (1024.0 * 1024.0))
}

/// Optimizes Batch Size (Heuristic based on memory).
pub fn batch_size_optimizer(available_memory_gb: f64, model_size_gb: f64, activation_per_sample_gb: f64) -> Result<f64, String> {
    if activation_per_sample_gb <= 0.0 { return Err("Activation size must be > 0".into()); }
    let usable = available_memory_gb - model_size_gb;
    if usable <= 0.0 { return Ok(0.0); }
    Ok((usable / activation_per_sample_gb).floor())
}

/// Estimates cost of Fine-tuning.
pub fn fine_tuning_cost(base_training_cost: f64, data_fraction: f64, epochs: f64) -> Result<f64, String> {
    // Fine-tuning is usually a fraction of pre-training on a smaller subset for fewer epochs
    Ok(base_training_cost * data_fraction * epochs)
}
