use gananayantra::data_science::metrics::{accuracy, f1_score, iou};
use gananayantra::data_science::stats::drift_detection_z_score;

fn main() {
    println!("--- Data Science Metrics ---");

    // Metrics
    match f1_score(0.8, 0.7) {
        Ok(f1) => println!("F1 Score (P=0.8, R=0.7): {:.2}", f1),
        Err(e) => println!("Error: {}", e),
    }

    match iou(50.0, 100.0) {
        Ok(res) => println!("IoU (Overlap=50, Union=100): {:.2}", res),
        Err(e) => println!("Error: {}", e),
    }

    // Drift
    match drift_detection_z_score(0.5, 0.1, 0.6, 100.0) {
        Ok(z) => println!("Drift Z-Score: {:.2}", z),
        Err(e) => println!("Error: {}", e),
    }
}
