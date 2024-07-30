// Problem 5:
/*
A Pythagorean triple consists of three positive integers a, b, and c, satisfying the condition a^2 + b^2 = c^2.
These triples are commonly written as (a, b, c), and a well-known example is (3, 4, 5).

Write a program that computes the Pythagorean triplet such that a < b < c and a + b + c = 1000.
*/
use std::fs::read_to_string;
use std::io::{self, Write};
fn main() {
    println!("Pythagorean triplet validate and calculate.");
    // Function to read and validate integer input.
    fn read_integer(prompt: &str) -> i32 {
        loop {
            // Create a mutable string to store the user input
            let mut user_input = String::new();
            // prompt user for input
            println!("{}", prompt);
            // Flush the output bugger to ensure the prompt is shown before input is taken
            io::stdout().flush().unwrap();

            // Read the input from the user and store it in the user_input variable
            io::stdin()
                .read_line(&mut user_input)
                .expect("Failed to read line.");
            // Remove the newline character from the input
            let user_input = user_input.trim();
            // Try to parse the input as an integer
            match user_input.parse::<i32>() {
                Ok(number) => return number,
                Err(_) => println!("The input is not a valid integer. Please try again."),
            }
        }
    }

    // collect inputs for variables a, b, and c
    let a = read_integer("Please enter integer for variable a: ");
    let b = read_integer("Please enter integer for variable b: ");
    let c = read_integer("Please enter integer for variable c: ");

    println!("Entered variables are a = {a}, b = {b}, c = {c}");
}

fn is_valid_triplet(a: i32, b: i32, c: i32) -> bool {
    false
}

fn calculate_triplet() {

}