/// Calculates Accuracy.
pub fn accuracy(true_positives: f64, true_negatives: f64, total_samples: f64) -> Result<f64, String> {
    if total_samples <= 0.0 { return Err("Total samples must be positive".into()); }
    Ok((true_positives + true_negatives) / total_samples)
}

/// Calculates Precision.
pub fn precision(true_positives: f64, false_positives: f64) -> Result<f64, String> {
    if (true_positives + false_positives) == 0.0 { return Ok(0.0); }
    Ok(true_positives / (true_positives + false_positives))
}

/// Calculates Recall (Sensitivity).
pub fn recall(true_positives: f64, false_negatives: f64) -> Result<f64, String> {
    if (true_positives + false_negatives) == 0.0 { return Ok(0.0); }
    Ok(true_positives / (true_positives + false_negatives))
}

/// Calculates F1 Score.
pub fn f1_score(precision: f64, recall: f64) -> Result<f64, String> {
    if (precision + recall) == 0.0 { return Ok(0.0); }
    Ok(2.0 * (precision * recall) / (precision + recall))
}

/// Calculates Intersection over Union (IoU) for Object Detection.
pub fn iou(area_overlap: f64, area_union: f64) -> Result<f64, String> {
    if area_union <= 0.0 { return Err("Union area must be positive".into()); }
    Ok(area_overlap / area_union)
}

/// Calculates Perplexity from Cross-Entropy Loss.
pub fn perplexity(cross_entropy_loss: f64) -> Result<f64, String> {
    Ok(cross_entropy_loss.exp())
}

/// BLEU Score Approximation (Simplified unigram precision).
/// Note: Real BLEU requires n-gram matching logic which is complex for a math lib.
/// This is a placeholder for the metric calculation given a match count.
pub fn bleu_score_simple(matches: f64, total_ngrams: f64, brevity_penalty: f64) -> Result<f64, String> {
    if total_ngrams <= 0.0 { return Err("Total n-grams must be positive".into()); }
    Ok(brevity_penalty * (matches / total_ngrams))
}
