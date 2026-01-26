use gananayantra::geo::earth_distance::haversine_distance;

fn main() {
    let km = haversine_distance(23.0225, 72.5714, 19.0760, 72.8777).unwrap();
    println!("Distance between Ahmedabad and Mumbai: {:.2} km", km);
}
