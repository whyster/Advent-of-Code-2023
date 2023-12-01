use anyhow::Result;
use {{crate_name}}::part2::process;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let input= include_str!("../../input1.txt");
    let result = process(file).context("process part 1")?;
    println!("{}", result);
    Ok(())
}
