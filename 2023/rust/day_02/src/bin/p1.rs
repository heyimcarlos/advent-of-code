use std::{collections::BTreeMap, error::Error};

use day_02::util;
use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, digit1, line_ending},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

fn main() {
    let input_exists = std::path::Path::new("input.txt").exists();
    if !input_exists {
        match util::fetch("https://adventofcode.com/2023/day/2/input") {
            Ok(input) => std::fs::write("input.txt", input).unwrap(),
            Err(e) => println!("Error: {}", e),
        }
    }
    let input = std::fs::read_to_string("input.txt").expect("Should be a string");
    println!("{}", p1(&input).unwrap());
}

#[derive(Debug)]
struct Cube {
    color: String,
    amount: u32,
}

#[derive(Debug)]
struct Game {
    id: String,
    rounds: Vec<Vec<Cube>>, // -> A game is a collection of rounds, each round is a collection of cubes
}

impl Game {
    fn valid_for_cube_set(&self, map: BTreeMap<&str, u32>) -> Option<u32> {
        self.rounds
            .iter()
            .all(|round| {
                round.iter().all(|shown_cube| {
                    shown_cube.amount
                        <= *map
                            .get(shown_cube.color.as_str())
                            .expect("should be a valid cube")
                })
            })
            .then_some(self.id.parse::<u32>().expect("should be a parseable u32"))
    }
}

// 4 red
fn cube(input: &str) -> IResult<&str, Cube> {
    let (input, (amount, color)) = separated_pair(complete::u32, tag(" "), alpha1)(input)?;
    Ok((
        input,
        Cube {
            amount,
            color: color.to_string(),
        },
    ))
}

// 3 blue, 4 red, 1 green
fn round(input: &str) -> IResult<&str, Vec<Cube>> {
    let (input, cubes) = separated_list1(tag(", "), cube)(input)?;
    Ok((input, cubes))
}

// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
fn game(input: &str) -> IResult<&str, Game> {
    let (input, id) = preceded(tag("Game "), digit1)(input)?;
    let (input, rounds) = preceded(tag(": "), separated_list1(tag("; "), round))(input)?;
    Ok((
        input,
        Game {
            id: id.to_string(),
            rounds,
        },
    ))
}

fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, games) = separated_list1(line_ending, game)(input)?;
    Ok((input, games))
}

fn p1(input: &str) -> Result<String, Box<dyn Error>> {
    let map = BTreeMap::from([("red", 12), ("green", 13), ("blue", 14)]); // BTreeMap has a deterministic iteration order
    let games = parse_games(input).expect("should parse");
    let it = games
        .1
        .iter()
        .filter_map(|game| game.valid_for_cube_set(map.clone()))
        .sum::<u32>()
        .to_string();
    Ok(it)
}

// the input's lifetime is the duration of the p1 function.
fn _p1(input: String) -> i32 {
    let mut sum = 0;
    for (idx, line) in input.lines().enumerate() {
        let game_number = idx as i32 + 1;
        let mut is_valid = true;
        line.split(": ")
            .last()
            .unwrap()
            .split("; ")
            .collect::<Vec<_>>()
            .iter()
            .for_each(|part| {
                let game_round: Vec<&str> = part.split(", ").collect();
                println!("game_round: {:?}", game_round);
                game_round.iter().for_each(|p| {
                    let num = p
                        .split_whitespace()
                        .next()
                        .unwrap()
                        .to_string()
                        .parse::<i32>()
                        .unwrap();

                    let color = p.split_whitespace().last().unwrap().to_string();

                    if color == "red" && num > 12
                        || color == "green" && num > 13
                        || color == "blue" && num > 14
                    {
                        is_valid = false;
                    }
                });
            });
        if is_valid {
            sum += game_number;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() -> Result<(), Box<dyn Error>> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("8", p1(input)?);
        Ok(())
    }
}
