use std::fs;

#[allow(dead_code)]

pub fn main() {
    let input = fs::read_to_string("inputs/test.txt").expect("Unable to open file");
    println!("{}", input);
}
