use std::collections::HashMap;
use anyhow::{anyhow, Result};
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1, line_ending, multispace1, newline};
use nom::combinator::{into, map, map_res};
use nom::IResult;
use nom::multi::{many0, separated_list0, separated_list1};
use nom::sequence::{pair, preceded};

#[derive(Eq, PartialEq, Debug)]
struct Game {
    id: u32,
    played_cubes: HashMap<Color, u32>,
    rounds: Vec<Round>,
    // played_cubes: Vec<Round>
}

fn game_id(input: &str) -> IResult<&str, u32> {
    map_res(
        preceded(tag("Game "), digit1),
        |id: &str| { id.parse::<u32>()}
    )(input)
}

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
enum Color {
    Red,
    Green,
    Blue
}

impl From<&str> for Color {
    fn from(value: &str) -> Self {
        match value {
            "red" => Color::Red,
            "green" => Color::Green,
            "blue" => Color::Blue,
            _ => panic!("Cannot format as color!")
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Round {
    played_cubes: Vec<(u32, Color)>
}

// Expecting input like: "3 blue, 4 red"
//                   or: "8 green, 6 blue, 20 red"
fn round(input: &str) -> IResult<&str, Round> {
    let (remaining , played_cubes) =
        separated_list1(tag(", "),
                        pair(map_res(digit1, |count: &str| {count.parse()}),
                             into(preceded(tag(" "), alpha1::<&str, nom::error::Error<&str>>))
                        )
        )(input)?;
    Ok((remaining, Round {
        played_cubes
    }))
}

fn parse_game(input: &str) -> IResult<&str, Game> {

    let (remaining, id) = game_id(input)?;
    let (remaining, _) = tag(": ")(remaining)?;
    
    map(separated_list1(tag("; "), round), move |rounds| {
        let mut played_cubes: HashMap<Color, u32> = HashMap::new();
        let extra_rounds = rounds.clone();
        for round in rounds {
            for played_cube in round.played_cubes {
                let mut cube_count = *played_cubes.get(&played_cube.1).unwrap_or(&0);

                let new_cube_count = cube_count + played_cube.0;

                played_cubes.insert(played_cube.1, new_cube_count);
            }
        }
        Game {
            id, 
            played_cubes,
            rounds: extra_rounds,
        }
    })(remaining)
}

fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    separated_list0(line_ending, parse_game)(input)
}

pub fn process(input: &str) -> Result<i32> {
    let games;
    match parse_games(input) {
        Ok((_, parsed_games)) => games = parsed_games,
        Err(_) => return Err(anyhow!("failed to parse games")),
    }
    let mut sum = 0;
    for game in games {
        let mut is_game_valid = true;
        for round in game.rounds {
            let mut red_cube_count = 0;
            let mut green_cube_count = 0;
            let mut blue_cube_count = 0;
            for played_cube in &round.played_cubes {
                match played_cube.1 {
                    Color::Red => {
                        red_cube_count += played_cube.0
                    }
                    Color::Green => {
                        green_cube_count += played_cube.0
                    }
                    Color::Blue => {
                        blue_cube_count += played_cube.0
                    }
                };
            }
            let dbg_round = round.clone();
            if !((red_cube_count <= 12) && (green_cube_count <= 13) && (blue_cube_count <= 14)) {
                is_game_valid = false;
                break;
            }
        }
        if is_game_valid {
            sum += game.id;
        }
        
    }
    Ok(sum as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_complete() -> Result<()> {
        let input = include_str!("../input1.txt");
        assert_eq!(2085, process(input)?);
        Ok(())
    }
    
    #[test]
    fn test_process() -> Result<()> {
        let input = include_str!("../input2.txt");
        assert_eq!(8, process(input)?);
        Ok(())
    }
}
