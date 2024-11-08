use aoc_runner_derive::{aoc, aoc_generator};

use itertools::Itertools;
use std::fmt;

#[derive(Copy, Clone, PartialEq)]
pub struct GroupYesCounts([u8; 26], u8);

impl GroupYesCounts {
    fn new() -> Self {
        GroupYesCounts([0_u8; 26], 0)
    }

    fn persons(&self) -> u8 {
        self.1
    }

    fn add_persons_answers(&mut self, s: &str) {
        self.1 += 1;
        for c in s.chars() {
            debug_assert!(c.is_ascii_lowercase());
            let i = (c as u8 - b'a') as usize;
            self.0[i] += 1;
        }
    }

    fn anyone_yes(&self) -> usize {
        self.0.iter().map(|b| (*b != 0) as usize).sum()
    }

    fn everyone_yes(&self) -> usize {
        self.0.iter().map(|b| (*b == self.1) as usize).sum()
    }
}

impl fmt::Display for GroupYesCounts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut d = f.debug_struct("GroupYesCounts");

        d.field("persons", &self.persons());

        for (b, count) in self.0.iter().copied().enumerate() {
            if count > 0 {
                let buf: [u8; 1] = [b as u8 + b'a'];
                d.field(std::str::from_utf8(&buf).unwrap(), &count);
            }
        }

        d.finish()
    }
}

impl fmt::Debug for GroupYesCounts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

#[aoc_generator(day6)]
pub fn parse_input(input: &str) -> Vec<GroupYesCounts> {
    input
        .trim()
        .lines()
        .chunk_by(|l| l.trim().is_empty())
        .into_iter()
        .filter_map(|(is_empty, lines)| if is_empty { None } else { Some(lines) })
        .map(|lines| {
            let mut g = GroupYesCounts::new();

            for line in lines {
                g.add_persons_answers(line);
            }

            g
        })
        .collect()
}

#[test]
fn check_ex() {
    let input = r#"
abc

a
b
c

ab
ac

a
a
a
a

b
    "#;
    let expected_counts = [
        {
            let mut g = GroupYesCounts::new();
            g.add_persons_answers("abc");
            g
        },
        {
            let mut g = GroupYesCounts::new();
            g.add_persons_answers("a");
            g.add_persons_answers("b");
            g.add_persons_answers("c");
            g
        },
        {
            let mut g = GroupYesCounts::new();
            g.add_persons_answers("ab");
            g.add_persons_answers("ac");
            g
        },
        {
            let mut g = GroupYesCounts::new();
            g.add_persons_answers("a");
            g.add_persons_answers("a");
            g.add_persons_answers("a");
            g.add_persons_answers("a");
            g
        },
        {
            let mut g = GroupYesCounts::new();
            g.add_persons_answers("b");
            g
        },
    ];

    let group_counts = parse_input(input);
    assert_eq!(group_counts, expected_counts);

    assert_eq!(part1(&group_counts), 11);
    assert_eq!(part2(&group_counts), 6);
}

// Part1 ======================================================================
#[aoc(day6, part1)]
pub fn part1(input: &[GroupYesCounts]) -> usize {
    input.iter().map(|g: &GroupYesCounts| g.anyone_yes()).sum()
}

// Part2 ======================================================================
#[aoc(day6, part2)]
pub fn part2(input: &[GroupYesCounts]) -> usize {
    input
        .iter()
        .map(|g: &GroupYesCounts| g.everyone_yes())
        .sum()
}
