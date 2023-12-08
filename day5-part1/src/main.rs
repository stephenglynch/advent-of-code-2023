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
    pub fn apply(&self, val: i64) -> Option<i64> {
        if val >= self.source && val < self.source + self.len {
            let offset = self.destination - self.source;
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
        if let Some(mapped) = map_range.apply(val) {
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

fn main() {
    let contents = fs::read_to_string("input/input.txt").unwrap();
    let contents = contents.trim();

    let (seeds, maps) = parse_input(contents);

    let answer = seeds.iter().map(|seed| {
        apply_all_maps(&maps, *seed)
    }).min().unwrap();

    println!("answer = {}", answer);
}
