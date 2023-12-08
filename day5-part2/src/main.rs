use std::fs;
use std::str;
use regex::Regex;
use lazy_static::lazy_static;


type AgriMap = Vec<AgriMapRange>;

struct AgriMapRange {
    source: i64,
    destination: i64,
    len: i64
}

impl AgriMapRange {
    pub fn apply_rev(&self, val: i64) -> Option<i64> {
        if val >= self.destination && val < self.destination + self.len {
            let offset = self.source - self.destination;
            Some(val + offset)
        } else {
            None
        }
    }
}

fn parse_num_list(input: &str) -> Vec<i64> {
    input.split(" ").map(|s| s.parse().unwrap()).collect()
}

fn parse_argri_map_range(input: &str) -> AgriMapRange {
    let mut it = input.split(" ");
    AgriMapRange {
        destination: it.next().unwrap().parse().unwrap(),
        source: it.next().unwrap().parse().unwrap(),
        len: it.next().unwrap().parse().unwrap()
    }
}

fn parse_argri_map(input: &str) -> AgriMap {
    let mut map = vec![];
    for line in input.trim().lines() {
        map.push(parse_argri_map_range(line))
    }
    map
}

fn map_value(map: &AgriMap, val: i64) -> i64 {
    for map_range in map {
        if let Some(mapped) = map_range.apply_rev(val) {
            return mapped
        }
    }
    val
}

fn apply_all_maps(maps: &Vec<AgriMap>, val: i64) -> i64 {
    let mut current = val;
    for map in maps {
        current = map_value(map, current);
    }
    current
}

fn parse_input(input: &str) -> (Vec<i64>, Vec<AgriMap>) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?x)
        seeds:\s(.*)\n\n
        (?:[a-z-]+)\smap:\n([0-9\s]*)
        (?:[a-z-]+)\smap:\n([0-9\s]*)
        (?:[a-z-]+)\smap:\n([0-9\s]*)
        (?:[a-z-]+)\smap:\n([0-9\s]*)
        (?:[a-z-]+)\smap:\n([0-9\s]*)
        (?:[a-z-]+)\smap:\n([0-9\s]*)
        (?:[a-z-]+)\smap:\n([0-9\s]*)").unwrap();
    }

    let cap = RE.captures(input).unwrap();
    let seeds = parse_num_list(cap.get(1).unwrap().as_str().trim());

    let mut maps = Vec::with_capacity(7);
    for i in 2..9 {
        let group = cap.get(i).unwrap().as_str();
        maps.push(parse_argri_map(group));
    }
    (seeds, maps)
}

fn check_location(seed_ranges: &Vec<i64>, trial_seed: i64) -> bool {
    for sr in seed_ranges.chunks(2) {
        let start = sr[0];
        let len = sr[1];
        if trial_seed >= start && trial_seed < start + len {
            return true
        }
    }
    false
}

fn main() {
    let contents = fs::read_to_string("input/input.txt").unwrap();
    let contents = contents.trim();

    let (seeds, mut maps) = parse_input(contents);
    maps.reverse(); // Reverse order as we're going in reverse!

    let mut answer = 0;
    for trial_location in 0.. {
        let trial_seed = apply_all_maps(&maps, trial_location);
        if check_location(&seeds, trial_seed) {
            answer = trial_location;
            break;
        }
    }

    println!("answer = {}", answer);
}
