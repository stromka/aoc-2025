use daily_template::part1::process as process_part1;
use daily_template::part2::process as process_part2;
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
        let res = process_part1(&Path::new("../inputs/day5.txt"));
    } else if args.part == 2 {
        let res = process_part2(&Path::new("../inputs/day5.txt"));
    }
}
