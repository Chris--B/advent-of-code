use aoc_runner_derive::{aoc, aoc_generator};

use smallvec::{smallvec, SmallVec};

use core::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fmt;

#[aoc_generator(day15)]
pub fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.trim().chars().map(|c| c as u8 - b'0').collect())
        .collect()
}

#[derive(Clone, PartialEq, Eq)]
struct Path(SmallVec<[(u8, isize, isize); 200]>);

impl Path {
    fn last(&self) -> (u8, isize, isize) {
        // Our Path object is never empty
        *self.0.last().unwrap()
    }

    fn visted(&self, x: usize, y: usize) -> bool {
        let (x, y) = (x as isize, y as isize);
        self.0.iter().any(|(_r, xx, yy)| (*xx == x) && (*yy == y))
    }

    fn push(&mut self, risk: u8, x: usize, y: usize) {
        self.0.push((risk, x as isize, y as isize))
    }

    fn len(&self) -> isize {
        self.0.len() as isize
    }

    fn risk(&self) -> i64 {
        assert_eq!(self.0[0], (1, 0, 0));
        // First entry is skipped
        self.0.iter().skip(1).map(|(r, _, _)| *r as i64).sum()
    }
}

impl fmt::Debug for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let points: Vec<_> = self.0.iter().map(|(r, _x, _y)| r).collect();

        f.debug_struct("Path")
            .field("risk", &self.risk())
            .field("len", &self.len())
            .field("path", &format!("{:?}", points))
            .finish()
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        self.risk().cmp(&other.risk())
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Part1 ======================================================================
#[aoc(day15, part1)]
#[inline(never)]
pub fn part1(input: &[Vec<u8>]) -> i64 {
    let max_x = (input.len() - 1) as isize;
    let max_y = (input[0].len() - 1) as isize;

    let mut paths = BinaryHeap::new();
    paths.push(Path(smallvec![(input[0][0], 0, 0)]));

    let mut shortest: Option<Path> = None;

    let mut times = 0;
    while let Some(path) = paths.pop() {
        times += 1;
        if times % 5_000 == 0 {
            println!("[{times:>10}] {} paths in queue", paths.len());
            // println!("Looking at: {:?}", path.0);
        }

        let mut grew = false;

        let (_, x, y) = path.last();
        for (dx, dy) in [
            // No diagonals
            (1, 0),
            (0, 1),
            (-1, 0),
            (0, -1),
        ] {
            // TODO: Are x and y flipped here...?
            let x = (x + dx) as usize;
            let y = (y + dy) as usize;

            if (x < input.len()) && (y < input[0].len()) && !path.visted(x, y) {
                grew = true;

                let mut new = path.clone();
                new.push(input[x][y], x, y);

                // If we have a shortest, we can skip anything that gets too long
                if let Some(ref s) = shortest {
                    if new.risk() < s.risk() {
                        paths.push(new);
                    }
                } else {
                    // If we don't have a shortest, save this unconditionally
                    paths.push(new);
                }
            }
        }

        // If it didn't grow, then it reached an end.
        if !grew {
            // If it finished in the bottom right, we're good
            let (_, x, y) = path.last();

            if (x == max_x) && (y == max_y) {
                if let Some(ref s) = shortest {
                    if s.risk() > path.risk() {
                        println!("Updating shortest: {:?}", path);
                        shortest = Some(path);
                    }
                } else {
                    println!("Found first shortest: {:?}", path);
                    shortest = Some(path);
                }
            } else {
                // Otherwise it might have gotten stuck and we ignore it
                dbg!(path);
            }
        }
    }

    dbg!(times);
    let shortest = shortest.unwrap();

    dbg!(&shortest);
    shortest.risk()
}

// Part2 ======================================================================
// #[aoc(day15, part2)]
// #[inline(never)]
// pub fn part2(input: &[i64]) -> i64 {
//     unimplemented!();
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_example_1() {
        let input = r"
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
"
        .trim();
        assert_eq!(part1(&parse_input(input)), 40);
    }
}
