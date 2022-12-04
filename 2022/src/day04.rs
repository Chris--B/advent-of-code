use aoc_runner_derive::{aoc, aoc_generator};

use scan_fmt::scan_fmt;

pub struct Section(u128);

impl Section {
    fn from_pair(a0: u8, a1: u8) -> Self {
        assert!(a0 <= a1);

        let x = (1 << a1) - 1;
        let y = (1 << (a0 - 1)) - 1;

        Self(x ^ y)
    }

    // True if one section range fully contains the other
    fn fully_contains(&self, other: &Self) -> bool {
        ((self.0 & other.0) == self.0) || ((self.0 & other.0) == other.0)
    }
}

#[aoc_generator(day4)]
pub fn parse_input(input: &str) -> Vec<(Section, Section)> {
    input
        .lines()
        .map(|line| {
            let (a0, a1, b0, b1) = scan_fmt!(line, "{}-{},{}-{}", u8, u8, u8, u8).unwrap();
            (Section::from_pair(a0, a1), Section::from_pair(b0, b1))
        })
        .collect()
}

// Part1 ========================================================================
#[aoc(day4, part1)]
#[inline(never)]
pub fn part1(input: &[(Section, Section)]) -> i64 {
    input.iter().filter(|(a, b)| a.fully_contains(b)).count() as i64
}

// Part2 ========================================================================
#[aoc(day4, part2)]
#[inline(never)]
pub fn part2(input: &[i64]) -> i64 {
    unimplemented!();
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

    #[test]
    fn check_section_init_1() {
        // from 2: 0b0001 (1 << 0) - 1
        // from 4: 0b1111 (1 << 5) - 1
        //       ^
        // exp:    0b1110
        let s = Section::from_pair(2, 4);
        let b = 0b_1110;
        assert_eq!(
            s.0, b,
            "Section(2, 4) needs to have binary of 0b{:04b} but has 0b{:04b}",
            b, s.0
        );
    }

    #[rstest]
    #[case::given(2, EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1)]
        p: impl FnOnce(&[(Section, Section)]) -> i64,
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
