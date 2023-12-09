use std::fs;
use std::str;
use regex::Regex;
use lazy_static::lazy_static;

fn parse_input(input: &str) -> (usize, usize) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?x)
        Time:\s+(\d*)\s*(\d*)\s*(\d*)\s*(\d*)\n
        Distance:\s+(\d*)\s*(\d*)\s*(\d*)\s*(\d*)").unwrap();
    }

    let cap = RE.captures(input).unwrap();
    
    let mut time_s = String::new();
    for i in 1..5 {
        if let Some(time_match) = cap.get(i) {
            time_s.push_str(time_match.as_str())
        }
    }

    let mut dist_s = String::new();
    for i in 1..5 {
        if let Some(dist_match) = cap.get(i + 4) {
            dist_s.push_str(dist_match.as_str())
        }
    }

    (time_s.parse().unwrap(), dist_s.parse().unwrap())
}

fn calc_lower_upper(dist: usize, time: usize) -> (usize, usize) {
    let dist = dist as f64;
    let time = time as f64;
    let lower = time/2. - f64::sqrt(time.powi(2) / 4. - dist);
    let upper = time/2. + f64::sqrt(time.powi(2) / 4. - dist);
    return (lower.ceil() as usize, upper.floor() as usize)
}

fn main() {
    let contents = fs::read_to_string("input/input.txt").unwrap();
    let contents = contents.trim();

    let (time, dist) = parse_input(contents);

    let (lower, upper) = calc_lower_upper(dist, time);
    let answer = upper - lower + 1;

    println!("answer = {}", answer);
}
