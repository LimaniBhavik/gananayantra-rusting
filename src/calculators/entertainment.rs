use crate::calculators::utils::read_input;

pub fn run_menu() {
    loop {
        println!("\n--- Entertainment ---");
        println!("1. Dice Roller");
        println!("2. Love Calculator (%)");
        println!("0. Back");
        let choice = read_input("Select an option: ");

        match choice as i32 {
            1 => dice_roller(),
            2 => love_calc(),
            0 => break,
            _ => println!("Invalid choice."),
        }
    }
}

fn dice_roller() {
    let sides = read_input("Sides on die: ") as i32;
    use std::time::{SystemTime, UNIX_EPOCH};
    let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros();
    println!("Result: {}", (seed % sides as u128) as i32 + 1);
}

fn love_calc() {
    println!("Enter names to calculate compatibility:");
    let _n1 = read_input("Name 1: "); // Just for flavor
    let _n2 = read_input("Name 2: ");
    use std::time::{SystemTime, UNIX_EPOCH};
    let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros();
    println!("Love Compatibility: {}%", (seed % 101) as i32);
}
