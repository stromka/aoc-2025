use std::fs::read_to_string;
use std::path::Path;

pub fn read_txt(path: &Path) -> Vec<String> {
    let file_text = read_to_string(path).expect("Failed to read file");

    file_text.lines().map(|line| line.to_string()).collect()
}

pub fn process_input(data: String) -> Vec<(usize, usize)> {
    let elements = data.split(',').map(|s| {
        s.to_owned().clone()
    }).collect::<Vec<String>>();

    let pairs = elements.iter().map(|s| {
        s.split('-').map(|v| v).collect::<Vec<&str>>()
    }).map(|p| {
        (p[0].parse::<usize>().unwrap(), p[1].parse::<usize>().unwrap())
    }).collect::<Vec<(usize, usize)>>();

    return pairs
}

pub fn is_valid_id(id: usize) -> bool {
    let id_str = id.to_string();

    // an odd length is automatically valid
    if id_str.len() % 2 != 0 { return true }

    let (half_1, half_2) = id_str.split_at(id_str.len() / 2);

    if half_1 == half_2 {
        return false
    }
    true
}

#[tracing::instrument]
pub fn process(path: &Path) -> anyhow::Result<usize> {
    let text = read_txt(&path);
    let row = text.iter().next().unwrap().clone();
    let input = process_input(row);

    // iterate through numbers in each range. 
    // convert to string and split in half
    // see if the halves are the same
    let mut sum = 0;

    input.iter().for_each(|(min, max)| {
        for id in *min..=*max {
            if !is_valid_id(id) {
                sum += id
            }
        }
    });

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = Path::new("./../inputs/day2_part1_sample.csv");

        assert_eq!(1227775554, process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_input() -> anyhow::Result<()> {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124".to_string();

        assert_eq!(11, process_input(input).len());
        Ok(())
    }

    #[test]
    fn test_process_full() -> anyhow::Result<()> {
        let input = Path::new("./../inputs/day2_part1.csv");

        assert_eq!(54234399924, process(input)?);
        Ok(())
    }

    #[test]
    fn test_is_valid_id_1() -> anyhow::Result<()> {
        assert!(!is_valid_id(11));
        assert!(is_valid_id(12));

        assert!(is_valid_id(101));
        assert!(is_valid_id(1122));
        assert!(!is_valid_id(1212));

        assert!(!is_valid_id(1188511885));

        Ok(())
    }
}
