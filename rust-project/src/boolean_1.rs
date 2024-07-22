fn main() {

    let _is_male: bool = true;
    let _is_over_18: bool = false;

    if _is_male {
        print!("You are male!\n")
    } else {
        print!("You care not male!\n")
    }

    if _is_male && !_is_over_18 {
        print!("You are a legal male!\n")
    }
}