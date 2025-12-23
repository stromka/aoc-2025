use std::fs::read_to_string;
use std::path::Path;

pub fn read_txt(path: &Path) -> Vec<String> {
    let file_text = read_to_string(path).expect("Failed to read file");

    file_text.lines().map(|line| line.to_string()).collect()
}

#[tracing::instrument]
pub fn process(input: &[u8]) -> anyhow::Result<String> {
    todo!("day 01 - part 1");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = b"";

        assert_eq!("", process(input)?);
        Ok(())
    }
}
