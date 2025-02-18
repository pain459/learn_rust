// -------------------------------------------
// 			Conditionals
// 			- If else
// 			- If else if ladder
// 			- Match
// -------------------------------------------

fn main() {
    println!("We will discuss conditionals in this section.");
    let num: i8 = 42;
    if num > 50 {
        println!("The number is greater than 50");
    } else {
        println!("The number is less than 50");
    }

    // if else ladder
    let marks = 95;
    let grade = if marks >= 90 {
        'A'
    } else if marks >= 80 {
        'B'
    } else if marks >= 70 {
        'C'
    } else {
        'D'
    };

    println!("For marks {marks}, grade is {grade}");

    // conditional chaining using pattern match
    let marks = 75;
    let grade = match marks {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        _ => 'F'
    };

    println!("For marks {marks}, grade is {grade}");
}