/// Calculates Variance of a dataset.
pub fn variance(data: &[f64]) -> Result<f64, String> {
    if data.is_empty() { return Err("Data cannot be empty".into()); }
    let mean = data.iter().sum::<f64>() / data.len() as f64;
    let sum_sq_diff: f64 = data.iter().map(|x| (x - mean).powi(2)).sum();
    Ok(sum_sq_diff / data.len() as f64)
}

/// Detects Data Drift using simple mean shift (Z-test approximation).
/// Returns Z-score of the difference.
pub fn drift_detection_z_score(
    mean_train: f64,
    std_train: f64,
    mean_current: f64,
    sample_size_current: f64
) -> Result<f64, String> {
    if std_train == 0.0 || sample_size_current <= 0.0 {
        return Err("Invalid inputs for Z-test".into());
    }
    // Z = (M - mu) / (sigma / sqrt(n))
    Ok((mean_current - mean_train) / (std_train / sample_size_current.sqrt()))
}

/// Placeholder for PCA Efficiency (Explained Variance Ratio).
///
/// # Arguments
/// * `eigenvalues` - List of eigenvalues
/// * `k` - Number of components selected
///
/// # Returns
/// * Explained Variance Ratio (0.0 to 1.0)
pub fn pca_explained_variance(eigenvalues: &[f64], k: usize) -> Result<f64, String> {
    if k > eigenvalues.len() { return Err("k cannot be greater than total components".into()); }
    let total_variance: f64 = eigenvalues.iter().sum();
    if total_variance == 0.0 { return Err("Total variance is zero".into()); }
    let explained: f64 = eigenvalues.iter().take(k).sum();
    Ok(explained / total_variance)
}
