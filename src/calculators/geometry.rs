use crate::calculators::utils::read_input;
use std::f64::consts::PI;

pub fn run_menu() {
    loop {
        println!("\n--- Geometry Calculators ---");
        println!("1. Triangle Calculator (Area/Perimeter)");
        println!("2. Volume Calculator (Cube/Sphere/Cylinder)");
        println!("3. Slope Calculator");
        println!("4. Area Calculator (Circle/Square/Rectangle)");
        println!("5. Distance Formula (2D)");
        println!("6. Surface Area Calculator");
        println!("7. Pythagorean Theorem");
        println!("0. Back");
        let choice = read_input("Select an option: ");

        match choice as i32 {
            1 => triangle_calc(),
            2 => volume_calc(),
            3 => slope_calc(),
            4 => area_calc(),
            5 => distance_calc(),
            6 => surface_area_calc(),
            7 => pythagorean_calc(),
            0 => break,
            _ => println!("Invalid choice."),
        }
    }
}

fn triangle_calc() {
    println!("\n--- Triangle Calculator ---");
    let base = read_input("Base: ");
    let height = read_input("Height: ");
    println!("Area: {:.2}", 0.5 * base * height);
}

fn volume_calc() {
    println!("\n--- Volume Calculator ---");
    println!("1. Cube  2. Sphere  3. Cylinder");
    let choice = read_input("Choice: ");
    match choice as i32 {
        1 => {
            let s = read_input("Side: ");
            println!("Volume: {:.2}", s.powi(3));
        }
        2 => {
            let r = read_input("Radius: ");
            println!("Volume: {:.2}", (4.0/3.0) * PI * r.powi(3));
        }
        3 => {
            let r = read_input("Radius: ");
            let h = read_input("Height: ");
            println!("Volume: {:.2}", PI * r.powi(2) * h);
        }
        _ => println!("Invalid."),
    }
}

fn slope_calc() {
    println!("\n--- Slope Calculator ---");
    let x1 = read_input("x1: ");
    let y1 = read_input("y1: ");
    let x2 = read_input("x2: ");
    let y2 = read_input("y2: ");
    if x2 - x1 == 0.0 {
        println!("Slope is undefined (vertical line).");
    } else {
        println!("Slope (m): {:.2}", (y2 - y1) / (x2 - x1));
    }
}

fn area_calc() {
    println!("\n--- Area Calculator ---");
    println!("1. Circle  2. Rectangle");
    let choice = read_input("Choice: ");
    if choice == 1.0 {
        let r = read_input("Radius: ");
        println!("Area: {:.2}", PI * r.powi(2));
    } else {
        let l = read_input("Length: ");
        let w = read_input("Width: ");
        println!("Area: {:.2}", l * w);
    }
}

fn distance_calc() {
    println!("\n--- Distance Formula ---");
    let x1 = read_input("x1: ");
    let y1 = read_input("y1: ");
    let x2 = read_input("x2: ");
    let y2 = read_input("y2: ");
    let d = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
    println!("Distance: {:.2}", d);
}

fn surface_area_calc() {
    println!("\n--- Surface Area (Sphere) ---");
    let r = read_input("Radius: ");
    println!("Surface Area: {:.2}", 4.0 * PI * r.powi(2));
}

fn pythagorean_calc() {
    println!("\n--- Pythagorean Theorem ---");
    let a = read_input("Side a: ");
    let b = read_input("Side b: ");
    println!("Hypotenuse c: {:.2}", (a.powi(2) + b.powi(2)).sqrt());
}
