use std::fs;
use std::str;
use regex::Regex;
use lazy_static::lazy_static;

type LensBox = Vec<(String, usize)>;

fn calc_hash(input: &str) -> u8 {
    input.bytes().fold(0, |acc, c| {
        acc.wrapping_add(c).wrapping_mul(17)
    })
}

fn run_step(boxes: &mut [LensBox], step: &str) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([a-z]+)(=|-)(\d+)?").unwrap();
    }

    let cap = RE.captures(step).unwrap();
    let label = cap.get(1).unwrap().as_str();
    let box_i = calc_hash(label) as usize;
    let op = cap.get(2).unwrap().as_str();
    let lensbox = &mut boxes[box_i];

    if op == "=" {
        let focal = cap.get(3).unwrap().as_str().parse().unwrap();
        if let Some(i) = lensbox.iter().position(|(l,_)| l == label) {
            lensbox[i] = (label.to_string(), focal);
        } else {
            lensbox.push((label.to_string(), focal));
        }
    } else {
        if let Some(i) = lensbox.iter().position(|(l,_)| l == label) {
            lensbox.remove(i);
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input/input.txt").unwrap();
    let contents = contents.trim();
    const EMPTY: LensBox = vec![];
    let mut boxes = [EMPTY; 256];
    
    for s in contents.split(',') {
        run_step(&mut boxes, s)
    }

    let mut answer = 0;
    for (i, b) in boxes.iter().enumerate() {
        for (j, (_, f)) in b.iter().enumerate() {
            answer += (i+1) * (j+1) * f;
        }
    }

    println!("answer = {}", answer);
}
