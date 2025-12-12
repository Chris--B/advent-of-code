use crate::prelude::*;

#[aoc(day10, part1)]
pub fn part1(input: &str) -> i64 {
    let input = input.as_bytes();

    // Dedicated function to remove it from the parsing nonsense
    fn solve(buttons: &[u16], goal: u16) -> i64 {
        let mut min_presses = i64::MAX;

        for mut bits in 0..(1 << buttons.len()) {
            let mut state = 0;
            let mut presses = 0;

            'buttons: for btn in buttons {
                if bits == 0 {
                    break;
                }

                if bits & 1 != 0 {
                    state ^= btn;
                    presses += 1;

                    if state == goal {
                        min_presses = i64::min(min_presses, presses);
                        break 'buttons;
                    }
                }
                bits >>= 1;
            }
        }

        debug_assert!(min_presses != i64::MAX);
        min_presses
    }

    let mut total_presses = 0;
    let mut line_start = 0;
    for i in memchr_iter(b'\n', input).chain([input.len()]) {
        // "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"
        let mut line: &[u8] = &input[line_start..i];

        // "[.##.]"
        let mut goal = 0_u16;
        {
            let k = memchr(b']', line).unwrap_or_default();
            for kk in 1..k {
                if line[kk] == b'#' {
                    goal |= 1 << (kk - 1);
                }
            }
            line = &line[k..];
        }

        // " (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"
        let mut buttons = [0_u16; 30];
        let mut btn_idx = 0;
        {
            let mut s = 0;
            // let mut e = 0;
            for j in memchr2_iter(b'(', b')', line) {
                if line[j] == b'(' {
                    s = j;
                } else {
                    for &b in &line[s..j] {
                        if let b'0'..=b'9' = b {
                            buttons[btn_idx] |= 1 << (b - b'0');
                        }
                    }
                    btn_idx += 1;
                    // e = j;
                }
            }
            // line = &line[e + 1..];
        }

        // Solve
        total_presses += solve(&buttons[..btn_idx], goal);

        line_start = i + 1;
    }

    total_presses
}

// Part2 ========================================================================
// #[aoc(day10, part2)]
pub fn part2(_input: &str) -> i64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

    #[rstest]
    #[case::given(2+3+2, EXAMPLE_INPUT)]
    #[case::given_1(2, "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}")]
    #[case::given_2(3, "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}")]
    #[case::given_3(2, "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}")]
    #[trace]
    #[timeout(Duration::from_millis(100))]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        init_logging();

        let input = input.trim();
        assert_eq!(p(input), expected);
    }

    #[rstest]
    // #[case::given(10+12+11, EXAMPLE_INPUT)]
    #[case::given_1(10, "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}")]
    // #[case::given_2(12, "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}")]
    // #[case::given_3(11, "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}")]
    #[trace]
    #[ignore]
    #[timeout(Duration::from_millis(100))]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        init_logging();

        let input = input.trim();
        assert_eq!(p(input), expected);
    }
}
