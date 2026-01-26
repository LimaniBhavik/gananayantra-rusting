use gananayantra::finance::compound_interest::compound_interest;

fn main() {
    let amount = compound_interest(5_000.0, 0.07, 10.0, 4).unwrap();
    println!("Final Amount: {:.2}", amount);
}
