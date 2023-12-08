use std::fs;
use std::str;
use regex::Regex;
use lazy_static::lazy_static;

fn parse_scratch_card(line: &str, num_winners: usize, num_draw: usize) -> (Vec<u32>, Vec<u32>) {
    lazy_static! {
        // Real input regex
        static ref RE: Regex = Regex::new(r"(?x)
            Card\s+(\d+):
            \s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)
            \s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)
            \s+\|
            \s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)
            \s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)
            \s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)
            \s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)
            \s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)"
        ).unwrap();
    }
    let mut winners = vec![];
    let mut draw = vec![];

    let cap = RE.captures(line).unwrap();
    let mut group_i = 2;
    for _ in 0..num_winners {
        winners.push(cap.get(group_i).unwrap().as_str().parse().unwrap());
        group_i += 1;
    }
    for _ in 0..num_draw {
        draw.push(cap.get(group_i).unwrap().as_str().parse().unwrap());
        group_i += 1;
    }

    (winners, draw)
}

fn score_draw(winners: &Vec<u32>, draw: &Vec<u32>) -> u32 {
    let mut total_matches = 0;
    for w in winners.iter() {
        if draw.contains(w) {
            total_matches += 1;
        }
    }

    if total_matches > 0 {
        2u32.pow(total_matches - 1)
    } else {
        0
    }
}

fn main() {
    let contents = fs::read_to_string("input/input.txt").unwrap();
    let contents = contents.trim();

    // Number of winning numbers and draw numbers for real input
    let num_winners = 10;
    let num_draw = 25;

    let mut mutscore_total = 0;
    for line in contents.lines() {
        let (winners, draw) = parse_scratch_card(line, num_winners, num_draw);
        mutscore_total += score_draw(&winners, &draw);
    }

    println!("Answer = {}", mutscore_total);
}
