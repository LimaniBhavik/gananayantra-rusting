/// Calculates Chatbot Response Time (Processing + Network).
pub fn chatbot_response_time(processing_ms: f64, network_latency_ms: f64) -> Result<f64, String> {
    Ok(processing_ms + network_latency_ms)
}

/// Calculates Recommendation Engine Accuracy (Precision @ K).
pub fn recommendation_accuracy(relevant_items_in_top_k: f64, k: f64) -> Result<f64, String> {
    if k <= 0.0 { return Err("K must be > 0".into()); }
    Ok(relevant_items_in_top_k / k)
}

/// Calculates Customer Lifetime Value (CLV) Predictor.
pub fn clv_prediction(avg_order_value: f64, purchase_frequency: f64, lifespan: f64) -> Result<f64, String> {
    Ok(avg_order_value * purchase_frequency * lifespan)
}
