// Problem 4:

/*
A palindrome is a word, verse, or sentence that reads the same backward or forward,
such as 'Able was I ere I saw Elba,' or a number like 1881.

Write a function named is_palindrome() that checks whether a given string is a palindrome or not.
The function should take a string as input and return a boolean value indicating whether the string is a palindrome or not.
*/

fn main() {
    let _input = String::from("malayalam");
    println!("Checking if {_input} is a palindrome!");
    println!("It is {:?} that the given string is a palindrome.",
    palindrome(_input));
}

fn palindrome(_input: String) -> bool {
    let mut chars = _input.chars();
    while let (Some(c1), Some(c2)) = (chars.next(), chars.next_back()) {
        if c1 != c2 {
            return false;
        }
    }
    true
}