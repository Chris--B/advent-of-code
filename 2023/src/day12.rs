#![allow(unused)]

use crate::prelude::*;

fn count_runs(s: &str, pattern: &str) -> [u8; 15] {
    let mut counts = [0; 15];

    let groups = s.chars().group_by(|c| pattern.contains(*c));
    for (is_run, chars) in groups.into_iter() {
        counts[chars.count()] += 1;
    }

    counts
}

fn sss(bytes: &[u8]) -> &str {
    std::str::from_utf8(bytes).unwrap()
}

fn check_row(row: &str) -> i64 {
    dbg!(row);
    if !row.contains('?') {
        return 1;
    }

    let (row_text, lens_text) = row.split_once(' ').unwrap();

    let rs: Vec<_> = todo!();
    let mut ls: Vec<i64> = lens_text.split(',').map(|n| n.parse().unwrap()).collect();
    rs.sort_by_key(|n| -n);
    ls.sort_by_key(|n| -n);

    // Count the number of integer partitions for each r in 'runs', that we can construct with 'lens'
    fn solve(rs: &[i64], ls: &[i64]) -> i64 {
        assert!(!rs.is_empty());
        assert!(!ls.is_empty());

        // To reduce on recurse, we need to replace '#'->'?' in row OR shrink 'ls'
        // Both should always happen together!

        match (rs, ls) {
            ([r], [l]) => (r >= l) as i64,

            _ => todo!(),
        }
    }

    solve(&rs, &ls)
}

// Part1 ========================================================================
#[aoc(day12, part1)]
pub fn part1(input: &str) -> i64 {
    input.lines().map(check_row).sum()
}

// Part2 ========================================================================
#[aoc(day12, part2)]
pub fn part2(input: &str) -> i64 {
    #![allow(unused)]

    0
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";

    #[rstest]
    // #[case::given(21, EXAMPLE_INPUT)]
    #[case::given(1, "? 1")]
    #[case::given(2, "?.? 1,1")]
    // #[case::given(2, "??? 1,1")]
    // #[case::given(1, "???.### 1,1,3")]
    // #[case::given(4, ".??..??...?##. 1,1,3")]
    // #[case::given(1, "?#?#?#?#?#?#?#? 1,3,1,6")]
    // #[case::given(1, "????.#...#... 4,1,1")]
    // #[case::given(4, "????.######..#####. 1,6,5")]
    // #[case::given(10, "?###???????? 3,2,1")]
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
    #[ignore]
    #[case::given(999_999, EXAMPLE_INPUT)]
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
