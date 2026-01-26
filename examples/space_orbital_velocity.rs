use gananayantra::space::orbital_velocity::orbital_velocity;

fn main() {
    let v = orbital_velocity(400_000.0).unwrap();
    println!("Altitude: 400km (LEO)");
    println!("Orbital Velocity: {:.2} m/s", v);
}
