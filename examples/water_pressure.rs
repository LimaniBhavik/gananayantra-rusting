use gananayantra::water::pressure_at_depth::pressure_at_depth;

fn main() {
    let pressure = pressure_at_depth(1000.0, 10.0).unwrap();
    println!("Fluid Density: 1000 kg/m^3 (Water)");
    println!("Depth: 10 meters");
    println!("Hydrostatic Pressure: {:.2} Pa", pressure);
}
