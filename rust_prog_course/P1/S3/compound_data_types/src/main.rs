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

    // Arrays
    let mut _array_1 = [4, 5, 6, 7, 8, 9];
    let _num = _array_1[3];
    let _array_2 = [0; 10];
    let _array_3 = [1; 10];

    println!("{:?}", _num);
    println!("{:?}", _array_2);
    println!("{:?}", _array_3);
}
