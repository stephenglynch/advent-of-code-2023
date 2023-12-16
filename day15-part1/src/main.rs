use std::fs;
use std::str;

fn calc_hash(input: &str) -> u8 {
    input.bytes().fold(0, |acc, c| {
        acc.wrapping_add(c).wrapping_mul(17)
    })
}

fn main() {
    let contents = fs::read_to_string("input/input.txt").unwrap();
    let contents = contents.trim();
    let answer = contents.split(',').fold(0, |acc, s| {
            acc + calc_hash(&s) as u64
    });

    println!("answer = {}", answer);
}
