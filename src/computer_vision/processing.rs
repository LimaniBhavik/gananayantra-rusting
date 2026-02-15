/// Calculates Video Processing Time.
///
/// # Arguments
/// * `duration_sec` - Video duration in seconds
/// * `fps` - Frames per second
/// * `processing_time_per_frame_ms` - Time to process one frame
///
/// # Returns
/// * Total processing time in seconds
pub fn video_processing_time(duration_sec: f64, fps: f64, processing_time_per_frame_ms: f64) -> Result<f64, String> {
    if duration_sec < 0.0 || fps <= 0.0 || processing_time_per_frame_ms < 0.0 {
        return Err("Inputs must be positive".into());
    }
    let total_frames = duration_sec * fps;
    Ok((total_frames * processing_time_per_frame_ms) / 1000.0)
}

/// Estimates Image Compression Quality (PSNR approximation).
/// This is a heuristic placeholder as PSNR requires raw pixel data.
pub fn image_compression_ratio(original_size_kb: f64, compressed_size_kb: f64) -> Result<f64, String> {
    if compressed_size_kb <= 0.0 { return Err("Compressed size must be > 0".into()); }
    Ok(original_size_kb / compressed_size_kb)
}

/// Frame Rate Optimizer (Max FPS supported).
pub fn max_supported_fps(processing_time_ms: f64) -> Result<f64, String> {
    if processing_time_ms <= 0.0 { return Err("Processing time must be > 0".into()); }
    Ok(1000.0 / processing_time_ms)
}
