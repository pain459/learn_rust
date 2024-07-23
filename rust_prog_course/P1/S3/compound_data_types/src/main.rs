// -------------------------------------------------
// 			- Compound Data Types
//	 			- &str and String
// 				- Arrays
// 				- Vectors
// 				- Tuples
// 				- Empty Tuple
// -------------------------------------------------

fn main() {
    // &str and String
    let fixed_string: &str = "This is a fixed length string!";
    let mut variable_string = String::from("This string can modify in size.");
    variable_string.push_str("Test");

    println!("fixed length string is {fixed_string}");
    println!("variable length string is {variable_string}");
}