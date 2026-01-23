use crate::calculators::utils::read_input;

pub fn run_menu() {
    loop {
        println!("\n--- Internet Tools ---");
        println!("1. Password Generator");
        println!("2. Bandwidth Calculator");
        println!("0. Back");
        let choice = read_input("Select an option: ");

        match choice as i32 {
            1 => password_gen(),
            2 => bandwidth_calc(),
            0 => break,
            _ => println!("Invalid choice."),
        }
    }
}

fn password_gen() {
    let len = read_input("Length: ") as usize;
    let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()";
    use std::time::{SystemTime, UNIX_EPOCH};
    let mut seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros();
    let mut pass = String::new();
    for _ in 0..len {
        seed = (seed * 1103515245 + 12345) & 0x7fffffff;
        let idx = (seed % chars.len() as u128) as usize;
        pass.push(chars.chars().nth(idx).unwrap());
    }
    println!("Generated Password: {}", pass);
}

fn bandwidth_calc() {
    let size_gb = read_input("File Size (GB): ");
    let speed_mbps = read_input("Speed (Mbps): ");
    let seconds = (size_gb * 8192.0) / speed_mbps;
    println!("Estimated Transfer Time: {:.2} seconds ({:.2} minutes)", seconds, seconds / 60.0);
}
