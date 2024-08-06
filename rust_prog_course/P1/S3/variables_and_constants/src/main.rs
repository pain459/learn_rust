// -------------------------------------------------
// 			- Variables
// 			        - Definition
// 			        - Mutability
// 			        - Scope
// 			        - Shadowing
// 			 - Constants
// -------------------------------------------------

mod test;

fn main() {
    // Definition
    let i = 10;
    let i: i8 = 10;
    println!("value of i is {i}");

    // Mutability
    let mut x: i8 = 10;
    x = 20;
    println!("Value is x is {x}");

    // Scope
    {
        let b: i8 = 20;
        println!("Value of b inside scope is {b}");
    }
    // println! {"Value of b outside scope is {b}"} // This should fail and as it cannot print.

    // Shadowing
    let c: i8 = 25;
    let c: i16 = 256;
    println!("Value of c after shadowing is {c}");

    // Constants
    const MAX_VALUE: u32 = 45678;
    println!("Max value of constant assigned is {MAX_VALUE}")
}
