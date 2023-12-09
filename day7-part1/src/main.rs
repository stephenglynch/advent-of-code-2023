use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::str;
use regex::Regex;
use lazy_static::lazy_static;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind
}

#[derive(PartialEq, Eq, Ord, Clone)]
struct Hand {
    cards: String,
    hand_type: HandType,
    score: usize
}

impl Hand {
    fn new(cards: &str) -> Hand {
        let mut tally: HashMap<char, u8> = HashMap::new();
        for c in cards.chars() {
            tally.entry(c)
                .and_modify(|t| *t += 1)
                .or_insert(1);
        }

        let mut tally_vals: Vec<u8> = tally.values().copied().collect();
        tally_vals.sort();
        tally_vals.reverse();

        let hand_type = if tally_vals[0] == 5 {
            HandType::FiveOfAKind
        } else if tally_vals[0] == 4 {
            HandType::FourOfAKind
        } else if tally_vals[0] == 3 && tally_vals[1] == 2 {
            HandType::FullHouse
        } else if tally_vals[0] == 3 {
            HandType::ThreeOfAKind
        } else if tally_vals[0] == 2 && tally_vals[1] == 2 {
            HandType::TwoPair
        } else if tally_vals[0] == 2 {
            HandType::OnePair
        } else {
            HandType::HighCard
        };

        // Encode score for hand for second ordering rule 
        let mut score = 0;
        for i in 0..5 {
            let card_value = card_value(cards.chars().nth(i).unwrap());
            score += card_value; 
            score <<= 4;
        }

        Hand { cards: cards.to_string(), hand_type: hand_type, score: score }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => {
                self.score.partial_cmp(&other.score)
            }
            order => Some(order)
        }
    }
}

// Encode card value as number between 1-13
fn card_value(card: char) -> usize {
    match card {
        'A' => 13,
        'K' => 12,
        'Q' => 11,
        'J' => 10,
        'T' => 9,
        '9' => 8,
        '8' => 7,
        '7' => 6,
        '6' => 5,
        '5' => 4,
        '4' => 3,
        '3' => 2,
        '2' => 1,
        _ => panic!()
    }
}

fn parse_input(input: &str) -> Vec<(Hand, usize)> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([2-9TJQKA]{5})\s+(\d+)").unwrap();
    }

    input.lines().map(|line| {
        let cap = RE.captures(line).unwrap();
        let hand = Hand::new(cap.get(1).unwrap().as_str());
        let bid = cap.get(2).unwrap().as_str().parse().unwrap();
        (hand, bid)
    }).collect()
}

fn main() {
    let contents = fs::read_to_string("input/input.txt").unwrap();
    let contents = contents.trim();

    let mut hands = parse_input(contents);
    hands.sort_by_key(|h| h.0.clone());

    let mut answer = 0;
    for (i, (_, b)) in hands.into_iter().enumerate() {
        answer += (i + 1) * b;
    }

    println!("answer = {}", answer);
}
