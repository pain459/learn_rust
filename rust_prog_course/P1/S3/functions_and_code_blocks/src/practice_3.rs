fn print_distance(point: (f32, f32)) -> f32 {
    let (x, y) = point;
    (x.powf(2.0) + y.powf(2.0)).sqrt()  // formula for computing distance
}

fn main() {
    println!(
        "The distance of the number the point from the origin is {}",
        print_distance((5.0, 4.0))
    )
}