use std::io;
mod goto_bmi_calculate;

fn main() {
    // Welcome Message
    println!("=========================================");
    println!("Hello, Welcome to Gananayantra!\n");
    println!("Please select what you want to complete?");
    println!("=========================================");

    // List of Calculation for Fitness & Health Calculators
    // Variable to get store the user input
    let mut user_selection = String::new();

    println!("1. BMI Calculator");
    println!("2. Calorie Calculator");
    println!("3. Body Fat Calculator");
    println!("4. BMR Calculator");
    println!("5. Ideal Weight Calculator");
    println!("6. Pace Calculator");
    println!("7. Pregnancy Calculator");
    println!("8. Pregnancy Conception Calculator");
    println!("9. Due Date Calculator");

    // Get the User Selection
    println!("Please enter Number you want to Calculate :");

    // Reads the input from STDIN and places it in the String named input.
    println!("Enter a value:");
    io::stdin().read_line(&mut user_selection).unwrap();
    
    // convert line to integer
    let converted_number : i32 = user_selection.trim().parse().unwrap();

    // Go to the Calculation based on User Input
    print!(" Your Selection is {} \n", converted_number);
    if converted_number == 1{
        goto_bmi_calculate::calculate_bmi();
    }
    // else if user_selection == 2{}
    // else if user_selection == 3{}
    // else if user_selection == 4{}
    // else if user_selection == 5{}
    // else if user_selection == 6{}
    // else if user_selection == 7{}
    // else if user_selection == 8{}
    // else if user_selection == 9{}
    // else{}
}
   