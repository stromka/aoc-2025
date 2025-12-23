use std::fs::read_to_string;
use std::path::Path;

pub fn read_txt(path: &Path) -> Vec<Vec<usize>> {
    let file_text = read_to_string(path).expect("Failed to read file");

    file_text.lines().map(|line| process_line(line)).collect::<Vec<Vec<usize>>>()
}

pub fn process_line(line: &str) -> Vec<usize> {
    line.chars().map(|ch| ch as usize - 0x30).collect::<Vec<usize>>()
}

pub fn find_highest_pair(vals: &Vec<usize>) -> (usize, usize) {
    /*
    Two passes. first find the highest value, not including the last one
    Then find the highest value following the current highest, including the last one
     */
    let mut first_highest = 0;
    let mut first_index = 0;
    let mut second_highest = 0;

    for i in 0..vals.len()-1 {
        if vals[i] > first_highest {
            first_highest = vals[i];
            first_index = i;
        }
    }

    for j in first_index+1..vals.len() {
        if vals[j] > second_highest {
            second_highest = vals[j];
        }
    }

    return (first_highest, second_highest)
}

pub fn create_digit(first: usize, second: usize) -> usize {
    first * 10 + second
}

#[tracing::instrument]
pub fn process(path: &Path) -> anyhow::Result<usize> {
    let data = read_txt(path);

    let sum = data.iter().map(|ls| {
        let (first, second) = find_highest_pair(ls);
        create_digit(first, second)
    }).sum();

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = Path::new("./../inputs/day3_sample.csv");

        assert_eq!(357, process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_full() -> anyhow::Result<()> {
        let input = Path::new("./../inputs/day3.csv");

        assert_eq!(17031, process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_line() -> anyhow::Result<()> {
        let res = process_line("0123456789");

        assert_eq!(res, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        Ok(())
    }

    #[test]
    fn test_find_highest_pair_last() -> anyhow::Result<()> {
        let res = find_highest_pair(&vec![0, 1, 2, 3]);
        assert_eq!(res, (2, 3));
        Ok(())
    }

    #[test]
    fn test_find_highest_pair_first() -> anyhow::Result<()> {
        let res = find_highest_pair(&vec![9, 8, 1, 2]);
        assert_eq!(res, (9, 8));
        Ok(())
    }

    #[test]
    fn test_find_highest_pair_same_digit() -> anyhow::Result<()> {
        let res = find_highest_pair(&vec![8, 9, 9, 1]);
        assert_eq!(res, (9, 9));
        Ok(())
    }

    #[test]
    fn test_find_highest_pair_second() -> anyhow::Result<()> {
        let res = find_highest_pair(&vec![8, 9, 4, 5, 8]);
        assert_eq!(res, (9, 8));
        Ok(())
    }
}
