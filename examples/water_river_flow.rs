use gananayantra::water::river_flow_rate::river_flow_rate;

fn main() {
    let q = river_flow_rate(50.0, 2.5).unwrap();
    println!("Cross-sectional Area: 50.0 m^2");
    println!("Average Velocity: 2.5 m/s");
    println!("River Discharge (Flow Rate): {:.2} m^3/s", q);
}
