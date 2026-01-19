use crate::calculators::utils::read_input;

pub fn run_menu() {
    loop {
        println!("\n--- Weather Tools ---");
        println!("1. Wind Chill");
        println!("2. Heat Index");
        println!("0. Back");
        let choice = read_input("Select an option: ");

        match choice as i32 {
            1 => wind_chill(),
            2 => heat_index(),
            0 => break,
            _ => println!("Invalid choice."),
        }
    }
}

fn wind_chill() {
    let t = read_input("Temperature (F): ");
    let v = read_input("Wind Speed (mph): ");
    let chill = 35.74 + 0.6215*t - 35.75*v.powf(0.16) + 0.4275*t*v.powf(0.16);
    println!("Wind Chill: {:.1} F", chill);
}

fn heat_index() {
    let t = read_input("Temperature (F): ");
    let r = read_input("Relative Humidity (%): ");
    let hi = -42.379 + 2.049*t + 10.14*r - 0.224*t*r - 0.0068*t*t - 0.054*r*r + 0.0012*t*t*r + 0.00085*t*r*r - 0.0000019*t*t*r*r;
    println!("Heat Index: {:.1} F", hi);
}
