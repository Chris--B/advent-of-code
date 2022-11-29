use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn parse_input(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect()
}

// Part1 ========================================================================
#[aoc(day1, part1)]
#[inline(never)]
pub fn part1(_input: &[i64]) -> i64 {
    unimplemented!();
}

// Part2 ========================================================================
#[aoc(day1, part2)]
#[inline(never)]
pub fn part2(_input: &[i64]) -> i64 {
    unimplemented!();
}
