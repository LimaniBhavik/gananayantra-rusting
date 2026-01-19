use crate::calculators::utils::read_input;

pub fn run_menu() {
    loop {
        println!("\n--- Electronics and Circuits ---");
        println!("1. Ohm's Law (V = I * R)");
        println!("2. Resistor Color Code (Basic 4-band)");
        println!("3. Voltage Drop");
        println!("0. Back");
        let choice = read_input("Select an option: ");

        match choice as i32 {
            1 => ohms_law(),
            2 => resistor_calc(),
            3 => voltage_drop(),
            0 => break,
            _ => println!("Invalid choice."),
        }
    }
}

fn ohms_law() {
    println!("1. Find Voltage (V)  2. Find Current (I)  3. Find Resistance (R)");
    let c = read_input("Choice: ");
    if c == 1.0 {
        let i = read_input("Current (A): ");
        let r = read_input("Resistance (Ω): ");
        println!("Voltage: {:.2} V", i * r);
    } else if c == 2.0 {
        let v = read_input("Voltage (V): ");
        let r = read_input("Resistance (Ω): ");
        println!("Current: {:.2} A", v / r);
    } else {
        let v = read_input("Voltage (V): ");
        let i = read_input("Current (A): ");
        println!("Resistance: {:.2} Ω", v / i);
    }
}

fn resistor_calc() {
    println!("Enter digits for 4-band resistor:");
    let d1 = read_input("1st Digit: ");
    let d2 = read_input("2nd Digit: ");
    let mult = read_input("Multiplier (Power of 10): ");
    let val = (d1 * 10.0 + d2) * 10.0_f64.powf(mult);
    println!("Resistance: {:.0} Ω", val);
}

fn voltage_drop() {
    let i = read_input("Current (A): ");
    let r = read_input("Wire Resistance (Ω): ");
    println!("Voltage Drop: {:.2} V", i * r);
}
