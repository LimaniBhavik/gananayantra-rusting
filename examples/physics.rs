use gananayantra::physics::fluid::calculate_reynolds_number;
use gananayantra::physics::kinematics::{calculate_kinetic_energy, calculate_potential_energy};

fn main() {
    println!("--- Physics Examples ---");

    // Reynolds Number
    let velocity = 10.0; // m/s
    let length = 0.5; // m
    let viscosity = 1e-6; // m^2/s (approx water)
    match calculate_reynolds_number(velocity, length, viscosity) {
        Ok(re) => println!("Reynolds Number: {:.2e}", re),
        Err(e) => println!("Error: {}", e),
    }

    // Kinetic Energy
    let mass = 10.0; // kg
    let v = 5.0; // m/s
    match calculate_kinetic_energy(mass, v) {
        Ok(ke) => println!("Kinetic Energy (10kg, 5m/s): {:.2} J", ke),
        Err(e) => println!("Error: {}", e),
    }

    // Potential Energy
    let h = 20.0; // m
    match calculate_potential_energy(mass, h) {
        Ok(pe) => println!("Potential Energy (10kg, 20m): {:.2} J", pe),
        Err(e) => println!("Error: {}", e),
    }
}
