#![allow(unused)]

use indicatif::ProgressBar;

use crate::prelude::*;

fn steps_to(start: IVec2, end: IVec2, map: &Framebuffer<char>) -> Option<i32> {
    let dim = map.width();

    let mut cost_map: Framebuffer<i32> = Framebuffer::new_matching_size(map);
    cost_map.clear(i32::MAX);
    cost_map[start] = 0;

    let mut queue = vec![start];
    while let Some(curr) = queue.pop() {
        if curr == end {
            continue;
        }

        let curr_cost = cost_map[curr];
        for dir in Cardinal::ALL_NO_DIAG {
            let next = curr + dir.into();
            let next_cost = curr_cost + 1;

            if let Some(&cost) = cost_map.get(next.x as _, next.y as _) {
                if (map[next] == '.') && (next_cost < cost) {
                    // Better deal
                    cost_map[next] = next_cost;
                    queue.push(next);
                }
            }
        }
    }

    if cost_map[end] == i32::MAX {
        None
    } else {
        Some(cost_map[end])
    }
}

// Part1 ========================================================================
#[aoc(day18, part1)]
pub fn part1(input: &str) -> i32 {
    let dim = if cfg!(test) { 6 } else { 70 } + 1;
    let first_n = if cfg!(test) { 12 } else { 1024 };

    let mut map = Framebuffer::new(dim as u32, dim as u32);
    map.clear('.');

    if cfg!(test) {
        println!("After the corruption:");
        map.just_print();
    }

    for (x, y) in input.i64s().map(|n| n as i32).tuples().take(first_n) {
        let y = dim - y - 1;
        map[(x, y)] = '#';
    }

    let start = IVec2::new(0, dim - 1);
    let end = IVec2::new(dim - 1, 0);

    steps_to(start, end, &map).expect("No path found?")
}

// Part2 ========================================================================
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Coord((i32, i32));

impl std::fmt::Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

#[aoc(day18, part2, bruteforce)]
pub fn part2_bruteforce(input: &str) -> Coord {
    println!("Running day18 part2()");

    if !cfg!(test) {
        println!("  jk, comment out this to actually run it");
        return Coord((-1, -1));
    }

    let dim = if cfg!(test) { 6 } else { 70 } + 1;

    let mut map = Framebuffer::new(dim as u32, dim as u32);
    map.clear('.');

    let start = IVec2::new(0, dim - 1);
    let end = IVec2::new(dim - 1, 0);

    let pb = ProgressBar::new(input.i64s().count() as u64 / 2);
    for (x, y) in input.i64s().map(|n| n as i32).tuples() {
        let y = dim - y - 1;
        map[(x, y)] = '#';

        let maybe_steps = steps_to(start, end, &map);
        pb.inc(1);

        if maybe_steps.is_none() {
            pb.finish();
            let y = dim - y - 1;
            return Coord((x, y));
        }
    }
    pb.finish();

    unreachable!()
}

#[aoc(day18, part2, rayon)]
pub fn part2_rayon(input: &str) -> Coord {
    use rayon::iter::{IntoParallelIterator, ParallelIterator};
    println!("Running day18 part2_rayon()");

    let dim = if cfg!(test) { 6 } else { 70 } + 1;

    let mut map = Framebuffer::new(dim as u32, dim as u32);
    map.clear('.');

    let start = IVec2::new(0, dim - 1);
    let end = IVec2::new(dim - 1, 0);

    let coords: Vec<_> = input.i64s().map(|n| n as i32).tuples().collect_vec();
    let counts: Vec<_> = (1..=coords.len()).collect_vec();

    let pb = ProgressBar::new(counts.len() as u64);
    let answer: Vec<(usize, Coord)> = counts
        .into_par_iter()
        .filter_map(|n| {
            let mut map = map.clone();
            for &(x, y) in &coords[..n] {
                map[(x, dim - y - 1)] = '#';
            }

            let maybe_steps = steps_to(start, end, &map);
            pb.inc(1);

            if maybe_steps.is_none() {
                Some((n, Coord(coords[n - 1])))
            } else {
                None
            }
        })
        .collect();
    pb.finish();

    let (_, coord) = answer.into_iter().min_by_key(|&(n, _)| n).unwrap();
    coord
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";

    #[rstest]
    #[case::given(22, EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1)]
        p: impl FnOnce(&str) -> i32,
        #[case] expected: i32,
        #[case] input: &str,
    ) {
        init_logging();

        let input = input.trim();
        assert_eq!(p(input), expected);
    }

    #[rstest]
    #[case::given((6,1), EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2_bruteforce, part2_rayon)]
        p: impl FnOnce(&str) -> Coord,
        #[case] expected: (i32, i32),
        #[case] input: &str,
    ) {
        init_logging();

        let input = input.trim();
        assert_eq!(p(input), Coord(expected));
    }
}
