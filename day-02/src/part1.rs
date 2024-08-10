use anyhow::Result;

pub fn process(input: &str) -> Result<i32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = include_str!("../input2.txt");
        assert_eq!(8, process(input)?);
        Ok(())
    }
}
