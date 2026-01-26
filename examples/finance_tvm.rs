use gananayantra::finance::tvm::future_value;

fn main() {
    let fv = future_value(10_000.0, 0.08, 5).unwrap();
    println!("Future Value: {:.2}", fv);
}
