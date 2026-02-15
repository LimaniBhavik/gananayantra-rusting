use gananayantra::economics::ai_roi::{calculate_ai_roi, calculate_ai_cac};

fn main() {
    println!("--- Economics Examples ---");

    match calculate_ai_roi(1_000_000.0, 200_000.0) {
        Ok(roi) => println!("AI ROI: {:.2}%", roi),
        Err(e) => println!("Error: {}", e),
    }

    match calculate_ai_cac(50_000.0, 10_000.0, 500.0) {
        Ok(cac) => println!("AI-Driven CAC: ${:.2}", cac),
        Err(e) => println!("Error: {}", e),
    }
}
