#![allow(unused)]

use crate::{framebuffer::ParsingInfo, prelude::*};

pub struct Map {
    heights: Framebuffer<u8>,
    zeroes: Vec<IVec2>,
}

#[aoc_generator(day10)]
fn parse(input: &str) -> Map {
    let mut zeroes = vec![];
    let mut heights = Framebuffer::parse_grid2(input, |ParsingInfo { c, x, y, .. }| {
        if c == '.' {
            99
        } else if c == '0' {
            zeroes.push(IVec2::new(x, y));
            0
        } else {
            c as u8 - b'0'
        }
    });

    heights.set_border_color(Some(99));

    Map { heights, zeroes }
}

fn find_peaks(map: &Framebuffer<u8>, trailhead: IVec2) -> Vec<IVec2> {
    let mut peaks = vec![];

    let mut queue = VecDeque::from([trailhead]);
    while let Some(curr) = queue.pop_front() {
        if map[curr] == 9 {
            if !peaks.contains(&curr) {
                peaks.push(curr);
            }
            continue;
        }

        for next in curr.neighbors() {
            if map[next] < 10 && (map[next] == (map[curr] + 1)) {
                queue.push_back(next);
            }
        }
    }

    peaks
}

// Part1 ========================================================================
#[aoc(day10, part1)]
pub fn part1(map: &Map) -> i64 {
    map.zeroes
        .iter()
        .map(|&z| find_peaks(&map.heights, z).len() as i64)
        .sum()
}

// Part2 ========================================================================
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Path {
    history: SmallVec<[IVec2; 16]>,
}

impl Path {
    fn new(trailhead: IVec2) -> Self {
        Self {
            history: smallvec![trailhead],
        }
    }

    fn push(&self, next: IVec2) -> Self {
        let mut history = self.history.clone();
        history.push(next);

        Self { history }
    }

    fn last(&self) -> IVec2 {
        *self.history.last().unwrap()
    }
}

fn find_paths(map: &Framebuffer<u8>, trailhead: IVec2) -> Vec<Path> {
    // Map from Peaks to Paths that reach that peak from `trailhead`
    let mut paths: Vec<Path> = Vec::new();

    let mut queue = VecDeque::from([Path::new(trailhead)]);
    while let Some(curr) = queue.pop_front() {
        if map[curr.last()] == 9 {
            if !paths.contains(&curr) {
                paths.push(curr);
            }
            continue;
        }

        for next in curr.last().neighbors() {
            if map[next] < 10 && (map[next] == (map[curr.last()] + 1)) {
                queue.push_back(curr.push(next));
            }
        }
    }

    paths
}

#[aoc(day10, part2)]
pub fn part2(map: &Map) -> i64 {
    map.zeroes
        .iter()
        .map(|&z| find_paths(&map.heights, z).len() as i64)
        .sum()
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_JUST_2: &str = r"
...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9
";

    const EXAMPLE_JUST_4: &str = r"
..90..9
...1.98
...2..7
6543456
765.987
876....
987....
";

    const EXAMPLE_JUST_9: &str = r"
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

    #[rstest]
    #[case::given_2(2, EXAMPLE_JUST_2)]
    #[case::given_4(4, EXAMPLE_JUST_4)]
    #[case::given_9(36, EXAMPLE_JUST_9)]
    #[timeout(Duration::from_millis(100))]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1)]
        p: impl FnOnce(&Map) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        init_logging();

        let input = input.trim();
        let input = parse(input);
        input
            .heights
            .print(|x, y, &h| if h > 9 { '.' } else { (h + b'0') as char });

        assert_eq!(p(&input), expected);
    }

    const EXAMPLE_P2_RATING_3: &str = r"
.....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9....
";

    const EXAMPLE_P2_RATING_13: &str = r"
..90..9
...1.98
...2..7
6543456
765.987
876....
987....
";

    const EXAMPLE_P2_RATING_227: &str = r"
012345
123456
234567
345678
4.6789
56789.
";

    #[rstest]
    #[case::given(3, EXAMPLE_P2_RATING_3)]
    #[case::given(13, EXAMPLE_P2_RATING_13)]
    #[case::given(227, EXAMPLE_P2_RATING_227)]
    #[case::given(20+24+10+4+1+4+5+8+5, EXAMPLE_JUST_9)]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2)]
        p: impl FnOnce(&Map) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        init_logging();

        let input = input.trim();
        let input = parse(input);
        input
            .heights
            .print(|x, y, &h| if h > 9 { '.' } else { (h + b'0') as char });

        assert_eq!(p(&input), expected);
    }
}
