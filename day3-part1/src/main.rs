use std::fs;
use std::str;
use regex::bytes::Regex;
use lazy_static::lazy_static;

struct Schematic {
    grid: Vec<Vec<u8>>,
    x_max: usize,
    y_max: usize
}

struct PartNumber {
    num: u32,
    x_left: usize,
    x_right: usize,
    y_left: usize
}

impl Schematic {
    // Just converting to Vec of Strings for now
    pub fn new(text: &str) -> Schematic {
        let grid: Vec<Vec<u8>> = text.lines()
            .map(|l| l.bytes().collect())
            .collect();
        let x_max = grid[0].len();
        let y_max = grid.len();

        Schematic {
            grid: grid,
            x_max: x_max,
            y_max: y_max
        }
    }

    pub fn find_part_numbers(&self) -> Vec<PartNumber> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\d+").unwrap();
        }

        let mut part_numbers = vec![];

        for (y, line) in self.grid.iter().enumerate() {
            for cap in RE.captures_iter(&line) {
                let mat = cap.get(0).unwrap();
                let num = str::from_utf8(mat.as_bytes()).unwrap().parse().unwrap();
                let x_left = mat.start();
                let x_right = mat.end();
                part_numbers.push(
                    PartNumber { num: num, x_left: x_left, x_right: x_right, y_left: y}
                );
            }
        }

        part_numbers
    }


    pub fn has_part(&self, part_num: &PartNumber) -> bool {
        let grid = &self.grid;
        let x_start = part_num.x_left.saturating_sub(1);
        let x_end = (part_num.x_right + 1).clamp(0, self.x_max-1);
        let y_start = part_num.y_left.saturating_sub(1);
        let y_end = (part_num.y_left + 2).clamp(0, self.y_max-1);

        for y in y_start..y_end {
            for x in x_start..x_end {
                if !b"0123456789.".contains(&grid[y][x]) {
                    return true;
                }
            }
        }

        println!("num={}", part_num.num);
        false
    }
    
}

fn main() {
    let contents = fs::read_to_string("input/input.txt").unwrap();
    let contents = contents.trim();

    let sch = Schematic::new(contents);
    let mut sum_of_parts = 0;
    for part_num in sch.find_part_numbers() {
        if sch.has_part(&part_num) {
            sum_of_parts += part_num.num;
        }
    }    

    println!("Answer = {}", sum_of_parts);
}
