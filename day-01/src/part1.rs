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
pub fn process() -> anyhow::Result<(usize, i32)> {
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

fn process_moves(moves: Vec<Move>) -> (usize, i32) {
    let mut dial = 50;
    let mut n_zeros: usize = 0;

    moves.iter().for_each(|m| {
        match m.direction {
            Direction::Left => {
                dial -= m.distance % 100;
                if dial < 0 {
                    dial += 100;
                }
            },
            Direction::Right => {
                dial += m.distance % 100;
                if dial > 99 {
                    dial -= 100;
                }
            }
        };
        if dial == 0 { n_zeros += 1; }
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
        assert_eq!(1078, z);
        Ok(())
    }

    #[test]
    fn test_process_moves_99() {
        let moves = vec![
            Move { direction: Direction::Right, distance: 49 },
        ];

        let (z, d) = process_moves(moves);
        assert_eq!(99, d);
        assert_eq!(0, z);
    }

    #[test]
    fn test_process_moves_100() {
        let moves = vec![
            Move { direction: Direction::Right, distance: 50 },
        ];

        let (z, d) = process_moves(moves);
        assert_eq!(0, d);
        assert_eq!(1, z);
    }

    #[test]
    fn test_process_moves_149() {
        let moves = vec![
            Move { direction: Direction::Right, distance: 149 },
        ];

        let (z, d) = process_moves(moves);
        assert_eq!(99, d);
        assert_eq!(0, z);
    }

    #[test]
    fn test_process_moves_left_zero() {
        let moves = vec![
            Move { direction: Direction::Left, distance: 50 },
        ];

        let (z, d) = process_moves(moves);
        assert_eq!(0, d);
        assert_eq!(1, z);
    }

    #[test]
    fn test_process_moves_left_99() {
        let moves = vec![
            Move { direction: Direction::Left, distance: 51 },
        ];

        let (z, d) = process_moves(moves);
        assert_eq!(99, d);
        assert_eq!(0, z);
    }
}
