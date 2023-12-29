use crate::prelude::*;

// use core_simd::*;

// Part1 ========================================================================
#[aoc(day11, part1, simd)]
pub fn part1(input: &str) -> i64 {
    let input = input.as_bytes();

    let mut points = [(0_i32, 0_i32); 512];

    {
        let mut pos: usize = 0;
        let mut count: usize = 0;

        while pos < input.len() {
            let is_galaxy = input[pos] == b'#';

            if is_galaxy {
                let id = pos;
                let x = id % 141;
                let y = id / 141;

                dbg!((x, y));
                points[x].0 += 1;
                points[y].1 += 1;
                count += 1;
            }

            pos += 1;
        }
    }

    // dbg!(&points[..10]);

    0
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

    #[rstest]
    #[case::given(374, EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(input), expected);
    }
}
