use gananayantra::energy::power_consumption::energy_consumption;

fn main() {
    let kwh = energy_consumption(1.5, 8.0).unwrap();
    println!("Power: 1.5 kW, Time: 8 hours");
    println!("Energy Consumption: {:.2} kWh", kwh);
}
