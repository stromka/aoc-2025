use day_05::part1::process;
use std::path::Path;

fn main() {
    let res = process(&Path::new("../inputs/day5.txt"));

    assert_eq!(res.unwrap(), 737);
}
