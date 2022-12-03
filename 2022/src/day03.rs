use aoc_runner_derive::aoc;

use std::collections::HashSet;

pub fn parse_input_part_1(input: &str) -> Vec<(HashSet<u8>, HashSet<u8>)> {
    let mut bytes = input.as_bytes().to_owned();

    for b in &mut bytes {
        *b = match *b {
            b'a'..=b'z' => *b - b'a' + 1,
            b'A'..=b'Z' => *b - b'A' + 27,
            b'\n' => u8::MAX,
            _ => unreachable!("Unexpected byte: {} ({})", *b as char, *b),
        };
    }

    bytes
        .split(|b| *b == u8::MAX)
        .map(|line| {
            let (a, b) = line.split_at(line.len() / 2);
            debug_assert_eq!(a.len(), b.len());

            (
                HashSet::from_iter(a.iter().copied()),
                HashSet::from_iter(b.iter().copied()),
            )
        })
        .collect()
}

// Part1 ========================================================================
#[aoc(day3, part1)]
#[inline(never)]
pub fn part1(input: &str) -> i64 {
    let input = parse_input_part_1(input);
    let mut priority = 0;

    for (a, b) in &input {
        let mut iter = a.intersection(b);
        priority += *iter.next().unwrap() as i64;

        debug_assert_eq!(iter.next(), None);
    }

    priority
}

// Part2 ========================================================================
#[aoc(day3, part2)]
#[inline(never)]
pub fn part2(input: &str) -> i64 {
    unimplemented!();
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

    #[rstest]
    #[case::given(157, EXAMPLE_INPUT)]
    #[case::given_line1(16, "vJrwpWtwJgWrhcsFMMfFFhFp")]
    #[case::given_line2(38, "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL")]
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

    // #[rstest]
    // #[case::given(999_999, EXAMPLE_INPUT)]
    // #[trace]
    // fn check_ex_part_2(
    //     #[notrace]
    //     #[values(part2)]
    //     p: impl FnOnce(&[i64]) -> i64,
    //     #[case] expected: i64,
    //     #[case] input: &str,
    // ) {
    //     let input = input.trim();
    //     assert_eq!(p(&parse_input(input)), expected);
    // }
}
