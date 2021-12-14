use aoc_runner_derive::{aoc, aoc_generator};

use std::collections::HashMap;

use crate::framebuffer::Framebuffer;

#[derive(Copy, Clone, Debug)]
pub struct Line {
    x0: i64,
    y0: i64,

    x1: i64,
    y1: i64,
}

impl Line {
    fn points(&self) -> impl Iterator<Item = (i64, i64)> {
        LineIter::new(*self)
    }
}

struct LineIter {
    line: Line,

    curr_x: i64,
    dir_x: i64,

    curr_y: i64,
    dir_y: i64,
}

impl LineIter {
    fn new(line: Line) -> Self {
        let dir_x = if line.x1 > line.x0 { 1 } else { -1 };
        let dir_y = if line.y1 > line.y0 { 1 } else { -1 };

        LineIter {
            line,
            // offset here so the first .next() call produces (x0, y0)
            curr_x: line.x0 - dir_x,
            dir_x,
            // offset here so the first .next() call produces (x0, y0)
            curr_y: line.y0 - dir_y,
            dir_y,
        }
    }
}

impl Iterator for LineIter {
    type Item = (i64, i64);

    fn next(&mut self) -> Option<Self::Item> {
        let mut done = true;

        if self.curr_x != self.line.x1 {
            done = false;
            self.curr_x += self.dir_x;
        }

        if self.curr_y != self.line.y1 {
            done = false;
            self.curr_y += self.dir_y;
        }

        if done {
            None
        } else {
            Some((self.curr_x, self.curr_y))
        }
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
#[aoc(day5, part1, hashmap)]
#[inline(never)]
pub fn part1_hashmap(lines: &[Line]) -> usize {
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
#[aoc(day5, part2, hashmap)]
#[inline(never)]
pub fn part2_hashmap(lines: &[Line]) -> usize {
    let mut map = HashMap::<(i64, i64), u32>::new();

    for line in lines.iter().copied() {
        let Line { x0, y0, x1, y1 } = line;

        for (x, y) in line.points() {
            *map.entry((x, y)).or_insert(0) += 1;
        }
    }

    map.iter().filter(|(_p, c)| **c >= 2).count()
}

// Part2 ======================================================================
#[aoc(day5, part2, framebuffer)]
#[inline(never)]
pub fn part2_framebuffer(lines: &[Line]) -> usize {
    let max_x = lines.iter().map(|line| line.x0.max(line.x1)).max().unwrap();
    let max_x = 1 + max_x as usize;

    let max_y = lines.iter().map(|line| line.y0.max(line.y1)).max().unwrap();
    let max_y = 1 + max_y as usize;

    let mut fb: Framebuffer<usize> = Framebuffer::with_dims(max_x, max_y);

    for line in lines.iter().copied() {
        let Line { x0, y0, x1, y1 } = line;

        for (x, y) in line.points() {
            fb[(x as usize, y as usize)] += 1;
        }
    }

    fb.flatten().filter(|c| **c >= 2).count()
}
