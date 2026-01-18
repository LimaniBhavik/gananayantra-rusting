use crate::calculators::utils::read_input;

pub fn run() {
    println!("\n--- CPM (Cost Per Mille) Calculator ---");
    println!("1. Calculate CPM");
    println!("2. Calculate Ad Cost");
    println!("3. Calculate Impressions");
    let choice = read_input("Select option (1-3): ");

    match choice as i32 {
        1 => {
            let cost = read_input("Enter total cost: ");
            let impressions = read_input("Enter total impressions: ");
            if impressions > 0.0 {
                println!("CPM: {:.2}", (cost / impressions) * 1000.0);
            }
        }
        2 => {
            let cpm = read_input("Enter CPM: ");
            let impressions = read_input("Enter total impressions: ");
            println!("Ad Cost: {:.2}", (cpm * impressions) / 1000.0);
        }
        3 => {
            let cost = read_input("Enter total cost: ");
            let cpm = read_input("Enter CPM: ");
            if cpm > 0.0 {
                println!("Impressions: {:.0}", (cost / cpm) * 1000.0);
            }
        }
        _ => println!("Invalid option."),
    }
}
