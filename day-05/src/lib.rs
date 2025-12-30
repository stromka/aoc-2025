pub mod part1;
pub mod part2;

use std::cmp::{Ordering, PartialEq, PartialOrd, min, max};

#[derive(Hash, PartialEq, Eq, Debug, Ord, Clone, Copy)]
pub struct Range {
    pub min: isize,
    pub max: isize,
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
    pub fn merge(&self, other: &Self) -> Option<Range> {
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