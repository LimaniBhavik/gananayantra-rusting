/// Calculates Text Similarity (Jaccard Similarity for sets of words).
pub fn jaccard_similarity(set_a_size: usize, set_b_size: usize, intersection_size: usize) -> Result<f64, String> {
    let union_size = set_a_size + set_b_size - intersection_size;
    if union_size == 0 { return Ok(0.0); }
    Ok(intersection_size as f64 / union_size as f64)
}

/// Estimates Reading Time (WPM).
pub fn reading_time_minutes(word_count: usize, wpm: f64) -> Result<f64, String> {
    if wpm <= 0.0 { return Err("WPM must be > 0".into()); }
    Ok(word_count as f64 / wpm)
}

/// Text Summarization Compression Ratio.
pub fn summarization_ratio(original_words: usize, summary_words: usize) -> Result<f64, String> {
    if summary_words == 0 { return Ok(0.0); }
    Ok(original_words as f64 / summary_words as f64)
}
