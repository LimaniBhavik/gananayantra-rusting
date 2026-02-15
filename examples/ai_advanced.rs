use gananayantra::ai::{
    calculate_model_memory,
    estimate_training_time,
    predict_latency,
    chatbot_response_time,
    cost_per_prediction,
    cloud_compute_cost,
    calculate_token_cost
};

fn main() {
    println!("--- Advanced AI Examples (Clean API) ---");

    // Model Memory
    // 70B model, 16-bit, 4k context
    match calculate_model_memory(70.0, 16.0, 4096.0, 8192.0, 80.0) {
        Ok(mem) => println!("Memory Required: {:.2} GB", mem),
        Err(e) => println!("Error: {}", e),
    }

    // Training Time
    // 6*P*T flops, 312 TFLOPS per GPU, 1000 GPUs, 50% util
    let total_flops = 6.0 * 70e9 * 2000e9; // 70B model, 2T tokens
    match estimate_training_time(total_flops, 312e12, 1000.0, 0.5) {
        Ok(hrs) => println!("Training Time: {:.2} hours ({:.1} days)", hrs, hrs/24.0),
        Err(e) => println!("Error: {}", e),
    }

    // Inference Latency
    match predict_latency(50.0, 5.0, 1.0) {
        Ok(ms) => println!("Inference Latency: {:.2} ms", ms),
        Err(e) => println!("Error: {}", e),
    }

    // Chatbot Application
    match chatbot_response_time(150.0, 40.0) {
        Ok(resp) => println!("Chatbot Total Response Time: {:.1} ms", resp),
        Err(e) => println!("Error: {}", e),
    }

    // Business Impact
    let predictions = 1_000_000.0;
    let cost = 500.0; // Total cost $500
    match cost_per_prediction(cost, predictions) {
        Ok(cpp) => println!("Cost per Prediction: ${:.6}", cpp),
        Err(e) => println!("Error: {}", e),
    }

    // Infrastructure Cost
    // 8x A100 instances for 720 hours @ $24/hr
    match cloud_compute_cost(8.0, 720.0, 24.0) {
        Ok(cloud_cost) => println!("Monthly Cloud Compute Cost: ${:.2}", cloud_cost),
        Err(e) => println!("Error: {}", e),
    }

    // Operations (Token Cost)
    // 1 billion tokens @ $5 per million
    match calculate_token_cost(1_000_000_000.0, 5.0) {
        Ok(token_cost) => println!("Cost for 1B tokens: ${:.2}", token_cost),
        Err(e) => println!("Error: {}", e),
    }
}
