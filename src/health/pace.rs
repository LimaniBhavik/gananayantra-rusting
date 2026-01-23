use crate::calculators::utils::read_input;

pub fn run() {
    println!("\n--- Pace Calculator ---");
    let distance = read_input("Enter distance (km): ");
    println!("Enter time taken:");
    let hours = read_input("  Hours: ");
    let minutes = read_input("  Minutes: ");
    let seconds = read_input("  Seconds: ");

    let total_seconds = hours * 3600.0 + minutes * 60.0 + seconds;
    if distance > 0.0 {
        let pace_seconds = total_seconds / distance;
        let p_min = (pace_seconds / 60.0).floor();
        let p_sec = (pace_seconds % 60.0).round();
        println!("Your pace is: {:.0}:{:02.0} per km", p_min, p_sec);
        println!("Speed: {:.2} km/h", (distance / (total_seconds / 3600.0)));
    }
}
