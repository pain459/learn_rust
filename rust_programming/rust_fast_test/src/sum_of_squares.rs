use std::time::Instant;

fn sum_of_squares(n: u64) -> u128 {
    let mut total: u128 = 0;
    for i in 0..n {
        total += (i as u128) * (i as u128);
    }
    total
}

fn main() {
    let n: u64 = 10_000_000;
    let start_time = Instant::now();
    let result = sum_of_squares(n);
    let end_time = Instant::now();
    let duration = end_time.duration_since(start_time);
    println!("Sum of squares: {}", result);
    println!("Execution time: {:?}", duration);
}
