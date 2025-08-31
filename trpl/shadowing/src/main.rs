fn main() {
    let x = 5;

    let x = x + 1;

    println!("x is now {x}");

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x outside the scope is {x}")
}