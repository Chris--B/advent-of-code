use aoc_runner_derive::{aoc, aoc_generator};

use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
pub enum Cmd {
    Forward(u64),
    Down(u64),
    Up(u64),
}
use Cmd::*;

impl FromStr for Cmd {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        let (dir, spd) = s.split_once(' ').unwrap();
        let spd = spd.trim().parse().unwrap();
        let cmd = match dir {
            "forward" => Forward(spd),
            "down" => Down(spd),
            "up" => Up(spd),
            _ => panic!("Unrecognized direction"),
        };

        Ok(cmd)
    }
}

#[aoc_generator(day2)]
pub fn parse_input(input: &str) -> Vec<Cmd> {
    input
        .lines()
        .map(str::parse)
        .collect::<Result<_, ()>>()
        .unwrap()
}

// Part1 ======================================================================
#[aoc(day2, part1)]
#[inline(never)]
pub fn part1(input: &[Cmd]) -> u64 {
    // x is horizontal distance
    // y is DEPTH, and is kind of backwards (except for graphics programmers)
    let (mut x, mut y) = (0, 0);

    for cmd in input {
        match cmd {
            Forward(spd) => x += spd,
            Down(spd) => y += spd,
            Up(spd) => y -= spd,
        }
    }

    x * y
}

// Part2 ======================================================================
#[aoc(day2, part2)]
#[inline(never)]
pub fn part2(input: &[Cmd]) -> u64 {
    // x is horizontal distance
    // y is DEPTH
    let (mut x, mut y) = (0, 0);
    let mut aim = 0;

    for cmd in input {
        match cmd {
            Forward(spd) => {
                x += spd;
                y += aim * spd;
            }
            Down(spd) => aim += spd,
            Up(spd) => aim -= spd,
        }
    }

    x * y
}
