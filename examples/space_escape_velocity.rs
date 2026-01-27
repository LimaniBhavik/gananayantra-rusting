use gananayantra::space::escape_velocity::escape_velocity;

fn main() {
    let v = escape_velocity(0.0).unwrap();
    println!("Altitude: Surface of Earth (0m)");
    println!("Escape Velocity: {:.2} m/s", v);
    println!("Escape Velocity: {:.2} km/s", v / 1000.0);
}
