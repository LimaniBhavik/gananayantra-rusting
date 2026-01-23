use crate::calculators::utils::read_input;

pub fn run() {
    println!("\n--- Basic Math Calculator ---");
    let a = read_input("Enter first number: ");
    let b = read_input("Enter second number: ");
    println!("1. Add");
    println!("2. Subtract");
    println!("3. Multiply");
    println!("4. Divide");
    let choice = read_input("Select operation (1-4): ");

    match choice as i32 {
        1 => println!("Result: {}", a + b),
        2 => println!("Result: {}", a - b),
        3 => println!("Result: {}", a * b),
        4 => {
            if b != 0.0 {
                println!("Result: {}", a / b);
            } else {
                println!("Error: Division by zero.");
            }
        }
        _ => println!("Invalid operation."),
    }
}
