use anyhow::{Context, Result};
use day_02::part2::process;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let input = include_str!("../../input1.txt");
    let result = process(input).context("process part 2")?;
    println!("{}", result);
    Ok(())
}
