use crate::calculators::utils::read_input;

pub fn run_menu() {
    loop {
        println!("\n--- Transportation ---");
        println!("1. Fuel Cost Calculator");
        println!("2. Gas Mileage (MPG)");
        println!("3. Horsepower Calculator");
        println!("0. Back");
        let choice = read_input("Select an option: ");

        match choice as i32 {
            1 => fuel_cost(),
            2 => mpg_calc(),
            3 => hp_calc(),
            0 => break,
            _ => println!("Invalid choice."),
        }
    }
}

fn fuel_cost() {
    let dist = read_input("Distance: ");
    let mpg = read_input("MPG: ");
    let price = read_input("Price per gallon: ");
    println!("Total Fuel Cost: {:.2}", (dist / mpg) * price);
}

fn mpg_calc() {
    let miles = read_input("Miles Driven: ");
    let gallons = read_input("Gallons Used: ");
    println!("MPG: {:.2}", miles / gallons);
}

fn hp_calc() {
    let torque = read_input("Torque (lb-ft): ");
    let rpm = read_input("RPM: ");
    println!("Horsepower: {:.2}", (torque * rpm) / 5252.0);
}
