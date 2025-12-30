use std::path::Path;

#[tracing::instrument]
pub fn process(input: &Path) -> anyhow::Result<String> {
    todo!("day 01 - part 2");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = Path::new("something");

        assert_eq!("", process(input)?);
        Ok(())
    }
}
