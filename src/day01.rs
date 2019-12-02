use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn parse_masses(input: &str) -> Vec<u32> {
    input
        .split_whitespace()
        .map(|line| line.trim().parse::<u32>().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn simple(input: &[u32]) -> u32 {
    input.iter().map(|m| m / 3 - 2).sum()
}
