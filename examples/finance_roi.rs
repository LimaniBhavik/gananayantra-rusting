use gananayantra::finance::roi::roi;

fn main() {
    println!("--- ROI Library Example ---");
    let result = roi(5000.0, 6500.0).unwrap();
    println!("Initial: $5000, Final: $6500");
    println!("ROI: {:.2}%", result);
}
