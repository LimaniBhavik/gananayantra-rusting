use gananayantra::utilities::percentage::percentage_of;

fn main() {
    let result = percentage_of(200.0, 15.0).unwrap();
    println!("Value: 200, Percent: 15%");
    println!("Result: {}", result);
}
