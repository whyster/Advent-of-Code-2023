use std::{any, ascii::AsciiExt, ops::Index};

use anyhow::Result;
use itertools::Itertools;
use nom::Slice;

#[derive(Debug)]
struct NumberLocation {
    x: usize,
    y: usize,
    len: usize,
}

#[derive(Debug)]
struct Point(usize, usize);

impl NumberLocation {
    pub fn get_neighborhood(&self) -> Vec<Point> {
        let upper_left_x = self.x.checked_sub(1).unwrap_or(self.x);
        let upper_left_y = self.y.checked_sub(1).unwrap_or(self.y);
        let bottom_right_x = self.x + self.len;
        let bottom_right_y = self.y + 1;
        let mut neighborhood = Vec::new();
        for x in upper_left_x..=bottom_right_x {
            for y in upper_left_y..=bottom_right_y {
                if (y == self.y && x >= self.x && x < bottom_right_x) {
                    continue;
                }
                neighborhood.push(Point(x, y));
            }
        }

        neighborhood
    }
}

#[derive(Debug)]
struct Schematic<'a> {
    input: &'a [&'a str],
    number_indices: Vec<NumberLocation>,
}

impl<'a> Schematic<'a> {
    pub fn new(input: &'a [&'a str]) -> Self {
        let mut schematic = Self {
            input,
            number_indices: Vec::new(),
        };
        schematic.populate_number_indices();
        schematic
    }

    pub fn populate_number_indices(&mut self) {
        for (y, &line) in self.input.iter().enumerate() {
            let mut matching_number = false;
            let mut num_length = 0;
            let mut start_x = 0;
            for (x, character) in line.chars().enumerate() {
                if character.is_digit(10) {
                    if !matching_number {
                        start_x = x;
                    }
                    matching_number = true;
                    num_length += 1;
                } else if matching_number {
                    // Stop matching the number
                    matching_number = false;
                    // Record number
                    self.number_indices.push(NumberLocation {
                        x: start_x,
                        y,
                        len: num_length,
                    });
                    start_x = 0;
                    num_length = 0;
                }
            }
            if matching_number {
                // Stop matching the number
                matching_number = false;
                // Record number
                self.number_indices.push(NumberLocation {
                    x: start_x,
                    y,
                    len: num_length,
                });
                start_x = 0;
                num_length = 0;
            }
        }
    }

    fn is_part_number(&self, location: &NumberLocation) -> bool {
        for point in location.get_neighborhood() {
            let x = point.0;
            let y = point.1;
            if y >= self.input.len() {
                continue;
            }
            let row = self.input[y];
            if x >= row.len() {
                continue;
            }
            if let Some(character) = row.chars().nth(x) {
                if character != '.' {
                    return true;
                }
            }
        }
        false
    }

    pub fn get_part_numbers(&self) -> Vec<usize> {
        self.number_indices
            .iter()
            .filter(|&num| self.is_part_number(num))
            .filter_map(|location| {
                self.input[location.y][location.x..(location.x + location.len)]
                    .parse::<usize>()
                    .ok()
            })
            .collect_vec()
    }
}

pub fn process(input: &str) -> Result<i32> {
    /* let mut test = Schematic {
        input: &input.lines().collect::<Vec<&str>>(),
        number_indices: Vec::new(),
    };
    test.populate_number_indices(); */
    let lines = input.lines().collect_vec();
    let schematic = Schematic::new(&lines);
    Ok(schematic.get_part_numbers().iter().sum::<usize>() as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_part_number() {
        let input = include_str!("../input2.txt");
        let lines = input.lines().collect_vec();
        let schematic = Schematic::new(&lines);

        assert!(schematic.is_part_number(&NumberLocation { x: 0, y: 0, len: 3 }));
        assert_eq!(
            false,
            schematic.is_part_number(&NumberLocation { x: 5, y: 0, len: 3 })
        );
    }

    #[test]
    fn test_process() -> Result<()> {
        let input = include_str!("../input2.txt");
        assert_eq!(4361, process(input)?);
        Ok(())
    }
}
