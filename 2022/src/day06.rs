use aoc_runner_derive::aoc;

use itertools::Itertools;

// Part1 ========================================================================
#[aoc(day6, part1)]
#[inline(never)]
pub fn part1(input: &str) -> usize {
    let input = input.as_bytes();
    for i in 0..(input.len()) {
        let xs = &input[i..i + 4];
        let n = xs.iter().unique().count();
        if n == 4 {
            return i + 4;
        }
    }

    unreachable!();
}

// Part2 ========================================================================
#[aoc(day6, part2)]
#[inline(never)]
pub fn part2(input: &str) -> usize {
    let input = input.as_bytes();

    for i in 0..(input.len()) {
        let xs = &input[i..i + 14];
        let n = xs.iter().unique().count();
        if n == 14 {
            return i + 14;
        }
    }

    unreachable!();
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case::given(7, "mjqjpqmgbljsphdztnvjfqwrcgsmlb")]
    #[case::given(5, "bvwbjplbgvbhsrlpgdmjqwftvncz")]
    #[trace]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1)]
        p: impl FnOnce(&str) -> usize,
        #[case] expected: usize,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(input), expected);
    }

    #[rstest]
    #[case::given(19, "mjqjpqmgbljsphdztnvjfqwrcgsmlb")]
    #[trace]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2)]
        p: impl FnOnce(&str) -> usize,
        #[case] expected: usize,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(input), expected);
    }
}
