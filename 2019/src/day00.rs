use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(dayN)]
pub fn parse_input(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect()
}

#[aoc(dayN, part1)]
pub fn part1(input: &[u32]) -> u32 {
    not_implemented!();
}
