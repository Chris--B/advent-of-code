use crate::prelude::*;

// Part1 ========================================================================
#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    let lines = input.lines().map(parse_list_whitespace::<2, i32>);

    let mut left = vec![];
    let mut right = vec![];

    for [l, r] in lines {
        left.push(l);
        right.push(r);
    }

    left.sort();
    right.sort();

    std::iter::zip(left, right)
        .map(|(l, r)| (l - r).abs())
        .sum()
}

// Part2 ========================================================================
#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
    let lines = input.lines().map(parse_list_whitespace::<2, i32>);

    let mut left = vec![];
    let tally = lines
        .map(|[l, r]| {
            // Save left column
            left.push(l);

            // Tally right column
            r
        })
        .tally();

    left.into_iter()
        .map(|l| l * (*tally.get(&l).unwrap_or(&0) as i32))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
3   4
4   3
2   5
1   3
3   9
3   3
";

    #[rstest]
    #[case::given(11, EXAMPLE_INPUT)]
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
    #[case::given(31, EXAMPLE_INPUT)]
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
