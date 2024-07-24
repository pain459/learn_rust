// -------------------------------------------------
// 			- Functions
// 			- Code Blocks
// -------------------------------------------------

fn main() {
    my_fn("This is from my_fn function!");
    let str = "This is a custom string passed to the function!";
    my_fn(str);

    let multiplication_result = basic_multiplication(123, 345);
    println!("Result is {multiplication_result}");
}

fn my_fn(s: &str) {
    println!("{s}");
}

fn basic_multiplication(num1: i32, num2: i32) -> i32 {
    println!("Computing multiplication!");
    num1 * num2
}