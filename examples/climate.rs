use gananayantra::climate::carbon::{
    calculate_carbon_footprint_electricity, calculate_carbon_footprint_fuel,
};

fn main() {
    println!("--- Climate Examples ---");

    // Electricity
    let kwh = 500.0;
    match calculate_carbon_footprint_electricity(kwh) {
        Ok(kg) => println!("Carbon Footprint ({} kWh): {:.2} kg CO2e", kwh, kg),
        Err(e) => println!("Error: {}", e),
    }

    // Fuel
    let gallons = 20.0;
    match calculate_carbon_footprint_fuel(gallons) {
        Ok(kg) => println!("Carbon Footprint ({} gal gas): {:.2} kg CO2e", gallons, kg),
        Err(e) => println!("Error: {}", e),
    }
}
