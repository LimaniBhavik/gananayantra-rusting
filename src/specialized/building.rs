use crate::calculators::utils::read_input;

pub fn run_menu() {
    loop {
        println!("\n--- Housing and Building ---");
        println!("1. Square Footage Calculator");
        println!("2. Concrete Calculator");
        println!("3. BTU Calculator (Heating/Cooling)");
        println!("4. Tile/Flooring Calculator");
        println!("0. Back");
        let choice = read_input("Select an option: ");

        match choice as i32 {
            1 => sq_ft_calc(),
            2 => concrete_calc(),
            3 => btu_calc(),
            4 => tile_calc(),
            0 => break,
            _ => println!("Invalid choice."),
        }
    }
}

fn sq_ft_calc() {
    let l = read_input("Length (ft): ");
    let w = read_input("Width (ft): ");
    println!("Total: {:.2} sq ft", l * w);
}

fn concrete_calc() {
    let l = read_input("Length (ft): ");
    let w = read_input("Width (ft): ");
    let d = read_input("Thickness (inches): ");
    let cubic_yards = (l * w * (d / 12.0)) / 27.0;
    println!("Required Concrete: {:.2} cubic yards", cubic_yards);
}

fn btu_calc() {
    let area = read_input("Room Area (sq ft): ");
    println!("Estimated Cooling Needed: {:.0} BTU", area * 20.0);
}

fn tile_calc() {
    let area = read_input("Area to Tile (sq ft): ");
    let tile_size = read_input("Tile Size (sq inches, e.g. 144 for 12x12): ");
    let count = (area * 144.0) / tile_size;
    println!("Estimated Tiles Needed: {:.0} (plus 10% waste: {:.0})", count, count * 1.1);
}
