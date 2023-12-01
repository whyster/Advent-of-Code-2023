use anyhow::Result;

fn process(input: &str) -> Result<i32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        todo!("haven't built test yet");
        let input = include_str!("../../input2.txt");
        assert_eq!("", process(input)?);
        Ok(())
    }
}
