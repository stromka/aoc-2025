use std::path::Path;
use std::collections::BTreeSet;
use crate::part1::{Range, read_txt, to_b_tree_set};


pub fn count_all_fresh_ids(tree: &BTreeSet<Range>) -> isize {
    let mut count = 0;
    for range in tree.iter() {
        count += range.max - range.min + 1
    }
    count
}

pub fn process_b_tree_set(path: &Path) -> anyhow::Result<usize> {
    let (ranges, _values) = read_txt(path);
    let tree = to_b_tree_set(ranges);

    let count = count_all_fresh_ids(&tree);

    Ok(count as usize)
}

#[tracing::instrument]
pub fn process(path: &Path) -> anyhow::Result<usize> {
    let count = process_b_tree_set(path);

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = Path::new("./../inputs/day5_sample.txt");

        assert_eq!(14, process(input)?);
        Ok(())
    }

    #[test]
    fn test_proces_full() -> anyhow::Result<()> {
        let input = Path::new("./../inputs/day5.txt");

        assert_eq!(357485433193284, process(input)?);
        Ok(())
    }

    #[test]
    fn test_read_txt() -> anyhow::Result<()> {
        let input = Path::new("./../inputs/day5_sample.txt");

        let (ranges, unknowns) = read_txt(input);

        let real_ranges = vec![(3, 5), (10, 14), (16, 20), (12, 18)];
        let real_unknowns = vec![1, 5, 8, 11, 17, 32];
        assert_eq!(ranges, real_ranges);
        assert_eq!(unknowns, real_unknowns);
        Ok(())
    }

    #[test]
    fn test_range_ordering() {
        let mut vals = vec![
            Range { min: 5, max: 10 },
            Range { min: 6, max: 12 },
            Range { min: 6, max: 5 }, // should get put before the last
            Range { min: 4, max: 6 },
            Range { min: 5, max: 10}, // an identical range
            Range { min: 6, max: 10},
            Range { min: 4, max: 10},
        ];

        vals.sort();

        let sorted = vec![
            Range { min: 6, max: 5 }, // should get put before the last
            Range { min: 4, max: 6 },
            Range { min: 4, max: 10},
            Range { min: 5, max: 10 },
            Range { min: 5, max: 10}, // an identical range
            Range { min: 6, max: 10},
            Range { min: 6, max: 12 },
        ];

        assert_eq!(vals, sorted);
    }

    #[test]
    fn test_count_all_fresh_ids() {
        let ranges = vec![
            (20, 30),
            (6, 10),
            (12, 15),
            (3, 5),
            (14, 15),
        ];

        let btree = to_b_tree_set(ranges);

        let final_ranges = vec![
            &Range { min: 3, max: 10 },
            &Range { min: 12, max: 15 },
            &Range { min: 20, max: 30 },
        ];

        let btree_ranges = btree.iter().collect::<Vec<&Range>>();
        assert_eq!(btree_ranges, final_ranges);

        let res = count_all_fresh_ids(&btree);

        assert_eq!(res, 23)
    }

    #[rstest]
    #[case::one_gap_right((0, 4), (6, 10), None)] // one gap right
    #[case::one_gap_left((6, 10), (0, 4), None)] // one gap left
    #[case::right_adjacent((0, 4), (5, 10), Some(Range { min: 0, max: 10}))] // right adjacent
    #[case::right_equal((0, 5), (5, 10), Some(Range { min: 0, max: 10}))] // right equal
    #[case::right_overlap((0, 5), (4, 10), Some(Range { min: 0, max: 10}))] // right overlap
    #[case::left_adjacent((5, 10), (0, 4), Some(Range { min: 0, max: 10}))] // left adjacent
    #[case::left_equal((5, 10), (0, 5), Some(Range { min: 0, max: 10}))] // left equal
    #[case::left_overlap((5, 10), (0, 6), Some(Range { min: 0, max: 10}))] // left overlap
    #[case::left_inside((4, 8), (0, 10), Some(Range { min: 0, max: 10}))] // left inside
    #[case::right_inside((0, 10), (4, 8), Some(Range { min: 0, max: 10}))] // right inside
    #[case::equal((0, 10), (0, 10), Some(Range { min: 0, max: 10}))] // equal
    #[case::max_eq_left_min_less((0, 10), (5, 10), Some(Range { min: 0, max: 10}))] // maxes equal, left min less
    #[case::max_eq_right_min_less((5, 10), (0, 10), Some(Range { min: 0, max: 10}))] // maxes equal, right min less
    #[case::min_eq_left_max_less((0, 5), (0, 10), Some(Range { min: 0, max: 10}))] // mins equal, left max less
    #[case::min_eq_right_max_less((0, 10), (0, 5), Some(Range { min: 0, max: 10}))] // mins equal, right max less
    fn test_range_merge(#[case] range1: (isize, isize), #[case] range2: (isize, isize), #[case] answer: Option<Range>) {
        let range_1 = Range { min: range1.0, max: range1.1 };
        let range_2 = Range { min: range2.0, max: range2.1 };

        let res = range_1.merge(&range_2);

        assert_eq!(res, answer);
    }
}
