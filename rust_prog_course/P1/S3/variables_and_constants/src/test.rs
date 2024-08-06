fn main() {
    let marks = 98;
    let grade = match marks {
        90..=100 => 'A',
        80..=89 => "B".parse().unwrap(),
        70..=79 => "C".parse().unwrap(),
        _ => 'F',
    };
}