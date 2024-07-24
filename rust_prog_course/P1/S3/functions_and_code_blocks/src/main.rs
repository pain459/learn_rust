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

    let (addition, subtraction, multiplication) = basic_math(12, 34);
    println!("Result of addition is {addition}\nResult of subtraction is {subtraction}\nResult of multiplication is {multiplication}");
}

fn my_fn(s: &str) {
    println!("{s}");
}

fn basic_multiplication(num1: i32, num2: i32) -> i32 {
    println!("Computing multiplication!");
    num1 * num2
}

// Multi return
fn basic_math(num1: i32, num2: i32) -> (i32, i32, i32) {
    println!("Basic math results!");
    (num1 + num2, num1 - num2, num1 * num2)
}