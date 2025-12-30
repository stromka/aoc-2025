use std::path::Path;
use std::fs::read_to_string;

pub fn read_txt(path: &Path) -> Vec<Vec<char>> {
    let file_text = read_to_string(path).expect("Failed to read file");

    let mut problems = vec![];

    let mut lines = file_text.lines();

    while let Some(line) = lines.next() {
        problems.push(line.chars().collect::<Vec<char>>());
    }

    problems
}

pub fn create_num_from_chars(chars: &[char]) -> Option<usize> {
    if chars.iter().all(|c| c == &' ') {
        return None
    }
    // iterate backwards over the char
    let mut multiple = 1;
    let mut value = 0;
    let n_chars = chars.len();

    for i in 0..n_chars {
        if chars[n_chars - i - 1] == ' ' { continue }
        value += (chars[n_chars - i - 1] as u32 - '0' as u32) * multiple;
        multiple *= 10;
    }

    Some(value as usize)
}

pub fn solve_problems(data: &Vec<Vec<char>>) -> Vec<usize> {
    let mut problems = vec![];
    let n_vals = data.len() - 1;
    let n_cols = data[0].len();
    let mut operator = ' ';

    let mut numbers = vec![];
    for j in (0..n_cols).rev() {
        // to skip any column separators
        if operator != ' ' {
            operator = data[n_vals][j];
            continue 
        }

        // we always compute the number from the row
        let num = create_num_from_chars(&(0..n_vals).map(|v| data[v][j]).collect::<Vec<char>>());
        numbers.push(num);

        if data[n_vals][j] != ' ' {
            operator = data[n_vals][j];
            let value = match operator {
                '*' => numbers.iter().flatten().product(),
                '+' => numbers.iter().flatten().sum(),
                _ => unreachable!()
            };
            problems.push(value);
            numbers.clear();
        }
    }
    problems
}

#[tracing::instrument]
pub fn process(input: &Path) -> anyhow::Result<usize> {
    let data = read_txt(input);
    let solutions = solve_problems(&data);

    Ok(solutions.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = Path::new("../inputs/day6_sample.txt");

        assert_eq!(3263827, process(&input)?);
        Ok(())
    }

    #[test]
    fn test_process_full() -> anyhow::Result<()> {
        let input = Path::new("../inputs/day6.txt");

        assert_eq!(11052310600986, process(&input)?);
        Ok(())
    }

    #[test]
    fn test_read_txt_small() {
        let input = Path::new("../inputs/day6_sample.txt");

        let res = read_txt(input);

        let line1 = vec!['1', '2', '3', ' ', '3', '2', '8', ' ', ' ', '5', '1', ' ', '6', '4', ' '];
        assert_eq!(res[0], line1);

        let line2 = vec![' ', '4', '5', ' ', '6', '4', ' ', ' ', '3', '8', '7', ' ', '2', '3', ' '];
        assert_eq!(res[1], line2);

        let line3 = vec![' ', ' ', '6', ' ', '9', '8', ' ', ' ', '2', '1', '5', ' ', '3', '1', '4'];
        assert_eq!(res[2], line3);

        let line4 = vec!['*', ' ', ' ', ' ', '+', ' ', ' ', ' ', '*', ' ', ' ', ' ', '+', ' ', ' '];
        assert_eq!(res[3], line4);
    }

    #[rstest]
    #[case::all_nums(vec!['1', '2', '3'], Some(123))]
    #[case::all_nums(vec![' ', ' ', '3'], Some(3))]
    #[case::all_nums(vec!['1', ' ', ' '], Some(1))]
    #[case::all_nums(vec!['1', '2', ' '], Some(12))]
    #[case::all_nums(vec![' ', '2', '3'], Some(23))]
    #[case::all_nums(vec![' ', ' ', ' '], None)]
    fn test_create_num_from_chars(#[case] vals: Vec<char>, #[case] answer: Option<usize>) {
        assert_eq!(create_num_from_chars(&vals), answer)
    }

    #[test]
    fn test_solve_problems_new() {
        let data = vec![
            vec!['1', '2', '3', ' ', '3', '2', '8', ' ', ' ', '5', '1', ' ', '6', '4', ' '],
            vec![' ', '4', '5', ' ', '6', '4', ' ', ' ', '3', '8', '7', ' ', '2', '3', ' '],
            vec![' ', ' ', '6', ' ', '9', '8', ' ', ' ', '2', '1', '5', ' ', '3', '1', '4'],
            vec!['*', ' ', ' ', ' ', '+', ' ', ' ', ' ', '*', ' ', ' ', ' ', '+', ' ', ' '],
        ];

        let res = solve_problems(&data);

        let answer = vec![
            vec![623, 431, 4].iter().sum(),
            vec![32, 581, 175].iter().product(),
            vec![369, 248, 8].iter().sum(),
            vec![1, 24, 356].iter().product(),
        ];

        assert_eq!(res, answer);
    }
}
