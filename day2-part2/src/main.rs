use std::fs;
use nom::{
    IResult,
    bytes::complete::tag,
    branch::alt,
    character::complete::one_of,
    combinator::{map, recognize},
    multi::{separated_list0, many1}
};

enum Colour {
    Red,
    Green,
    Blue    
}

fn parse_decimal(input: &str) -> IResult<&str, u32> {
    let (input, dec_str) = recognize(
        many1(one_of("0123456789"))
    )(input)?;
    Ok((input, dec_str.parse().unwrap()))
}

fn parse_game_id(input: &str) -> IResult<&str, u32> {
    let (input, _) = tag("Game ")(input)?;
    let (input, id) = parse_decimal(input)?;
    let (input, _) = tag(": ")(input)?;
    Ok((input, id))
}

fn parse_round(input: &str) -> IResult<&str, (u32, u32, u32)> {
    let (input, cubes) = separated_list0(tag(", "), parse_num_cubes)(input)?;
    let mut reds = 0;
    let mut greens = 0;
    let mut blues = 0;
    for (colour, num) in cubes {
        match colour {
            Colour::Red => reds += num,
            Colour::Green => greens += num,
            Colour::Blue => blues += num
        };
    }
    Ok((input, (reds, greens, blues)))
}

fn parse_colour(input: &str) -> IResult<&str, Colour> {
    alt((
        map(tag(" red"), |_| Colour::Red),
        map(tag(" green"), |_| Colour::Green),
        map(tag(" blue"), |_| Colour::Blue),
    ))(input)
}

fn parse_num_cubes(input: &str) -> IResult<&str, (Colour, u32)> {
    let (input, num) = parse_decimal(input)?;
    let (input, colour) = parse_colour(input)?;
    Ok((input, (colour, num)))
}

fn parse_game(line: &str) -> u32 {
    let (input, id) = parse_game_id(line).unwrap();
    let (_, rounds) = separated_list0(tag("; "), parse_round)(input).unwrap();
    for (r, g, b) in rounds {
        if r > 12 || g > 13 || b > 14{
            return 0
        }
    }
    id
}

fn main() {
    let contents = fs::read_to_string("input/input.txt").unwrap();
    let contents = contents.trim();
    let mut sum_of_ids = 0;

    for line in contents.lines() {
        sum_of_ids += parse_game(line);
    }

    println!("Answer = {}", sum_of_ids);
}
