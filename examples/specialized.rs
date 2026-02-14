use gananayantra::specialized::building::{calculate_board_feet, calculate_paint_gallons};
use gananayantra::specialized::network_cctv::ipv4_hosts;

fn main() {
    println!("--- Specialized Industry Examples ---");

    // Building
    let wall_area = 1000.0;
    let coats = 2.0;
    match calculate_paint_gallons(wall_area, coats) {
        Ok(gal) => println!("Paint Needed (1000 sqft, 2 coats): {:.2} gallons", gal),
        Err(e) => println!("Error: {}", e),
    }

    match calculate_board_feet(2.0, 4.0, 8.0, 10) {
        Ok(bf) => println!("Lumber (10 pcs 2x4x8): {:.2} board feet", bf),
        Err(e) => println!("Error: {}", e),
    }

    // Network
    let cidr = 24;
    match ipv4_hosts(cidr) {
        Ok(hosts) => println!("IPv4 Hosts in /24 subnet: {}", hosts),
        Err(e) => println!("Error: {}", e),
    }
}
