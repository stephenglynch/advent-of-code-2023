use std::fs;
use std::str;
use regex::Regex;
use lazy_static::lazy_static;

struct PartNumber {
    num: u32,
    x: usize,
    y: usize,
    len: usize
}

struct Gear {
    x: usize,
    y: usize
}

struct Schematic {
    x_max: usize,
    y_max: usize,
    gears: Vec<Gear>,
    part_numbers: Vec<PartNumber>
}

impl Schematic {
    pub fn new(text: &str) -> Schematic {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\d+").unwrap();
        }

        let mut gears = vec![];
        let mut part_numbers = vec![];
        for (y, line) in text.lines().enumerate() {

            // Add part numbers
            for cap in RE.captures_iter(line) {
                let mat = cap.get(0).unwrap();
                let x = mat.start();
                let len = mat.len();
                let num = mat.as_str().parse().unwrap();
                part_numbers.push(
                    PartNumber { num: num, x: x, y: y, len: len }
                );
            }

            // Add gears
            for (x, c) in line.chars().enumerate() {
                if c == '*' {
                    gears.push(Gear {x: x, y: y});
                }
            }
        }

        // Get limits
        let x_max = text.lines().count();
        let y_max = text.lines().next().unwrap().len();

        Schematic { gears: gears, part_numbers: part_numbers, x_max: x_max, y_max: y_max}
    }

    fn adjacent(&self, gear: &Gear, part: &PartNumber) -> bool {

        let x_start = part.x.saturating_sub(1);
        let x_end = (part.x + part.len + 1).clamp(0, self.x_max);
        let y_start = part.y.saturating_sub(1);
        let y_end = (part.y + 2).clamp(0, self.y_max);
    
        for y in y_start..y_end {
            for x in x_start..x_end {
                if x == gear.x && y == gear.y {
                    return true
                }
            }
        }
        false
    }

    fn gear_ratio(&self, gear: &Gear) -> u32 {
        let mut part_numbers = vec![];
        for part in self.part_numbers.iter() {
            if self.adjacent(gear, part) {
                part_numbers.push(part);
            }
        }
        println!("{}", part_numbers.len());
        if part_numbers.len() == 2 {
            part_numbers[0].num * part_numbers[1].num
        } else {
            0
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input/input.txt").unwrap();
    let contents = contents.trim();

    let sch = Schematic::new(contents);
    let mut sum_of_gears = 0;
    for gear in sch.gears.iter() {
        sum_of_gears += sch.gear_ratio(gear);
    }    

    println!("Answer = {}", sum_of_gears);
}
