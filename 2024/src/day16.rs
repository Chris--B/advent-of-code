#![allow(unused)]

use crate::prelude::*;

// Part1 ========================================================================
#[aoc(day16, part1)]
pub fn part1(input: &str) -> i32 {
    let mut start = IVec2::zero();
    let mut end = IVec2::zero();
    let map = Framebuffer::parse_grid2(input, |ParsingInfo { c, x, y }| match c {
        '#' | '.' => c,
        'S' => {
            start = IVec2::new(x, y);
            '.'
        }
        'E' => {
            end = IVec2::new(x, y);
            '.'
        }
        _ => unreachable!("Unrecognized map character: {c:?}"),
    });
    // map.just_print();
    // println!("Moving from {:?} -> {:?}", start.as_array(), end.as_array());

    let mut state: Framebuffer<i32> = Framebuffer::new_matching_size(&map);
    state.clear(i32::MAX);
    state[start] = 0;

    let mut queue = vec![(start, East)];
    while let Some((curr, curr_dir)) = queue.pop() {
        // println!("Exploring {:?} going {curr_dir:?}", curr.as_array());
        for dir in Cardinal::ALL_NO_DIAG {
            let next = curr + dir.into();
            if map[next] == '#' {
                continue;
            }

            let cost = state[curr] + if curr_dir == dir { 1 } else { 1001 };
            if state[next] > cost {
                // We found a better deal
                state[next] = cost;
                queue.push((next, dir));
            }
        }
    }

    state[end]
}

// Part2 ========================================================================
#[aoc(day16, part2)]
pub fn part2(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT_1: &str = r"
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############

";

    const EXAMPLE_INPUT_2: &str = r"
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";

    #[rstest]
    #[case::given_1(7_036, EXAMPLE_INPUT_1)]
    #[case::given_2(11_048, EXAMPLE_INPUT_2)]
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
    #[case::given_1(45, EXAMPLE_INPUT_1)]
    #[case::given_2(64, EXAMPLE_INPUT_2)]
    #[timeout(Duration::from_millis(750))]
    #[ignore]
    #[trace]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2)]
        p: impl FnOnce(&str) -> i32,
        #[case] expected: i32,
        #[case] input: &str,
    ) {
        init_logging();

        let input = input.trim();
        assert_eq!(p(input), expected);
    }
}
