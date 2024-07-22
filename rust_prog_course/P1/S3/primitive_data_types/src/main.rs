// -------------------------------------------------
// 			- Scalar Data Types
//	 			- Integers
// 				- Floats
// 				- Chars
// 				- Boolean
// -------------------------------------------------

fn main() {
    // Signed integers
    let i: i8 = 100;
    println!("{i}");

    // Unsigned integers
    let j: u8 = 10;
    println!("{j}");

    // Floats
    let k: f32 = -10.0;
    println!("{k}");

    // Chars
    let char1 = "character";
    let char2 = "character2";
    println!("{char1}");
    println!("{char2}");

    // Boolean
    let m: bool = false;
    println!("{m}");

    // Type aliasing
    type Age = u8;
    let person_age: Age = 18;
    println!("{person_age}");

    // Type Conversions
    let a = 10;
    let b = a as f32;
    println!("{a}");
    println!("{b}");
}
