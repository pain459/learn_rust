extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);


    println!("Please input your guess...");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line.");
    
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        ordering::Less => println!("Too less!"),
        ordering::Greater => println!("Too greater!"),
        ordering::Equal => println!("Congratulations, you won!"),
    }
}