use gananayantra::logistics::freight::calculate_volumetric_weight;
use gananayantra::logistics::inventory::calculate_eoq;

fn main() {
    println!("--- Logistics Examples ---");

    // Volumetric Weight
    let length = 50.0;
    let width = 40.0;
    let height = 30.0;
    let divisor = 5000.0; // Air freight standard
    match calculate_volumetric_weight(length, width, height, divisor) {
        Ok(vw) => println!("Volumetric Weight (50x40x30, div 5000): {:.2} kg", vw),
        Err(e) => println!("Error: {}", e),
    }

    // EOQ
    let demand = 1000.0;
    let order_cost = 10.0;
    let holding_cost = 2.0;
    match calculate_eoq(demand, order_cost, holding_cost) {
        Ok(q) => println!(
            "Economic Order Quantity (D=1000, S=10, H=2): {:.2} units",
            q
        ),
        Err(e) => println!("Error: {}", e),
    }
}
