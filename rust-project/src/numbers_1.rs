fn main() {

    let mut a: i128 = 10;

    for _i in 1..10 {
        a = a + a * _i;        
    }

    print!("a is {}\n", a)
}