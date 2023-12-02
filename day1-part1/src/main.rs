use std::fs;

fn main() {
    let contents = fs::read_to_string("input/input.txt").unwrap();
    let contents = contents.trim();

    let mut sum_of_values = 0;

    for line in contents.lines() {
        let mut nums = vec![];
        for c in line.chars() {
            if c.is_numeric() {
                nums.push(c);
            }
        }
        let combination = vec![nums[0], *nums.last().unwrap()];
        let value: u32 = combination.into_iter().collect::<String>().parse().unwrap();
        sum_of_values += value;
    }

    println!("Answer = {}", sum_of_values);
}
