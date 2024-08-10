use anyhow::{Context, Result};
use day_02::part1::process;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let input = include_str!("../../input1.txt");
    let result = process(input).context("process part 1")?;
    println!("{}", result);
    Ok(())
}
