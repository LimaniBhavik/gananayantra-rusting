use gananayantra::space::orbital_period::orbital_period;

fn main() {
    let alt = 400_000.0; // 400km LEO
    match orbital_period(alt) {
        Ok(t) => {
            println!("Altitude: {} meters", alt);
            println!("Orbital Period: {:.2} minutes", t);
        }
        Err(e) => println!("Error: {}", e),
    }
}
