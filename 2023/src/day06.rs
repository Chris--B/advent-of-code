use crate::prelude::*;

fn count_ways(time: i64, dist: i64) -> i64 {
    let time = time as f64;
    let dist = dist as f64;

    let d = (time * time - 4. * dist).sqrt();
    let a = ((time + d) / 2.).ceil();
    let b = ((time - d) / 2.).floor();

    debug_assert!(a > 0.);
    debug_assert!(b > 0.);

    (a - b - 1.) as i64
}

// Part1 ========================================================================
#[aoc(day6, part1)]
pub fn part1(input: &str) -> i64 {
    let (time_str, dist_str) = input
        .as_bytes()
        .split_at(if cfg!(test) && input.len() < 50 {
            21
        } else {
            36
        });

    let time = time_str[11..]
        .split(|b| b.is_ascii_whitespace())
        .filter(|bs| !bs.is_empty())
        .map(|s| fast_parse_u32(s) as i64);

    let dist = dist_str[11..]
        .split(|b| b.is_ascii_whitespace())
        .filter(|bs| !bs.is_empty())
        .map(|s| fast_parse_u32(s) as i64);

    let mut r = 1;
    for (t, d) in time.zip(dist) {
        r *= count_ways(t, d);
    }
    r
}

// Part2 ========================================================================
#[aoc(day6, part2)]
pub fn part2(input: &str) -> i64 {
    let (time_str, dist_str) = input
        .as_bytes()
        .split_at(if cfg!(test) && input.len() < 50 {
            21
        } else {
            36
        });

    let time = fast_parse_u64(
        time_str[11..]
            .iter()
            .filter(|b| b.is_ascii_digit())
            .copied(),
    );

    let dist = fast_parse_u64(
        dist_str[11..]
            .iter()
            .filter(|b| b.is_ascii_digit())
            .copied(),
    );

    count_ways(time as i64, dist as i64)
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
Time:      7  15   30
Distance:  9  40  200
";

    const MY_INPUT: &str = r"
Time:        59     79     65     75
Distance:   597   1234   1032   1328
";

    #[rstest]
    #[case::given_1(4, 7, 9)]
    #[case::given_2(8, 15, 40)]
    #[case::given_3(9, 30, 200)]
    #[case::given_pt2(71503, 71530, 940200)]
    #[case::mine_1(34, 59, 597)]
    #[case::mine_2(36, 79, 1234)]
    #[case::mine_3(10, 65, 1032)]
    #[case::mine_4(18, 75, 1328)]
    #[case::mine_pt2(34454850, 59796575, 597123410321328)]
    #[trace]
    fn check_count_ways(#[case] expectd: i64, #[case] time: i64, #[case] dist: i64) {
        assert_eq!(expectd, count_ways(time, dist));
    }

    #[rstest]
    #[case::given(288, EXAMPLE_INPUT)]
    #[case::mine(220320, MY_INPUT)]
    #[trace]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        let input = input.trim();
        let actual = p(input);
        assert_eq!(actual, expected, "Expected {expected} but got {actual}");
    }

    #[rstest]
    #[case::given(71503, EXAMPLE_INPUT)]
    #[case::mine(34454850, MY_INPUT)]
    #[trace]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        let input = input.trim();
        let actual = p(input);
        assert_eq!(actual, expected, "Expected {expected} but got {actual}");
    }
}
