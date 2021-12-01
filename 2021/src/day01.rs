use aoc_runner_derive::{aoc, aoc_generator};

type Foo = usize;

#[aoc_generator(day1)]
pub fn parse_input(input: &str) -> Vec<Foo> {
    input
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect()
}

// Part1 ======================================================================
#[aoc(day1, part1)]
pub fn part1(_input: &[Foo]) -> Foo {
    unimplemented!();
}

// Part2 ======================================================================
// #[aoc(day1, part2)]
pub fn part2(_input: &[Foo]) -> Foo {
    unimplemented!();
}
