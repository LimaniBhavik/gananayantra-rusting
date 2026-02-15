use gananayantra::edge_ai::{iot_battery_life, edge_inference_latency, quantization_memory_saving};

fn main() {
    println!("--- Edge AI Calculators ---");

    // Battery Life
    let capacity_mah = 5000.0; // Typical power bank
    let active_ma = 200.0; // Running inference
    let sleep_ma = 5.0; // Idle
    let duty_cycle = 0.1; // 10% active
    match iot_battery_life(capacity_mah, active_ma, sleep_ma, duty_cycle) {
        Ok(hours) => println!("Battery Life: {:.1} hours ({:.1} days)", hours, hours / 24.0),
        Err(e) => println!("Error: {}", e),
    }

    // Latency
    let model_ms = 15.0;
    let network_ms = 50.0;
    match edge_inference_latency(model_ms, network_ms) {
        Ok(total) => println!("Total Latency: {:.1} ms", total),
        Err(e) => println!("Error: {}", e),
    }

    // Quantization
    match quantization_memory_saving(32.0, 8.0) {
        Ok(saving) => println!("Quantization Saving (FP32 -> INT8): {:.1}%", saving * 100.0),
        Err(e) => println!("Error: {}", e),
    }
}
