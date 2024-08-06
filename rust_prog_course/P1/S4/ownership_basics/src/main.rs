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
    let s2 = s1;  // use s1.clone() instead to avoid error.
    println!("s1 is: {s1}");
}