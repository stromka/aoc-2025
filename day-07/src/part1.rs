use std::path::Path;
use std::fs::read_to_string;

/*
Two options:
1. Grid
Create a grid. Start at the S, go down line by line looking at what happens at the row above

*/

#[derive(PartialEq, Eq, Debug)]
pub enum Tachyon {
    Air,
    Beam,
    Splitter,
}

pub struct AirSpace {
    grid: Vec<Vec<Tachyon>>,
    start_index: usize,
    n_col: usize,
    n_row: usize,
}

impl AirSpace {
    pub fn insert_beam(&mut self, i: usize, j: usize) {
        // we don't replace any splitters with beams
        // if self.grid[i][j] != Tachyon::Splitter {
        //     self.grid[i][j] = Tachyon::Beam
        // }
        self.grid[i][j] = Tachyon::Beam
    }

    pub fn pew_pew(&mut self) -> usize {
        let mut visited_splitters = 0;
        for i in 1..self.n_row {
            visited_splitters += self.pew(i);
        }

        visited_splitters
    }

    pub fn pew(&mut self, row_index: usize) -> usize {
        let mut visited_splitters = 0;
        for j in 0..self.n_col {
            // we only care about this spot if there is a beam above
            let incoming = &self.grid[row_index-1][j];
            if incoming == &Tachyon::Beam {
                match self.grid[row_index][j] {
                    Tachyon::Air => self.insert_beam(row_index, j),
                    Tachyon::Splitter => {
                        self.insert_beam(row_index, j-1);
                        self.insert_beam(row_index, j+1);
                        visited_splitters += 1;
                    }
                    _ => continue
                }
            }
        }

        visited_splitters
    }
}

impl From<String> for AirSpace {
    fn from(s: String) -> AirSpace {
        let mut start = 0;
        let grid = s.lines().map(|line| line.chars().enumerate().map(|(i, c)| match c {
            'S' => {
                start = i;
                Tachyon::Beam
            },
            '.' => Tachyon::Air,
            '^' => Tachyon::Splitter,
            _ => unreachable!()
        }).collect::<Vec<Tachyon>>()).collect::<Vec<Vec<Tachyon>>>();

        let n_col = grid[0].len();
        let n_row = grid.len();

        AirSpace {
            grid: grid,
            start_index: start,
            n_row: n_row,
            n_col: n_col,
        }
    }
}

pub fn process(input: &Path) -> anyhow::Result<usize> {
    let file_text = read_to_string(input).expect("Failed to read file");

    let mut airspace = AirSpace::from(file_text);
    let n_splitters = airspace.pew_pew();

    Ok(n_splitters)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = Path::new("../inputs/day7_sample.txt");
        let n_splitters = process(input)?;

        assert_eq!(n_splitters, 21);
        Ok(())
    }

    #[test]
    fn test_process_full() -> anyhow::Result<()> {
        let input = Path::new("../inputs/day7.txt");
        let n_splitters = process(input)?;

        assert_eq!(n_splitters, 1490);
        Ok(())
    }

    #[test]
    fn test_pew_1() {
        let two_rows = vec![
            vec![Tachyon::Air, Tachyon::Air, Tachyon::Air, Tachyon::Beam, Tachyon::Air],
            vec![Tachyon::Splitter, Tachyon::Air, Tachyon::Air, Tachyon::Splitter, Tachyon::Air],
        ];

        let mut grid = AirSpace {
            grid: two_rows,
            start_index: 0,
            n_col: 5,
            n_row: 2,
        };

        let visited_splitters = grid.pew(1);

        let answer = vec![
            vec![Tachyon::Air, Tachyon::Air, Tachyon::Air, Tachyon::Beam, Tachyon::Air],
            vec![Tachyon::Splitter, Tachyon::Air, Tachyon::Beam, Tachyon::Splitter, Tachyon::Beam],
        ];

        assert_eq!(grid.grid, answer);
        assert_eq!(visited_splitters, 1);
    }

    #[test]
    fn test_pew_2() {
        let two_rows = vec![
            vec![Tachyon::Air, Tachyon::Beam, Tachyon::Air, Tachyon::Beam, Tachyon::Air],
            vec![Tachyon::Splitter, Tachyon::Air, Tachyon::Air, Tachyon::Splitter, Tachyon::Air],
        ];

        let mut grid = AirSpace {
            grid: two_rows,
            start_index: 0,
            n_col: 5,
            n_row: 2,
        };

        let visited_splitters = grid.pew(1);

        let answer = vec![
            vec![Tachyon::Air, Tachyon::Beam, Tachyon::Air, Tachyon::Beam, Tachyon::Air],
            vec![Tachyon::Splitter, Tachyon::Beam, Tachyon::Beam, Tachyon::Splitter, Tachyon::Beam],
        ];

        assert_eq!(grid.grid, answer);
        assert_eq!(visited_splitters, 1);
    }

    #[test]
    fn test_make_airspace_tiny() {
        let data = 
            ".S.\n".to_owned() + 
            "...\n";

        let airspace = AirSpace::from(data);

        let grid = vec![
            vec![Tachyon::Air, Tachyon::Beam, Tachyon::Air],
            vec![Tachyon::Air, Tachyon::Air, Tachyon::Air],
        ];
        assert_eq!(airspace.grid, grid);
        assert_eq!(airspace.start_index, 1);
        assert_eq!(airspace.n_col, 3);
        assert_eq!(airspace.n_row, 2);
    }

    #[test]
    fn test_make_airspace_bigger() {
        let data = 
            "..S..\n".to_owned() + 
            ".....\n" + 
            "..^..\n" + 
            ".....\n" + 
            ".^.^.\n" +
            ".....\n";

        let airspace = AirSpace::from(data);

        let grid = vec![
            vec![Tachyon::Air, Tachyon::Air, Tachyon::Beam, Tachyon::Air, Tachyon::Air],
            vec![Tachyon::Air, Tachyon::Air, Tachyon::Air, Tachyon::Air, Tachyon::Air],
            vec![Tachyon::Air, Tachyon::Air, Tachyon::Splitter, Tachyon::Air, Tachyon::Air],
            vec![Tachyon::Air, Tachyon::Air, Tachyon::Air, Tachyon::Air, Tachyon::Air],
            vec![Tachyon::Air, Tachyon::Splitter, Tachyon::Air, Tachyon::Splitter, Tachyon::Air],
            vec![Tachyon::Air, Tachyon::Air, Tachyon::Air, Tachyon::Air, Tachyon::Air],
        ];
        assert_eq!(airspace.grid, grid);
        assert_eq!(airspace.start_index, 2);
        assert_eq!(airspace.n_col, 5);
        assert_eq!(airspace.n_row, 6);
    }


}
