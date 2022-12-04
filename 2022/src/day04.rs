use aoc_runner_derive::aoc;

#[repr(transparent)]
pub struct Section(u128);

impl Section {
    fn from_pair(a0: u8, a1: u8) -> Self {
        debug_assert!(a0 <= a1);

        let x = (1 << a1) - 1;
        let y = (1 << (a0 - 1)) - 1;

        Self(x ^ y)
    }

    // True if one section range fully contains the other
    fn fully_contains(&self, other: &Self) -> bool {
        let x = self.0 & other.0;
        x == self.0 || x == other.0
    }

    fn overlap_any(&self, other: &Self) -> bool {
        (self.0 & other.0) != 0
    }
}

// Part1 ========================================================================

#[aoc(day4, part1)]
#[inline(never)]
pub fn part1(input: &str) -> i64 {
    use itertools::Itertools;

    input
        .split(|c| ",-\n".contains(c))
        .map(|x| x.parse::<u8>().unwrap())
        .tuples()
        .map(|(a, b, c, d)| (Section::from_pair(a, b), Section::from_pair(c, d)))
        .filter(|(a, b)| a.fully_contains(b))
        .count() as i64
}

// Part2 ========================================================================

#[aoc(day4, part2)]
#[inline(never)]
pub fn part2(input: &str) -> i64 {
    use itertools::Itertools;

    input
        .split(|c| ",-\n".contains(c))
        .map(|x| x.parse::<u8>().unwrap())
        .tuples()
        .map(|(a, b, c, d)| (Section::from_pair(a, b), Section::from_pair(c, d)))
        .filter(|(a, b)| a.overlap_any(b))
        .count() as i64
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
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(input), expected);
    }

    #[rstest]
    #[case::given(4, EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(input), expected);
    }
}
