use gananayantra::semiconductor::yields::{calculate_gross_dies, calculate_die_yield_murphy, calculate_net_dies};
use gananayantra::semiconductor::cost::calculate_cost_per_die;

fn main() {
    println!("--- Semiconductor Yield Analysis ---");

    let wafer_diam = 300.0; // mm
    let die_area = 120.0;   // mm^2
    let defect_density = 0.1; // per cm^2
    let wafer_cost = 5000.0; // USD

    let gross_dies = calculate_gross_dies(wafer_diam, die_area).unwrap();
    println!("Gross Dies per Wafer: {}", gross_dies);

    let yield_percent = calculate_die_yield_murphy(defect_density, die_area).unwrap();
    println!("Die Yield (Murphy): {:.2}%", yield_percent);

    let net_dies = calculate_net_dies(gross_dies, yield_percent).unwrap();
    println!("Net Good Dies: {}", net_dies);

    let cost_per_die = calculate_cost_per_die(wafer_cost, net_dies).unwrap();
    println!("Cost Per Good Die: ${:.2}", cost_per_die);
}
