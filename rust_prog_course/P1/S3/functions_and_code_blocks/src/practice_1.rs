fn main() {
    let x = 3;
    let y = 4;
    println!("Testing output x = {x} and y = {y}");
    let result_add_3 = add_3(x);
    println!("Result after adding {x} to x is {result_add_3}");
    let result_add_5 = add_5(y);
    println!("Result after added 5 to y is {result_add_5}");

}

fn add_3(x: i32) -> i32 {
    x + 3
}

fn add_5(x: i32) -> i32 {
    x + 5
}