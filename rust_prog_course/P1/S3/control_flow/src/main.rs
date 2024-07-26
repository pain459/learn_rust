// -------------------------------------------
// 			- Loops
// 			- For loops
// 			- while loops
// ------------------------------------------

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
}
