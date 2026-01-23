use crate::calculators::utils::read_input;

pub fn run_menu() {
    loop {
        println!("\n--- Everyday Utility ---");
        println!("1. GPA Calculator");
        println!("2. Tip Calculator");
        println!("3. Shoe Size Conversion");
        println!("4. Sleep Calculator");
        println!("0. Back");
        let choice = read_input("Select an option: ");

        match choice as i32 {
            1 => gpa_calc(),
            2 => tip_calc(),
            3 => shoe_size(),
            4 => sleep_calc(),
            0 => break,
            _ => println!("Invalid choice."),
        }
    }
}

fn gpa_calc() {
    let count = read_input("Number of courses: ");
    let mut total_points = 0.0;
    for i in 1..=(count as i32) {
        let grade = read_input(&format!("Grade for course {} (4.0 scale): ", i));
        let credits = read_input(&format!("Credits for course {}: ", i));
        total_points += grade * credits;
    }
    let total_credits = read_input("Total Credits: ");
    println!("GPA: {:.2}", total_points / total_credits);
}

fn tip_calc() {
    let bill = read_input("Bill Amount: ");
    let tip_p = read_input("Tip %: ");
    let people = read_input("Number of people: ");
    let tip = bill * (tip_p / 100.0);
    println!("Total Tip: {:.2}", tip);
    println!("Total Bill: {:.2}", bill + tip);
    println!("Per Person: {:.2}", (bill + tip) / people);
}

fn shoe_size() {
    let us_size = read_input("US Men's Size: ");
    println!("UK Size: {:.1}", us_size - 0.5);
    println!("EU Size: {:.1}", us_size + 33.0);
}

fn sleep_calc() {
    println!("If you want to wake up at X:00, you should go to bed at:");
    let wake_h = read_input("Wake up hour (24h): ");
    // Cycles of 90 mins, 5-6 cycles
    for cycles in [5, 6] {
        let total_min = cycles * 90;
        let mut bed_h = (wake_h as i32 - (total_min / 60)) % 24;
        if bed_h < 0 { bed_h += 24; }
        println!("For {} cycles ({}h): {:02}:00", cycles, cycles as f32 * 1.5, bed_h);
    }
}
