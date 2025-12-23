use std::collections::HashSet;
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

pub fn split_string(id_str: &String, chunk_size: usize) -> HashSet<&str> {
    // collect the substrings
    let mut chunks = HashSet::new();
    let mut start = 0;

    while start < id_str.len() {
        let end = start + chunk_size - 1;
        let chunk = &id_str[start..=end];

        chunks.insert(chunk);
        start += chunk_size;
    }

    chunks
}

pub fn is_valid_id(id: usize) -> bool {
    let id_str = id.to_string();

    for chunk_size in 1..=(id_str.len() / 2) {
        // only check chunk lengths that fit
        if id_str.len() % chunk_size == 0 {
            let chunks = split_string(&id_str, chunk_size);
            // we exit out with false if we encounter any invalid chunking
            if chunks.len() == 1 {
                return false
            }
        }
    }

    return true
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

        assert_eq!(4174379265, process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_full() -> anyhow::Result<()> {
        let input = Path::new("./../inputs/day2_part1.csv");

        assert_eq!(70187097315, process(input)?);
        Ok(())
    }

    #[test]
    fn test_split_string() -> anyhow::Result<()> {
        let mut set1 = HashSet::new();
        set1.insert("1");
        assert_eq!(set1, split_string(&"11".to_string(), 1));

        let mut set1 = HashSet::new();
        set1.insert("11");
        set1.insert("22");
        set1.insert("33");
        assert_eq!(set1, split_string(&"112233".to_string(), 2));

        let mut set1 = HashSet::new();
        set1.insert("1");
        set1.insert("2");
        assert_eq!(set1, split_string(&"211".to_string(), 1));

        let mut set1 = HashSet::new();
        set1.insert("1");
        set1.insert("2");
        assert_eq!(set1, split_string(&"112".to_string(), 1));

        Ok(())
    }

    #[test]
    fn test_is_valid_id_1() -> anyhow::Result<()> {
        assert!(!is_valid_id(11));
        assert!(is_valid_id(12));

        assert!(is_valid_id(101));
        assert!(!is_valid_id(111));
        assert!(is_valid_id(1122));
        assert!(!is_valid_id(1212));
        assert!(!is_valid_id(121212));

        assert!(!is_valid_id(9999999));
        assert!(is_valid_id(9999990));

        assert!(!is_valid_id(1188511885));

        Ok(())
    }
}
