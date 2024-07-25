fn double(x: i32) -> i32 {
    x * 2
}

fn quadruple(x: i32) -> i32 {
    2 * double(x)
}

fn main() {
    println!("Calculating quadruples.");
    println!(
        "For 1 the expected value is 4, the calculated value is {}",
    quadruple(1));

    println!(
        "For 2 the expected value is 8, the calculated value is {}",
        quadruple(2));

    println!(
        "For 3 the expected value is 12, the calculated value is {}",
        quadruple(3));

    println!(
        "For 4 the expected value is 16, the calculated value is {}",
        quadruple(4));
}