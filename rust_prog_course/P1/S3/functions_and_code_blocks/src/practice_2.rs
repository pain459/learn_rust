fn double(x: i32) -> i32 {
    x * 2
}

fn triple(y: i32) -> i32 {
    y * 3
}

fn main() {
    println!("Result comes here");
    let result: i32 = double(5) * triple(10);
    println!("Result is {result}");
}