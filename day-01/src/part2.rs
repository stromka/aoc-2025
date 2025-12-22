use std::fs::File;

pub enum Direction {
    Left,
    Right,
}

struct Move {
    direction: Direction,
    distance: i32,
}

#[tracing::instrument]
pub fn process() -> anyhow::Result<(i32, i32)> {
    let file = File::open("./../inputs/day1_part1.csv")?;
    let mut reader = csv::ReaderBuilder::new().has_headers(false).from_reader(file);

    let turns = reader.records().map(|r| {
        let record = r.unwrap().as_slice().to_owned();
        let split_record = &record.split_at(1);
        let direction = match split_record.0 {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Invalid direction"),
        };
        let distance: i32 = split_record.1.parse().unwrap();
        Move { direction, distance }
    }).collect::<Vec<Move>>();

    let (n_zeros, dial) = process_moves(turns);

    Ok((n_zeros, dial))
}

fn process_moves(moves: Vec<Move>) -> (i32, i32) {
    let mut dial = 50;
    let mut n_zeros = 0;

    moves.iter().for_each(|m| {
        match m.direction {
            Direction::Left => {
                let full_rotations = m.distance / 100;
                n_zeros += full_rotations;
                let dial_was = dial;

                dial -= m.distance % 100;
                if dial < 0 {
                    if dial_was != 0 {
                        n_zeros += 1
                    }
                    dial += 100;
                }
            },
            Direction::Right => {
                let full_rotations = m.distance / 100;
                n_zeros += full_rotations;
                dial += m.distance % 100;

                if dial > 100 {
                    n_zeros += 1
                }
                if dial > 99 {
                    dial -= 100;
                }
            }
        };
        if dial == 0 { n_zeros += 1; }
        println!("{n_zeros}, {dial}")
    });
    (n_zeros, dial)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        // assert_eq!("", process(input)?);
        let (z, d) = process()?;
        assert_eq!(6412, z);
        Ok(())
    }

    #[test]
    fn test_simple() -> anyhow::Result<()> {
        /*
        L68
        L30
        R48
        L5
        R60
        L55
        L1
        L99
        R14
        L82
         */
        let moves = vec![
            Move { direction: Direction::Left, distance: 68 }, // 1, 82
            Move { direction: Direction::Left, distance: 30 }, // 1, 52
            Move { direction: Direction::Right, distance: 48 }, // 2, 0
            Move { direction: Direction::Left, distance: 5 }, // 2, 95
            Move { direction: Direction::Right, distance: 60 },
            Move { direction: Direction::Left, distance: 55 },
            Move { direction: Direction::Left, distance: 1 },
            Move { direction: Direction::Left, distance: 99 },
            Move { direction: Direction::Right, distance: 14 },
            Move { direction: Direction::Left, distance: 82 },
        ];
        // assert_eq!("", process(input)?);
        let (z, d) = process_moves(moves);
        assert_eq!(6, z);
        Ok(())
    }

    #[test]
    fn test_process_moves_right_49() {
        let moves = vec![
            Move { direction: Direction::Right, distance: 49 },
        ];

        let (z, d) = process_moves(moves);
        assert_eq!(99, d);
        assert_eq!(0, z);
    }

    #[test]
    fn test_process_moves_right_51() {
        let moves = vec![
            Move { direction: Direction::Right, distance: 51}
        ];

        let (z, d) = process_moves(moves);
        assert_eq!(1, d);
        assert_eq!(1, z);
    }

    #[test]
    fn test_process_moves_right_151() {
        let moves = vec![
            Move { direction: Direction::Right, distance: 151}
        ];

        let (z, d) = process_moves(moves);
        assert_eq!(1, d);
        assert_eq!(2, z);
    }

    #[test]
    fn test_process_moves_right_50() {
        let moves = vec![
            Move { direction: Direction::Right, distance: 50 },
        ];

        let (z, d) = process_moves(moves);
        assert_eq!(0, d);
        assert_eq!(1, z);
    }

    #[test]
    fn test_process_moves_right_150() {
        let moves = vec![
            Move { direction: Direction::Right, distance: 150 },
        ];

        let (z, d) = process_moves(moves);
        assert_eq!(0, d);
        assert_eq!(2, z);
    }

    #[test]
    fn test_process_moves_right_149() {
        let moves = vec![
            Move { direction: Direction::Right, distance: 149 },
        ];

        let (z, d) = process_moves(moves);
        assert_eq!(99, d);
        assert_eq!(1, z);
    }

    #[test]
    fn test_process_moves_left_50() {
        let moves = vec![
            Move { direction: Direction::Left, distance: 50 },
        ];

        let (z, d) = process_moves(moves);
        assert_eq!(0, d);
        assert_eq!(1, z);
    }

    #[test]
    fn test_process_moves_left_49() {
        let moves = vec![
            Move { direction: Direction::Left, distance: 49 },
        ];

        let (z, d) = process_moves(moves);
        assert_eq!(1, d);
        assert_eq!(0, z);
    }

    #[test]
    fn test_process_moves_left_51() {
        let moves = vec![
            Move { direction: Direction::Left, distance: 51 },
        ];

        let (z, d) = process_moves(moves);
        assert_eq!(99, d);
        assert_eq!(1, z);
    }

    #[test]
    fn test_process_moves_left_150() {
        let moves = vec![
            Move { direction: Direction::Left, distance: 150 },
        ];

        let (z, d) = process_moves(moves);
        assert_eq!(0, d);
        assert_eq!(2, z);
    }

    #[test]
    fn test_process_moves_left_149() {
        let moves = vec![
            Move { direction: Direction::Left, distance: 149 },
        ];

        let (z, d) = process_moves(moves);
        assert_eq!(1, d);
        assert_eq!(1, z);
    }

    #[test]
    fn test_process_moves_left_151() {
        let moves = vec![
            Move { direction: Direction::Left, distance: 151 },
        ];

        let (z, d) = process_moves(moves);
        assert_eq!(99, d);
        assert_eq!(2, z);
    }
}
