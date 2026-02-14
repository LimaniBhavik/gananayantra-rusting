use gananayantra::ai::cost::{calculate_training_cost, calculate_inference_cost};
use gananayantra::ai::energy::calculate_ai_carbon_footprint;

fn main() {
    println!("--- AI Compute Economics ---");

    // Training Cost
    let params = 7.0; // 7B model
    let tokens = 1000.0; // 1T tokens
    let gpu_cost = 2.5; // $2.5/hr (A100)
    let flops = 312.0; // TFLOPS (A100 peak approx)

    match calculate_training_cost(params, tokens, gpu_cost, flops) {
        Ok(cost) => println!("Estimated Training Cost (7B model, 1T tokens): ${:.2}", cost),
        Err(e) => println!("Error: {}", e),
    }

    // Inference
    let cost_in = 0.03; // $0.03 per 1k
    let cost_out = 0.06;
    let ratio = 0.8; // 80% input tokens
    match calculate_inference_cost(cost_in, cost_out, ratio) {
        Ok(cost) => println!("Cost per 1M Tokens (80/20 split): ${:.2}", cost),
        Err(e) => println!("Error: {}", e),
    }

    // Carbon
    let power = 400.0; // W
    let gpus = 100.0;
    let hours = 240.0;
    let pue = 1.1;
    let grid_intensity = 400.0; // gCO2/kWh
    match calculate_ai_carbon_footprint(power, gpus, hours, pue, grid_intensity) {
        Ok(kg) => println!("Training Carbon Footprint: {:.2} kgCO2", kg),
        Err(e) => println!("Error: {}", e),
    }
}
