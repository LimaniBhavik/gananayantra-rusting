use crate::calculators::utils::read_input;

pub fn run() {
    println!("\n--- E-Commerce Calculator ---");
    println!("1. Discount Calculator");
    println!("2. Profit Margin Calculator");
    let choice = read_input("Select option (1-2): ");

    match choice as i32 {
        1 => {
            let original_price = read_input("Enter original price: ");
            let discount_percent = read_input("Enter discount percentage: ");
            let savings = original_price * (discount_percent / 100.0);
            println!("Savings: {:.2}", savings);
            println!("Sale Price: {:.2}", original_price - savings);
        }
        2 => {
            let cost = read_input("Enter cost of item: ");
            let revenue = read_input("Enter selling price (revenue): ");
            if revenue > 0.0 {
                let gross_profit = revenue - cost;
                let margin = (gross_profit / revenue) * 100.0;
                println!("Gross Profit: {:.2}", gross_profit);
                println!("Profit Margin: {:.2}%", margin);
            }
        }
        _ => println!("Invalid option."),
    }
}
