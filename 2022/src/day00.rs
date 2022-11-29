use aoc_runner_derive::{aoc, aoc_generator};

// Each day:
//  - Ctrl + F on dayN below
//  - Uncomment this module in lib.rs

#[aoc_generator(dayN)]
pub fn parse_input(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect()
}

// Part1 ========================================================================
#[aoc(dayN, part1)]
#[inline(never)]
pub fn part1(input: &[i64]) -> i64 {
    unimplemented!();
}

// Part2 ========================================================================
#[aoc(dayN, part2)]
#[inline(never)]
pub fn part2(input: &[i64]) -> i64 {
    unimplemented!();
}
