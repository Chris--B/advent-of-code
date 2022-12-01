use aoc_runner_derive::{aoc, aoc_generator};

// Each day:
//  - Ctrl + F on dayN below
//  - Uncomment this module in lib.rs

#[aoc_generator(dayN)]
pub fn parse_input(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect()
}

// Part1 ========================================================================
#[aoc(dayN, part1)]
#[inline(never)]
pub fn part1(input: &[i64]) -> i64 {
    unimplemented!();
}

// Part2 ========================================================================
#[aoc(dayN, part2)]
#[inline(never)]
pub fn part2(input: &[i64]) -> i64 {
    unimplemented!();
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
// todo
";

    #[rstest]
    #[case::given(999_999, EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1)]
        p: impl FnOnce(&[i64]) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(&parse_input(input)), expected);
    }

    #[rstest]
    #[case::given(999_999, EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2)]
        p: impl FnOnce(&[i64]) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(&parse_input(input)), expected);
    }
}
