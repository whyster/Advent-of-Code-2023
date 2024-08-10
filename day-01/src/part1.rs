use anyhow::Result;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while_m_n};
use nom::character::complete::{alpha0, alpha1, digit0, digit1};
use nom::character::{is_alphabetic, is_alphanumeric};
use nom::multi::{many0, many1};
use nom::IResult;
use std::mem::take;
use tracing::field::debug;
use tracing::{debug, event, info, instrument, trace, Level};

/*fn get_digits(input: &str) -> IResult<&str, Vec<&str>> {
    many1(alt((tag("one"), tag("five"), digit1, al)))(input)
}*/

#[instrument]
pub fn process(input: &str) -> Result<i32> {
    let mut sum: i32 = 0;
    for line in input.lines() {
        let mut first_digit: i32 = i32::MIN;
        let mut last_digit: i32 = i32::MIN;
        // let result = get_digits(line).unwrap();
        // println!("Test");
        // dbg!(&result);
        // event!(Level::INFO, "something happened");

        //info!("HELP ME {:?}", result.1);
        for character in line.chars() {
            if let Some(digit) = character.to_digit(10) {
                if first_digit == i32::MIN {
                    first_digit = digit as i32;
                }
                last_digit = digit as i32;
            }
        }
        sum += (first_digit * 10) + last_digit;
        // let result_vec = result.1;
        // return Ok(i32::from_str_radix(result_vec[0], 10)?);
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        tracing_subscriber::fmt::init();
        let input = include_str!("../input2.txt");
        assert_eq!(142, process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_input1() -> Result<()> {
        let input = include_str!("../input1.txt");
        assert_eq!(53651, process(input)?);
        Ok(())
    }
}
