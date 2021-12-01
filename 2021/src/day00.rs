use aoc_runner_derive::{aoc, aoc_generator};

// Each day:
//  - Pick the proper typedef/name for `Foo` below
//  - Ctrl + F on dayN below
//  - Uncomment out part2's attribute macros
//  - Uncomment this module in lib.rs
type Foo = u64;

#[aoc_generator(dayN)]
pub fn parse_input(input: &str) -> Vec<Foo> {
    input
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect()
}

// Part1 ======================================================================
#[aoc(dayN, part1)]
pub fn part1(input: &[Foo]) -> Foo {
    unimplemented!();
}

// Part2 ======================================================================
// #[aoc(dayN, part2)]
pub fn part2(input: &[Foo]) -> Foo {
    unimplemented!();
}
