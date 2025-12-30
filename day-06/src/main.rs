use day_06::part1::process as process_part1;
use day_06::part2::process as process_part2;
use std::path::Path;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    part: usize,
}

fn main() {
    let args = Args::parse();

    if args.part == 1 {
        let res = process_part1(&Path::new("../inputs/day6.txt"));
        assert_eq!(res.unwrap(), 6605396225322);
    } else if args.part == 2 {
        let res = process_part2(&Path::new("../inputs/day6.txt"));
        assert_eq!(res.unwrap(), 11052310600986);
    }
}
