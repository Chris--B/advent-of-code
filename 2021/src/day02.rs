use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Copy, Clone, Debug)]
pub enum Cmd {
    Forward,
    Down,
    Up,
}
use Cmd::*;

#[aoc_generator(day2)]
pub fn parse_input(input: &str) -> Vec<(Cmd, u64)> {
    input
        .lines()
        .map(|line| {
            let mut iter = line.trim().split(' ');
            let dir = iter.next().unwrap();
            let amount = iter.next().unwrap();

            match (dir, amount) {
                ("forward", x) => (Forward, x.parse().unwrap()),
                ("down", x) => (Down, x.parse().unwrap()),
                ("up", x) => (Up, x.parse().unwrap()),
                _ => panic!(),
            }
        })
        .collect()
}

// Part1 ======================================================================
#[aoc(day2, part1)]
#[inline(never)]
pub fn part1(input: &[(Cmd, u64)]) -> u64 {
    let (mut x, mut y) = (0, 0);

    for (cmd, spd) in input {
        match cmd {
            Forward => x += spd,
            Down => y += spd,
            Up => y -= spd,
        }
    }

    x * y
}

// Part2 ======================================================================
#[aoc(day2, part2)]
#[inline(never)]
pub fn part2(input: &[(Cmd, u64)]) -> u64 {
    let (mut x, mut y) = (0, 0);
    let mut aim = 0;

    for (cmd, spd) in input {
        match cmd {
            Down => aim += spd,
            Up => aim -= spd,
            /*
            forward X does two things:

            It increases your horizontal position by X units.
            It increases your depth by your aim multiplied by X.
            */
            Forward => {
                x += spd;
                y += aim * spd;
            }
        }
    }

    x * y
}
