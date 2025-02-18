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
    println!("{:?}", _array_3);

    // Vectors
    let vec_1 :Vec<i32> = vec![4, 5, 6, 7, 8, 9];
    let num = vec_1[3];
    println!("{:?}", num);

    // Tuples
    let my_tuple = ("Salary", 40000, "Age", 30);
    let my_salary = my_tuple.1;
    println!("{:?}", my_salary);

    let(salary, salary_value, age, age_value) = my_tuple;

    let unit: () = ();


    let x: u8;
    x = 1;
    println!("x is {}", x);

    let pi: f32;
    pi = 3.14159;
    println!("Value of pi is {}", pi);

    let a: i16 = -15;
    let b: i16 = 170;
    let name: &str = "Michael";
    println!("name is {}, and the multiplication result is {}", name, a*b);

     type Book = (String, String, u32);

    let book1:Book = (
        String::from("Rust Programming Language"),
        String::from("Rust Community"),
        2010,
    );
    println!(
        "Book Name: {}, Author: {}, year: {}",
        book1.0, book1.1, book1.2
    );

    let book2:Book = (
        String::from("Programming in Python"),
        String::from("Mark Lutz"),
        2012,
    );
    println!(
        "Book Name: {}, Author: {}, year: {}",
        book2.0, book2.1, book2.2
    );
}
