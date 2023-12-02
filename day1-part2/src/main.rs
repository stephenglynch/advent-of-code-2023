use std::fs;
use regex::Regex;
use lazy_static::lazy_static;

fn convert_value_str(vals: &str) -> u32 {
    match vals {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => vals.parse().unwrap()
    }
}

fn find_number(line: &str, offset: usize) -> Option<(u32, usize)> {
    lazy_static! {
        static ref PATTERN: Regex = Regex::new(
            r"([0-9]|one|two|three|four|five|six|seven|eight|nine)"
        ).unwrap();
    }

    let m = PATTERN.captures_at(line, offset)?.get(1).unwrap();

    let num = convert_value_str(m.as_str());
    // Aparrently overlapping numbers count i.e. "twone" counts as 2, 1 - eugh!
    let index = m.start() + 1;

    Some((num, index))
}

fn find_all_numbers(line: &str) -> Vec<u32> {
    let mut index = 0;
    let mut numbers = vec![];
    while let Some((num, new_index)) = find_number(line, index) {
        numbers.push(num);
        index = new_index;
    }
    numbers
}

fn main() {
    let contents = fs::read_to_string("input/input.txt").unwrap();
    let contents = contents.trim();
    let mut sum_of_values = 0;

    for line in contents.lines() {
        let nums = find_all_numbers(line);
        let first = *nums.first().unwrap();
        let last = *nums.last().unwrap();
        sum_of_values += first * 10 + last;
    }

    println!("Answer = {}", sum_of_values);
}
