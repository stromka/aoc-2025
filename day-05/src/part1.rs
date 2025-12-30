use std::fs::read_to_string;
use std::path::Path;
use std::collections::BTreeSet;
use std::cmp::{Ordering, PartialEq, PartialOrd, min, max};

#[derive(Hash, PartialEq, Eq, Debug, Ord, Clone, Copy)]
pub struct Range {
    min: isize,
    max: isize,
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // sort Range object first on Max, then Min
        if self.max == other.max {
            return self.min.partial_cmp(&other.min)
        } else {
            return self.max.partial_cmp(&other.max)
        }
    }
}

impl Range {
    fn merge(&self, other: &Self) -> Option<Range> {
        // [0, 10] - [4, 8]
        // [0, 4] - [5, 10]
        // [5, 10] - [0, 4]
        if (((self.max >= other.min - 1) & (self.max <= other.max)) | ((self.min <= other.max + 1) & (self.min >= other.min))) | 
        (((other.max >= self.min - 1) & (other.max <= self.max)) | ((other.min <= self.max + 1) & (other.min >= self.min))) { 
            Some(Range { min: min(self.min, other.min), max: max(self.max, other.max) })
        }
        else { None }
    }
}

pub fn read_txt(path: &Path) -> (Vec<(isize, isize)>, Vec<isize>) {
    let file_text = read_to_string(path).expect("Failed to read file");

    let mut ranges = vec![];
    let mut unknowns = vec![];

    let mut lines = file_text.lines();
    while let Some(line) = lines.next() {
        if line == "" { break };

        let coords = line.split('-').collect::<Vec<&str>>();
        ranges.push((coords[0].parse::<isize>().unwrap(), coords[1].parse::<isize>().unwrap()));
    }

    while let Some(line) = lines.next() {
        unknowns.push(line.parse::<isize>().unwrap());
    }

    (ranges, unknowns)
}

pub fn inspect_btree_for_insertion(tree: &BTreeSet<Range>, min: isize, max: isize) -> (Range, Vec<Range>) {
    let mut range = Range { min, max };
    let mut to_remove = vec![];

    for range_in_tree in tree.iter() {
        if let Some(new_range) = range.merge(range_in_tree) {
            // keep track of the ranges we have merged
            to_remove.push(range_in_tree.clone());
            range = new_range.clone();
        }
    }
    (range, to_remove)
}

pub fn to_b_tree_set(ranges: Vec<(isize, isize)>) -> BTreeSet<Range> {
    let mut tree_set = BTreeSet::new();
    for (min, max) in ranges {
        let (range_to_add, ranges_to_remove) = inspect_btree_for_insertion(&tree_set, min, max);
        
        // remove the ranges we have marked
        for remove_me in ranges_to_remove.into_iter() {
            tree_set.remove(&remove_me);
        }
        // now insert our new range 
        tree_set.insert(range_to_add);
    }
    tree_set
}

pub fn search_b_tree_set(tree: &BTreeSet<Range>, vals: Vec<isize>) -> isize {
    let mut count = 0;
    for val in vals {
        let mut last_max = 0;
        for range in tree.iter() {
            if (val >= range.min) & (val <= range.max) {
                count += 1;
                break; // stop if we've found our value in the range
            }
            // if we've passed the range the value would belong in, we stop searching
            if (val > last_max) & (val < range.min) { break }
            last_max = range.max;
        }
    }
    count
}

pub fn process_b_tree_set(path: &Path) -> anyhow::Result<usize> {
    let (ranges, values) = read_txt(path);
    let tree = to_b_tree_set(ranges);

    let count = search_b_tree_set(&tree, values);

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

        assert_eq!(3, process(input)?);
        Ok(())
    }

    #[test]
    fn test_proces_full() -> anyhow::Result<()> {
        let input = Path::new("./../inputs/day5.txt");

        assert_eq!(737, process(input)?);
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
    fn test_search_b_tree_set() {
        let ranges = vec![
            Range { min: 20, max: 30},
            Range { min: 6, max: 10 },
            Range { min: 12, max: 15},
            Range { min: 3, max: 5 },
            Range { min: 14, max: 15},
        ];

        let mut btree = BTreeSet::new();
        for range in ranges.into_iter() {
            btree.insert(range);
        }

        let vals = vec![3, 10, 11];

        let res = search_b_tree_set(&btree, vals);

        assert_eq!(res, 2)
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

    #[test]
    fn test_inspect_btree_empty() {
        let tree = BTreeSet::new();

        let (to_insert, to_remove) = inspect_btree_for_insertion(&tree, 0, 10);

        assert_eq!(to_insert, Range { min: 0, max: 10 });
        assert!(to_remove.is_empty());
    }

    #[test]
    fn test_inspect_btree_one_overlap() {
        let mut tree = BTreeSet::new();
        tree.insert(Range { min: 5, max: 10 });

        let (to_insert, to_remove) = inspect_btree_for_insertion(&tree, 0, 5);

        assert_eq!(to_insert, Range { min: 0, max: 10 });
        assert_eq!(to_remove, vec![Range { min: 5, max: 10 }]);
    }

    #[test]
    fn test_inspect_btree_one_overlap_multiple() {
        let mut tree = BTreeSet::new();
        tree.insert(Range { min: 30, max: 40 });
        tree.insert(Range { min: 5, max: 10 });
        tree.insert(Range { min: 0, max: 3 });
        tree.insert(Range { min: 15, max: 20 });

        let (to_insert, to_remove) = inspect_btree_for_insertion(&tree, 11, 15);

        assert_eq!(to_insert, Range { min: 5, max: 20 });
        assert_eq!(to_remove, vec![Range { min: 5, max: 10 }, Range { min: 15, max: 20 }]);
    }
}
