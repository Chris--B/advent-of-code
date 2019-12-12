use aoc_runner_derive::{aoc, aoc_generator};
use intcode::vm::Vm;

#[aoc_generator(dayN)]
pub fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect()
}

#[aoc(dayN, part1)]
pub fn part1(input: &[i32]) -> i32 {
    unimplemented!();
}
