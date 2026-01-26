use gananayantra::energy::electricity_cost::electricity_cost;

fn main() {
    let cost = electricity_cost(12.0, 6.0).unwrap();
    println!("Energy: 12.0 kWh, Rate: 6.0/kWh");
    println!("Electricity Cost: {}", cost);
}
