use std::fs::read_to_string;
use std::path::Path;

pub fn read_txt(path: &Path) -> Vec<Vec<usize>> {
    let file_text = read_to_string(path).expect("Failed to read file");

    file_text.lines().map(|line| line.chars().map(|c| match c {
        '@' => 1,
        '.' => 0,
        _ => unreachable!()
    }).collect::<Vec<usize>>()).collect::<Vec<Vec<usize>>>()
}

pub fn buffered_grid(vec: Vec<Vec<usize>>) -> (Vec<Vec<usize>>, usize, usize) {
    let n_rows = vec.len();
    let n_cols = vec.first().unwrap().len();

    let mut buffered = vec![vec![0; n_rows + 2]; n_cols + 2];

    for (i, row) in vec.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            buffered[i+1][j+1] = *val
        }
    }

    return (buffered, n_rows + 2, n_cols + 2)
}

pub fn find_n_blocking(vec: &Vec<Vec<usize>>, row: usize, col: usize) -> Option<usize> {
    if vec[row][col] == 0 { return None }

    let mut sum = 0;

    for i in row-1..=row+1 {
        for j in col-1..=col+1 {
            if (i == row) & (j == col) {
            continue;
            }
            sum += vec[i][j];
        }
    }
    return Some(sum)
}

pub fn find_coords_accessible(grid: &Vec<Vec<usize>>, n_rows: usize, n_cols: usize) -> Vec<(usize, usize)> {
    let mut debug = vec![vec!['.'; n_rows - 2]; n_cols - 2];
    let mut coords = vec![];
    
    for i in 1..n_rows-1 {
        for j in 1..n_cols-1 {
            let n_blocking = find_n_blocking(&grid, i, j);
            if let Some(n) = n_blocking {
                if n < 4 { 
                    debug[i-1][j-1] = 'x';
                    coords.push((i, j));
                } else {
                    debug[i-1][j-1] = '@';
                }
            }
        }
    }

    for line in debug.iter() {
        let line_str = line.iter().collect::<String>();
        println!("{line_str}");
    }

    coords
}

pub fn apply_coords(grid: &mut Vec<Vec<usize>>, coords: &Vec<(usize, usize)>) {
    for (i, j) in coords.iter() {
        grid[*i][*j] = 0;
    }
}

pub fn remove_all_tp(grid: &mut Vec<Vec<usize>>, n_rows: usize, n_cols: usize) -> usize {
    let mut total = 0;

    let mut rounds = 1;

    loop {
        println!("\nround {rounds}");
        let accessible_rolls = find_coords_accessible(&grid, n_rows, n_cols);

        if accessible_rolls.len() == 0 { break }

        apply_coords(grid, &accessible_rolls);

        total += accessible_rolls.len();
        rounds += 1;

    }

    total
}

#[tracing::instrument]
pub fn process(path: &Path) -> anyhow::Result<usize> {
    let text = read_txt(path);
    let (mut grid, n_rows, n_cols) = buffered_grid(text);

    let total = remove_all_tp(&mut grid, n_rows, n_cols);

    return Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = Path::new("./../inputs/day4_sample.csv");

        assert_eq!(43, process(input)?);
        Ok(())
    }

     #[test]
    fn test_process_full() -> anyhow::Result<()> {
        let input = Path::new("./../inputs/day4.txt");

        assert_eq!(9397, process(input)?);
        Ok(())
    }

    #[test]
    fn test_buffered_grid() -> anyhow::Result<()> {
        let grid = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];

        let (res, row, col) = buffered_grid(grid);
        
        assert_eq!(row, 5);
        assert_eq!(col, 5);
        let answer = vec![
            vec![0, 0, 0, 0, 0],
            vec![0, 1, 2, 3, 0],
            vec![0, 4, 5, 6, 0],
            vec![0, 7, 8, 9, 0],
            vec![0, 0, 0, 0, 0],
        ];

        assert_eq!(res, answer);

        Ok(())
    }

    #[rstest]
    #[case((1, 1), Some(2))]
    #[case((1, 2), None)]
    #[case((1, 3), None)]
    #[case((2, 1), Some(3))]
    #[case((2, 2), Some(5))]
    #[case((2, 3), Some(3))]
    #[case((3, 1), None)]
    #[case((3, 2), Some(4))]
    #[case((3, 3), Some(3))]
    fn test_find_n_blocking(#[case] coords: (usize, usize), #[case] answer: Option<usize>) -> anyhow::Result<()> {

        let grid = vec![
            vec![0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 0],
            vec![0, 1, 1, 1, 0],
            vec![0, 0, 1, 1, 0],
            vec![0, 0, 0, 0, 0],
        ];

        let res = find_n_blocking(&grid, coords.0, coords.1);
        assert_eq!(res, answer);

        Ok(())
    }

    #[test]
    fn test_find_n_accessible_1() -> anyhow::Result<()> {

        let grid = vec![
            vec![0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 0],
            vec![0, 1, 1, 0, 0],
            vec![0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0],
        ];

        let res = find_coords_accessible(&grid, 5, 5);
        assert_eq!(res.len(), 4);

        Ok(())
    }

     #[test]
    fn test_find_n_accessible_0() -> anyhow::Result<()> {

        let mut grid = vec![
            vec![0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 0],
            vec![0, 1, 1, 1, 0],
            vec![0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0],
        ];

        let res = find_coords_accessible(&grid, 5, 5);
        assert_eq!(res.len(), 4);

        assert_eq!(res, vec![(1, 1), (2, 1), (2, 3), (3, 2)]);

        apply_coords(&mut grid, &res);

        let answer = vec![
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
        ];

        assert_eq!(grid, answer);

        Ok(())
    }

    #[test]
    fn test_remove_all_tp() -> anyhow::Result<()> {
        let mut grid = vec![
            vec![0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 0],
            vec![0, 1, 1, 1, 0],
            vec![0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0],
        ];

        let total = remove_all_tp(&mut grid, 5, 5);

        assert_eq!(total, 5);

        Ok(())
    }

}
