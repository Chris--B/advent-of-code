#![allow(unused)]

use crate::prelude::*;

// Part1 ========================================================================
#[aoc(day12, part1)]
pub fn part1(input: &str) -> i64 {
    let mut input = input.as_bytes();
    let mut s = 0;
    memchr_iter(b'\n', input)
        .skip(30)
        .filter(|&i| {
            let line = &input[s..i];
            s = i + 1;
            let nums: SmallVec<[i64; 7]> = line.i64s().collect();

            let a = (nums[0] / 3) * (nums[1] / 3);
            let b = nums[2] + nums[3] + nums[4] + nums[5] + nums[6];
            a > b
        })
        .count() as i64
}

#[aoc(day12, part1, hard_coded)]
pub fn part1_hard_coded(input: &str) -> i64 {
    fn num_at(bytes: &[u8], i: usize) -> i64 {
        debug_assert!(
            bytes[i].is_ascii_digit(),
            "bytes[{i}] == {}, not a digit",
            bytes[i] as char
        );
        debug_assert!(
            bytes[i + 1].is_ascii_digit(),
            "bytes[{}] == {}, not a digit",
            i + 1,
            bytes[i + 1] as char
        );

        10 * (bytes[i] - b'0') as i64 + (bytes[i + 1] - b'0') as i64
    }

    let mut input = &input.as_bytes()[96..];
    let mut s = 0;
    memchr_iter(b'\n', input)
        .filter(|&i| {
            let line = &input[s..i];
            s = i + 1;

            // "36x45: 40 40 46 51 34 42"
            let width = num_at(line, 0);
            let height = num_at(line, 3);

            let a = ((width / 3) * (height / 3));
            let b = num_at(line, 7)
                + num_at(line, 10)
                + num_at(line, 13)
                + num_at(line, 16)
                + num_at(line, 19);
            // + num_at(line, 22);
            a > b
        })
        .count() as i64
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
";

    // #[rstest]
    // #[case::given(2, EXAMPLE_INPUT)]
    // #[trace]
    // #[timeout(Duration::from_millis(1))]
    // fn check_ex_part_1(
    //     #[notrace]
    //     #[values(part1)]
    //     p: impl FnOnce(&str) -> i64,
    //     #[case] expected: i64,
    //     #[case] input: &str,
    // ) {
    //     init_logging();

    //     let input = input.trim();
    //     assert_eq!(p(input), expected);
    // }
}
