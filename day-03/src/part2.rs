use std::fs::read_to_string;
use std::path::Path;

pub fn read_txt(path: &Path) -> Vec<Vec<usize>> {
    let file_text = read_to_string(path).expect("Failed to read file");

    file_text.lines().map(|line| process_line(line)).collect::<Vec<Vec<usize>>>()
}

pub fn process_line(line: &str) -> Vec<usize> {
    line.chars().map(|ch| ch as usize - 0x30).collect::<Vec<usize>>()
}

pub fn find_highest_set(vals: &Vec<usize>, n: usize) -> Vec<usize> {
    /*
    Two passes. first find the highest value, not including the last one
    Then find the highest value following the current highest, including the last one
     */

    let mut values = vec![0; n];

    let mut index = 0;
    for k in 0..n {
        let mut highest = 0;

        let last_digit_to_search = vals.len() - (n - k) + 1;
        for i in index..last_digit_to_search {
            if vals[i] > highest {
                highest = vals[i];
                index = i+1;
            }
        }
        values[k] = highest;
    }

    return values
}

pub fn create_digit(values: Vec<usize>) -> i64 {
    let n_digits = values.len() as u32;
    let sum = values.iter().enumerate().map(|(i, val)| *val as i64 * (10 as i64).pow(n_digits - i as u32 - 1 as u32)).sum();

    sum
}

#[tracing::instrument]
pub fn process(path: &Path) -> anyhow::Result<i64> {
    let data = read_txt(path);

    let sum = data.iter().map(|ls| {
        let vals = find_highest_set(ls, 12);
        create_digit(vals)
    }).sum();

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = Path::new("./../inputs/day3_sample.csv");

        assert_eq!(3121910778619, process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_full() -> anyhow::Result<()> {
        let input = Path::new("./../inputs/day3.csv");

        assert_eq!(168575096286051, process(input)?);
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
        let res = find_highest_set(&vec![0, 1, 2, 3], 2);
        assert_eq!(res, vec![2, 3]);
        Ok(())
    }

    #[test]
    fn test_find_highest_pair_first() -> anyhow::Result<()> {
        let res = find_highest_set(&vec![9, 8, 1, 2], 2);
        assert_eq!(res, vec![9, 8]);
        Ok(())
    }

    #[test]
    fn test_find_highest_pair_same_digit() -> anyhow::Result<()> {
        let res = find_highest_set(&vec![8, 9, 9, 1], 2);
        assert_eq!(res, vec![9, 9]);
        Ok(())
    }

    #[test]
    fn test_find_highest_pair_second() -> anyhow::Result<()> {
        let res = find_highest_set(&vec![8, 9, 4, 5, 8], 2);
        assert_eq!(res, vec![9, 8]);
        Ok(())
    }

    #[test]
    fn test_find_highest_trio() -> anyhow::Result<()> {
        let res = find_highest_set(&vec![8, 9, 4, 5, 8], 3);
        assert_eq!(res, vec![9, 5, 8]);
        Ok(())
    }

    #[test]
    fn test_find_highest_quad() -> anyhow::Result<()> {
        let res = find_highest_set(&vec![8, 9, 4, 1, 1, 5, 1, 1, 1, 8], 4);
        assert_eq!(res, vec![9, 5, 1, 8]);
        Ok(())
    }

    #[test]
    fn test_create_digit() -> anyhow::Result<()> {
        let vals = vec![9, 5, 1, 8];
        let num = create_digit(vals);

        assert_eq!(num, 9518);
        Ok(())
    }
}
