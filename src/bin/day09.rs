use advent_of_code::*;
use eyre::Result;
use std::io::BufRead;

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let handle = stdin.lock();

    let rules = parse_file(handle.lines().filter_map(|l| l.ok()))?;

    let preamble = 5;
    let result1 = part1_b(&rules, preamble).unwrap();
    println!("Part 1: {} ", result1);

    let result = part2(&rules, result1)?;
    println!("Part 2: {}", result);
    Ok(())
}
