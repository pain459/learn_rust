// -------------------------------------------------
// 			Bonus
// 			- Comments
// 			- More on Printing
//			- Inputs
//			- Variable Convention
//			- Statics
// -------------------------------------------------

fn main() {
    // First line in the comment
    // Second line in the comment
    /*
    This is a
    multiline comment
    */
    print!("This is the first print command.");
    print!("This is going to be printed on the same line");
    /* Escape sequences
    \n : newline character
    \t : Tab Space
    \r : Carriage Return
    \" : Double quote
    \\ : Backward Slash
    */
    println!("\n Will be printed after one empty line.");
    println!("\t A tab at the start.");
    println!("This will be overwritten \r Instead this will be printed.");
    println!("This prints quotes \" and this prints slash \\");

    println!(
        "I'm doing {2} from last {1} years and I {0} it!",
        "like", 20, "programming"
    );
    println!(
        "{language} is a system programming language which is cool to {activity} in.",
        language = "Rust", activity = "code"
    )
}
