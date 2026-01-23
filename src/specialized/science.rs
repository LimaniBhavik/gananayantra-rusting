use crate::calculators::utils::read_input;

pub fn run_menu() {
    loop {
        println!("\n--- Science and Measurements ---");
        println!("1. Density Calculator");
        println!("2. Speed/Distance/Time");
        println!("3. Roman Numeral Converter (Decimal to Roman)");
        println!("4. GDP Growth Calculator");
        println!("0. Back");
        let choice = read_input("Select an option: ");

        match choice as i32 {
            1 => density_calc(),
            2 => speed_calc(),
            3 => roman_calc(),
            4 => gdp_calc(),
            0 => break,
            _ => println!("Invalid choice."),
        }
    }
}

fn density_calc() {
    let mass = read_input("Mass (kg): ");
    let vol = read_input("Volume (m³): ");
    println!("Density: {:.2} kg/m³", mass / vol);
}

fn speed_calc() {
    let dist = read_input("Distance: ");
    let time = read_input("Time: ");
    println!("Speed: {:.2}", dist / time);
}

fn roman_calc() {
    let mut num = read_input("Number (1-3999): ") as i32;
    let vals = [(1000, "M"), (900, "CM"), (500, "D"), (400, "CD"), (100, "C"), (90, "XC"), (50, "L"), (40, "XL"), (10, "X"), (9, "IX"), (5, "V"), (4, "IV"), (1, "I")];
    let mut res = String::new();
    for (v, s) in vals {
        while num >= v {
            res.push_str(s);
            num -= v;
        }
    }
    println!("Roman Numeral: {}", res);
}

fn gdp_calc() {
    let old = read_input("Old GDP: ");
    let new = read_input("New GDP: ");
    println!("Growth Rate: {:.2}%", ((new - old) / old) * 100.0);
}
