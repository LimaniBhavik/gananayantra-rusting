use gananayantra::geo::horizon_distance::horizon_distance;

fn main() {
    let km = horizon_distance(10.0).unwrap();
    println!("Observer Height: 10m");
    println!("Distance to Horizon: {:.2} km", km);
}
