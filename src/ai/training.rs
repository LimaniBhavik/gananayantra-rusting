/// Estimates total training time.
///
/// # Arguments
/// * `total_flops` - Total FLOPs required (e.g. 6 * P * T)
/// * `gpu_flops` - Achieved FLOPs per GPU
/// * `num_gpus` - Number of GPUs
/// * `utilization` - Utilization rate (0.0 to 1.0)
///
/// # Returns
/// * Time in hours
pub fn estimate_training_time(
    total_flops: f64,
    gpu_flops: f64,
    num_gpus: f64,
    utilization: f64
) -> Result<f64, String> {
    if num_gpus <= 0.0 || utilization <= 0.0 {
        return Err("GPUs and utilization must be positive".into());
    }
    let effective_flops = gpu_flops * num_gpus * utilization;
    Ok((total_flops / effective_flops) / 3600.0)
}

/// Calculates effective learning rate after decay.
pub fn learning_rate_decay(initial_lr: f64, decay_rate: f64, steps: f64) -> Result<f64, String> {
    // Exponential decay: lr = lr0 * e^(-k*t)
    Ok(initial_lr * (-decay_rate * steps).exp())
}

/// Estimates required epochs for convergence (Heuristic).
///
/// # Arguments
/// * `dataset_size` - Number of samples
/// * `batch_size` - Batch size
/// * `complexity_factor` - Model complexity (1.0 = standard)
///
/// # Returns
/// * Estimated Epochs
pub fn estimate_epochs(dataset_size: f64, batch_size: f64, complexity_factor: f64) -> Result<f64, String> {
    if batch_size <= 0.0 { return Err("Batch size > 0".into()); }
    let iterations_per_epoch = dataset_size / batch_size;
    // Heuristic: More complex models needs more iterations relative to dataset
    Ok(iterations_per_epoch.sqrt() * complexity_factor / 10.0)
}
