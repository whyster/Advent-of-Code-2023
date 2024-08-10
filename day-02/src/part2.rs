use crate::part1::{parse_games, Color};

use anyhow::anyhow;
use anyhow::Result;

pub fn process(input: &str) -> Result<i32> {
    let games;
    match parse_games(input) {
        Ok((_, parsed_games)) => games = parsed_games,
        Err(_) => return Err(anyhow!("failed to parse games")),
    }
    let mut sum = 0;
    for game in games {
        let mut min_red_cube_count = 0;
        let mut min_green_cube_count = 0;
        let mut min_blue_cube_count = 0;
        for round in game.rounds {
            let mut red_cube_count = 0;
            let mut green_cube_count = 0;
            let mut blue_cube_count = 0;
            for played_cube in &round.played_cubes {
                match played_cube.1 {
                    Color::Red => red_cube_count += played_cube.0,
                    Color::Green => green_cube_count += played_cube.0,
                    Color::Blue => blue_cube_count += played_cube.0,
                };
            }
            min_red_cube_count = min_red_cube_count.max(red_cube_count);
            min_green_cube_count = min_green_cube_count.max(green_cube_count);
            min_blue_cube_count = min_blue_cube_count.max(blue_cube_count);
        }
        let power = min_red_cube_count * min_green_cube_count * min_blue_cube_count;
        sum += power;
    }
    Ok(sum as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = include_str!("../input2.txt");
        assert_eq!(2286, process(input)?);
        Ok(())
    }
}
