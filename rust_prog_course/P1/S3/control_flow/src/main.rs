// -------------------------------------------
// 			- Loops
// 			- For loops
// 			- while loops
// ------------------------------------------

mod practice_1;
mod practice_2;

fn main() {
    println!("Hello, world!");
    loop {
        println!("This is coming from the loop!");
        break;
    }

    'outer: loop {
        println!("Using outer loop!");
        break 'outer;
    }

    // using for loop
    let vec = vec![10, 20, -34, 32, 34];

    for i in vec {
        println!("{i}");
    }

    // Using while loop
    let mut num = 10;
    while num < 10 {
        num = num + 1;
        println!("num is now {num}");
    }
}
