// -------------------------------------------
// 	Ownership Basics
// -------------------------------------------

/*
1.	Each value has a variable that's its "owner."
2.	A value can have only one owner at a time.
3.	If the owner goes out of scope, the value is cleaned up.
*/

fn main() {
    let s1: String = String::from("World!");
    let s2 = s1.clone();  // use s1.clone() instead to avoid error.
    println!("s1 is: {s1}");

    // testing ownership in scopes
    let x: i32 = 123;
    {
        let y = x;
    }
    // println!("Y is: {y}");  // This statement should give scope error.

    let i: i8 = 12;
    let j: i8 = i;
    // This works due to primitive value types.
    println!("Value of i is {i}");
    println!("value of j is {j}");
}