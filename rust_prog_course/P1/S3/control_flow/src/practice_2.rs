// Problem 2:

/*
Write a program to find the sum of natural numbers below a given number N, where N is provided by the user.
The sum should only include numbers that are multiples of either 3 or 5.
For example, if the user enters N = 20, the multiples of 3 are 3, 6, 9, 12, 15, 18, and the multiples of 5 are 5, 10, and 15.

Please note that the value of 15 will be considered only once since it is a multiple of both 3 and 5.
    The sum will be calculated as follows: 3 + 5 + 6 + 9 + 10 + 12 + 15 + 18.

Write a program that takes the user input N, performs the necessary calculations, and outputs the sum.
*/

fn main() {
    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .expect("failed to read input");
    let n :i32 = n.trim().parse().expect("invalid input");

    println!("Entered number is {n}");

    let mut sum_result :i32 = 0;
    for i in 1..=n-1 {
        if multiple_by_2_or_3(i) == 0 {
            sum_result = sum_result + i;
        }
    }

    println!("Result of sum of numbers {sum_result}");
}

fn multiple_by_2_or_3(n :i32) -> i32 {
    let result = if n%3==0 || n%5 ==0 {
        0
    } else {
        -1
    };
    result
}