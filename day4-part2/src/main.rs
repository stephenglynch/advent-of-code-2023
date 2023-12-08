use std::fs;
use std::str;
use regex::Regex;
use lazy_static::lazy_static;

struct ScratchCard {
    copies: usize,
    matches: usize
}

fn parse_scratch_card(line: &str, num_winners: usize, num_draw: usize) -> ScratchCard {
    lazy_static! {
        // Example regex
        // static ref RE: Regex = Regex::new(r"(?x)
        //     Card\s+(\d+):\s+
        //     (\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+
        //     \|\s+ 
        //     (\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)"
        // ).unwrap();

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

    let matches = score_draw(&winners, &draw);
    
    ScratchCard { copies: 1, matches: matches }
}

fn score_draw(winners: &Vec<usize>, draw: &Vec<usize>) -> usize {
    let mut total_matches = 0;
    for w in winners.iter() {
        if draw.contains(w) {
            total_matches += 1;
        }
    }
    total_matches
}

fn process_cards(cards: &mut Vec<ScratchCard>) {
    for i in 0..cards.len() {
        let start_j = i + 1;
        let end_j = (start_j + cards[i].matches).clamp(0, cards.len());
        for j in start_j..end_j {
            cards[j].copies += cards[i].copies;
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input/input.txt").unwrap();
    let contents = contents.trim();

    // Number of winning numbers and draw numbers for example
    // let num_winners = 5;
    // let num_draw = 8;

    // Number of winning numbers and draw numbers for real input
    let num_winners = 10;
    let num_draw = 25;

    let mut scratch_cards = vec![];
    for line in contents.lines() {
        scratch_cards.push(parse_scratch_card(line, num_winners, num_draw));
    }

    process_cards(&mut scratch_cards);

    let total_cards = scratch_cards.iter().fold(0, |acc, c| acc + c.copies);
    println!("Answer = {}", total_cards);
}
