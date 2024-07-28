// Problem 1:
/*
Write a program to find the difference between the square of the sum and the sum of the squares of the first N numbers.
N will be a user-defined input that your program will take.

For example, if the user enters the number 5, you should compute the square of the sum as (1 + 2 + 3 + 4 + 5)^2 = 225.
Next, compute the sum of the squares as (1^2 + 2^2 + 3^2 + 4^2 + 5^2) = (1 + 4 + 9 + 16 + 25) = 55.
Finally, calculate the difference as 225 - 55 = 170.
*/
use std::collections::binary_heap::Iter;

fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).expect("failed to read input");
    let n: i32 = n.trim().parse().expect("invalid input");
    let mut square_of_sum = 0;
    let mut sum_of_squares = 0;

    // Square of the sum
    for i in 1..=n {
        square_of_sum = square_of_sum + i
    }
    // println!("after iterations {square_of_sum}");
    let result_square_of_sum = i32::pow(square_of_sum, 2);
    println!("Result for square of the sum is {result_square_of_sum}");

    println!("Test program!")
}