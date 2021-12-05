use aoc_runner_derive::{aoc, aoc_generator};

use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
pub struct Line {
    x0: i64,
    y0: i64,

    x1: i64,
    y1: i64,
}

impl Line {
    fn points(&self) -> impl Iterator<Item = (i64, i64)> {
        #![allow(non_snake_case)]

        let Line { x0, y0, x1, y1 } = *self;

        let mut v = vec![];

        let dx = x1 - x0;
        let dy = y1 - y0;

        if dx == 0 && dy != 0 {
            let y = i64::min(y0, y1);
            let Y = i64::max(y0, y1);
            v.extend((y..=Y).map(|y| (x0, y)));
        } else if dx != 0 && dy == 0 {
            let x = i64::min(x0, x1);
            let X = i64::max(x0, x1);
            v.extend((x..=X).map(|x| (x, y0)));
        } else if dx.abs() == dy.abs() {
            assert!(dx != 0);

            let steps = (y1 - y0).abs();

            let dir_x = if x1 > x0 { 1 } else { -1 };
            let dir_y = if y1 > y0 { 1 } else { -1 };

            v.extend((0..=steps).map(|i| (x0 + dir_x * i, y0 + dir_y * i)));
        } else {
            dbg!(*self);
            panic!("no")
        }

        v.into_iter()
    }
}

#[test]
fn check_line_points_p1_1() {
    let line: Line = "1,1 -> 1,3".parse().unwrap();
    let mut points: Vec<_> = line.points().collect();
    points.sort_unstable();

    assert_eq!(points, [(1, 1), (1, 2), (1, 3)]);
}

#[test]
fn check_line_points_p1_2() {
    let line: Line = "9,7 -> 7,7".parse().unwrap();
    let mut points: Vec<_> = line.points().collect();
    points.sort_unstable();

    assert_eq!(points, [(7, 7), (8, 7), (9, 7)]);
}

#[test]
fn check_line_points_p2_1() {
    let line: Line = "1,1 -> 3,3".parse().unwrap();
    let mut points: Vec<_> = line.points().collect();
    points.sort_unstable();

    assert_eq!(points, [(1, 1), (2, 2), (3, 3)]);
}

#[test]
fn check_line_points_p2_2() {
    let line: Line = "9,7 -> 7,9".parse().unwrap();
    let mut points: Vec<_> = line.points().collect();
    points.sort_unstable();

    assert_eq!(points, [(7, 9), (8, 8), (9, 7)]);
}

#[test]
fn check_line_points_p2_3() {
    let line: Line = "7,9 -> 9,7".parse().unwrap();
    let mut points: Vec<_> = line.points().collect();
    points.sort_unstable();

    assert_eq!(points, [(7, 9), (8, 8), (9, 7)]);
}

impl std::str::FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(',');

        // 5,5 -> 8,2
        let x0 = it.next().unwrap();
        let y0x1 = it.next().unwrap();
        let mut it2 = y0x1.split("->");
        let y0 = it2.next().unwrap();
        let x1 = it2.next().unwrap();
        let y1 = it.next().unwrap();

        Ok(Line {
            x0: x0.trim().parse().unwrap(),
            y0: y0.trim().parse().unwrap(),

            x1: x1.trim().parse().unwrap(),
            y1: y1.trim().parse().unwrap(),
        })
    }
}

#[aoc_generator(day5)]
pub fn parse_input(input: &str) -> Vec<Line> {
    input
        .trim()
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect()
}

// Part1 ======================================================================
#[aoc(day5, part1)]
#[inline(never)]
pub fn part1(lines: &[Line]) -> usize {
    let mut map = HashMap::<(i64, i64), u32>::new();

    for line in lines.iter().copied() {
        let Line { x0, y0, x1, y1 } = line;

        if x0 == x1 || y0 == y1 {
            for (x, y) in line.points() {
                *map.entry((x, y)).or_insert(0) += 1;
            }
        }
    }

    map.iter().filter(|(p, c)| **c >= 2).count()
}

// Part2 ======================================================================
#[aoc(day5, part2)]
#[inline(never)]
pub fn part2(lines: &[Line]) -> usize {
    let mut map = HashMap::<(i64, i64), u32>::new();

    for line in lines.iter().copied() {
        let Line { x0, y0, x1, y1 } = line;

        for (x, y) in line.points() {
            *map.entry((x, y)).or_insert(0) += 1;
        }
    }

    map.iter().filter(|(p, c)| **c >= 2).count()
}
