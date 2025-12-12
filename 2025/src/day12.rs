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
            ((nums[0] / 3) * (nums[1] / 3)) > (nums[2] + nums[3] + nums[4] + nums[5] + nums[6])
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
