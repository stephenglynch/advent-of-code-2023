use std::fs;
use std::str;
use regex::Regex;
use lazy_static::lazy_static;

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?x)
        Time:\s+(\d*)\s*(\d*)\s*(\d*)\s*(\d*)\n
        Distance:\s+(\d*)\s*(\d*)\s*(\d*)\s*(\d*)").unwrap();
    }

    let cap = RE.captures(input).unwrap();
    
    let mut times = vec![];
    for i in 1..5 {
        if let Some(time_match) = cap.get(i) {
            times.push(time_match.as_str().parse().unwrap())
        }
    }

    let mut dists = vec![];
    for i in 1..5 {
        if let Some(dist_match) = cap.get(i + 4) {
            dists.push(dist_match.as_str().parse().unwrap())
        }
    }

    times.into_iter().zip(dists).collect()
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

    let races = parse_input(contents);

    let mut answer = 1;
    for (time, dist) in races {
        let (lower, upper) = calc_lower_upper(dist, time);
        answer *= upper - lower + 1;
    }

    println!("answer = {}", answer);
}
