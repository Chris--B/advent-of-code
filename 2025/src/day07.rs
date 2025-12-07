#![allow(unused)]

use crate::prelude::*;

// Part1 ========================================================================
#[aoc(day7, part1)]
pub fn part1(input: &str) -> i64 {
    let mut map = Framebuffer::parse_grid_char(input);
    let mut beams = Bitset256::new();
    beams.insert(map.width() as u32 / 2);

    let mut splits = 0;

    for y in map.range_y() {
        let y = map.height() as i32 - y - 1;

        let mut next = Bitset256::new();
        for x in map.range_x() {
            let x = x as i64;
            if beams.contains(x) {
                if map[(x as i32, y)] == '^' {
                    splits += 1;
                    // println!("Found splitter at ({x}, {y})");
                    next.insert(x - 1);
                    next.insert(x + 1);
                } else {
                    next.insert(x);
                }
            }
        }
        beams = next;
    }

    splits
}

// Part2 ========================================================================
#[aoc(day7, part2)]
pub fn part2(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

    #[rstest]
    #[case::given(21, EXAMPLE_INPUT)]
    #[timeout(Duration::from_millis(1))]
    #[trace]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        init_logging();

        let input = input.trim();
        assert_eq!(p(input), expected);
    }

    #[rstest]
    #[case::given(999_999, EXAMPLE_INPUT)]
    #[ignore]
    #[trace]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        init_logging();

        let input = input.trim();
        assert_eq!(p(input), expected);
    }
}
