use std::io::Lines;

use anyhow::{anyhow, Result};
use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::character::complete::{alpha0, satisfy};
use nom::combinator::{map, peek, value};
use nom::multi::{many0, many_till};
use nom::sequence::{pair, preceded};
use nom::IResult;

fn word_digit(input: &str) -> IResult<&str, u32> {
    /*alt((
        value(1, tag("one")),
        value(2, tag("two")),
        value(3, tag("three")),
        value(4, tag("four")),
        value(5, tag("five")),
        value(6, tag("six")),
        value(7, tag("seven")),
        value(8, tag("eight")),
        value(9, tag("nine")),
    ))(input)*/
    alt((
        value(1, pair(tag("o"), peek(tag("ne")))),
        value(2, pair(tag("t"), peek(tag("wo")))),
        value(3, pair(tag("t"), peek(tag("hree")))),
        value(4, pair(tag("f"), peek(tag("our")))),
        value(5, pair(tag("f"), peek(tag("ive")))),
        value(6, pair(tag("s"), peek(tag("ix")))),
        value(7, pair(tag("s"), peek(tag("even")))),
        value(8, pair(tag("e"), peek(tag("ight")))),
        value(9, pair(tag("n"), peek(tag("ine")))),
    ))(input)
}

fn parse_digit(input: &str) -> IResult<&str, u32> {
    let (rest, digit) = alt((
        map(
            many_till(satisfy(|d| d.is_alphabetic()), word_digit),
            |(_, digit)| digit,
        ),
        preceded(
            alpha0,
            map(take(1_usize), |st: &str| st.parse::<u32>().expect("digit")),
        ),
    ))(input)?;

    return Ok((rest, digit));
}

fn parse_digits(input: &str) -> IResult<&str, Vec<u32>> {
    many0(parse_digit)(input)
}

pub fn process(input: &str) -> Result<i32> {
    let mut sum: u32 = 0;
    for line in input.lines() {
        let digits: Vec<u32>;
        match parse_digits(line) {
            Ok((_, digs)) => digits = digs,
            Err(_) => return Err(anyhow!("failed to parse digits")),
        }

        /*for digit in digits {
            if let None = first_digit {
                first_digit = Some(digit);
            }
            last_digit = digit;
        }*/
        let first_digit = digits.first().expect("digits should not empty");
        let last_digit = digits.last().expect("digits should not be empty");
        let concatted = (*first_digit * 10) + *last_digit;
        println!("{line}");
        println!("{first_digit} + {last_digit} = {concatted}");
        assert!(digits.len() >= 1);
        // assert!(digits.len() >= 2);
        /*for character in line.chars(){
            if let Some(digit) = character.to_digit(10){
                if first_digit == i32::MIN{
                    first_digit = digit as i32;
                }
                last_digit = digit as i32;
            }
        } */
        sum += (*first_digit * 10) + *last_digit;
    }

    Ok(sum as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() -> Result<()> {
        let (rest, dig) = parse_digit("one")?;
        assert_eq!(1, dig);
        let (rest, dig) = parse_digit("two")?;
        assert_eq!(2, dig);
        let (rest, dig) = parse_digit("ztwo")?;
        assert_eq!(2, dig);
        let (rest, dig) = parse_digit("ztwo3")?;
        assert_eq!(2, dig);
        let (rest, dig) = parse_digit("z3two3")?;
        assert_eq!(3, dig);
        let (rest, dig) = parse_digit(rest)?;
        assert_eq!(2, dig);
        Ok(())
    }

    #[test]
    fn test_parse_digits() -> Result<()> {
        let (_, digits) = parse_digits("two1nine")?;
        assert_eq!(vec![2, 1, 9], digits);
        let (_, digits) = parse_digits("eightwothree")?;
        assert_eq!(vec![8, 2, 3], digits);

        Ok(())
    }

    #[test]
    fn test_process() -> Result<()> {
        let input = include_str!("../input2_part2.txt");
        assert_eq!(281, process(input)?);
        Ok(())
    }
}
