use gananayantra::robotics::kinematics::calculate_dof_planar;
use gananayantra::robotics::energy::calculate_robot_runtime;

fn main() {
    println!("--- Robotics Examples ---");

    // Kinematics
    match calculate_dof_planar(4.0, 4.0, 1.0) {
        Ok(dof) => println!("Planar DOF (4-bar): {}", dof),
        Err(e) => println!("Error: {}", e),
    }

    // Energy
    match calculate_robot_runtime(500.0, 50.0, 0.2) {
        Ok(hrs) => println!("Runtime (500Wh, 50W load): {:.2} hours", hrs),
        Err(e) => println!("Error: {}", e),
    }
}
