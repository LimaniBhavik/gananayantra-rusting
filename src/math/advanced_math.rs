use crate::calculators::utils::read_input;

pub fn run_menu() {
    loop {
        println!("\n--- Advanced Math ---");
        println!("1. Percentage Calculator");
        println!("2. Random Number Generator");
        println!("3. Exponent & Root");
        println!("4. Logarithm (log10/ln)");
        println!("5. Quadratic Formula");
        println!("6. Binary/Hex Converter");
        println!("7. GCD & LCM");
        println!("8. Rounding Calculator");
        println!("9. Fraction Calculator (Basic)");
        println!("0. Back");
        let choice = read_input("Select an option: ");

        match choice as i32 {
            1 => percentage_calc(),
            2 => random_gen(),
            3 => exponent_root(),
            4 => log_calc(),
            5 => quadratic_calc(),
            6 => base_converter(),
            7 => gcd_lcm_calc(),
            8 => rounding_calc(),
            9 => fraction_calc(),
            0 => break,
            _ => println!("Invalid choice."),
        }
    }
}

fn percentage_calc() {
    println!("1. What is X% of Y?");
    println!("2. X is what % of Y?");
    let mode = read_input("Choice: ");
    let x = read_input("X: ");
    let y = read_input("Y: ");
    if mode == 1.0 {
        println!("Result: {:.2}", (x / 100.0) * y);
    } else {
        println!("Result: {:.2}%", (x / y) * 100.0);
    }
}

fn random_gen() {
    let min = read_input("Min: ") as i32;
    let max = read_input("Max: ") as i32;
    // Simple pseudo-random using timestamp since we avoid extra crates
    use std::time::{SystemTime, UNIX_EPOCH};
    let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros();
    let res = (seed % (max - min + 1) as u128) as i32 + min;
    println!("Random Number: {}", res);
}

fn exponent_root() {
    let base = read_input("Base: ");
    let exp = read_input("Exponent: ");
    println!("{} ^ {} = {:.4}", base, exp, base.powf(exp));
    println!("{} root of {} = {:.4}", exp, base, base.powf(1.0/exp));
}

fn log_calc() {
    let n = read_input("Number: ");
    println!("log10: {:.4}", n.log10());
    println!("ln: {:.4}", n.ln());
}

fn quadratic_calc() {
    let a = read_input("a: ");
    let b = read_input("b: ");
    let c = read_input("c: ");
    let d = b.powi(2) - 4.0 * a * c;
    if d < 0.0 {
        println!("No real roots.");
    } else {
        let r1 = (-b + d.sqrt()) / (2.0 * a);
        let r2 = (-b - d.sqrt()) / (2.0 * a);
        println!("Roots: {:.2}, {:.2}", r1, r2);
    }
}

fn base_converter() {
    let n = read_input("Decimal Number: ") as i64;
    println!("Binary: {:b}", n);
    println!("Hex: {:X}", n);
}

fn gcd_lcm_calc() {
    let mut a = read_input("a: ") as i64;
    let mut b = read_input("b: ") as i64;
    let oa = a;
    let ob = b;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    println!("GCD: {}", a);
    println!("LCM: {}", (oa * ob).abs() / a);
}

fn rounding_calc() {
    let n = read_input("Number: ");
    println!("Floor: {}", n.floor());
    println!("Ceil: {}", n.ceil());
    println!("Round: {}", n.round());
}

fn fraction_calc() {
    let n1 = read_input("Numerator 1: ");
    let d1 = read_input("Denominator 1: ");
    let n2 = read_input("Numerator 2: ");
    let d2 = read_input("Denominator 2: ");
    println!("Sum: {:.2}/{:.2}", n1*d2 + n2*d1, d1*d2);
}
