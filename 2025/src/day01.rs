use crate::prelude::*;

fn parse_turns(input: &str) -> Vec<i32> {
    let input = input.as_bytes();

    let mut turns: Vec<i32> = Vec::with_capacity(5000);
    let mut turn = 0;
    let mut left = false;

    for &b in input {
        match b {
            b'\n' => {
                if left {
                    turn = -turn;
                }
                turns.push(turn);
                turn = 0;
            }
            b'L' => left = true,
            b'R' => left = false,
            b'0'..=b'9' => {
                turn *= 10;
                turn += (b - b'0') as i32;
            }
            _ => unreachable!(),
        }
    }

    // If (when?) input doesn't end in a newline, save whatever's in progress.
    if left {
        turn = -turn;
    }
    turns.push(turn);

    turns
}

#[allow(unused)]
fn parse_turns_squashed(input: &str) -> Vec<i32> {
    let input = input.as_bytes();

    let mut turns: Vec<i32> = Vec::with_capacity(5000);
    let mut turn = 0;
    let mut left = false;

    for &b in input {
        match b {
            b'\n' => {
                if left {
                    turn = -turn;
                }

                if let Some(last) = turns.last_mut()
                    && last.signum() == turn.signum()
                {
                    *last += turn;
                } else {
                    debug_assert_ne!(turn, 0);
                    turns.push(turn);
                }

                turn = 0;
            }
            b'L' => left = true,
            b'R' => left = false,
            b'0'..=b'9' => {
                turn *= 10;
                turn += (b - b'0') as i32;
            }
            _ => unreachable!(),
        }
    }

    // If (when?) input doesn't end in a newline, save whatever's in progress.
    if left {
        turn = -turn;
    }
    turns.push(turn);

    turns
}

// Part1 ========================================================================
#[aoc(day1, part1)]
pub fn part1(input: &str) -> i64 {
    let turns = parse_turns(input);
    let mut count: i64 = 0;
    let mut dial = 50;

    for turn in turns {
        dial = (dial + turn).rem_euclid(100);
        if dial == 0 {
            count += 1;
        }
    }

    count
}

// Part2 ========================================================================
#[aoc(day1, part2, brute)]
pub fn part2_brute(input: &str) -> i64 {
    let mut count: i64 = 0;
    let mut dial = 50;

    for turn in parse_turns(input) {
        for _ in 0..turn.abs() {
            if turn < 0 {
                dial -= 1;
            } else {
                dial += 1;
            }
            dial = (dial + 100) % 100;

            if dial == 0 {
                count += 1;
            }
        }
    }

    count
}

#[aoc(day1, part2, math)]
pub fn part2_math(input: &str) -> i64 {
    let mut count: i64 = 0;
    let mut dial = 50;

    for big_turn in parse_turns_squashed(input) {
        let prev_dial = dial;

        // Each full rotation must pass zero
        count += (big_turn.unsigned_abs() / 100) as i64;

        // Remainder may or may not pass zero
        dial += big_turn % 100;

        // NOTE: Moving negative from 0 does NOT trigger a tick
        if (prev_dial != 0 && dial <= 0) || dial >= 100 {
            count += 1;
        }

        dial = dial.rem_euclid(100);
    }

    count
}

#[aoc(day1, part2, math_inline)]
pub fn part2_math_inline(input: &str) -> i64 {
    fn count_zeros(dial: &mut i32, big_turn: i32) -> i64 {
        let mut count = 0;
        let prev_dial = *dial;

        count += (big_turn.unsigned_abs() / 100) as i64;
        *dial += big_turn % 100;

        if (prev_dial != 0 && *dial <= 0) || *dial >= 100 {
            count += 1;
        }

        *dial = dial.rem_euclid(100);

        count
    }

    let mut count: i64 = 0;
    let mut dial = 50;

    let mut turn = 0;
    let mut left = false;

    for &b in input.as_bytes() {
        match b {
            b'\n' => {
                if left {
                    turn = -turn;
                }

                // TODO: Combine with prev turn if same sign

                count += count_zeros(&mut dial, turn);
                turn = 0;
            }
            b'L' => left = true,
            b'R' => left = false,
            b'0'..=b'9' => {
                turn *= 10;
                turn += (b - b'0') as i32;
            }
            _ => unreachable!(),
        }
    }

    // If (when?) input doesn't end in a newline, save whatever's in progress.
    if left {
        turn = -turn;
    }
    if turn != 0 {
        count += count_zeros(&mut dial, turn);
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

    const EXAMPLE_SHORT_INPUT: &str = r"
L95
R674
L350
";

    const EXAMPLE_BLAH: &str = r"
L50
R150
R150
";

    #[test]
    fn check_parse_turns() {
        let input = EXAMPLE_INPUT.trim();
        let turns = parse_turns(input);
        assert_eq!(turns, [-68, -30, 48, -5, 60, -55, -1, -99, 14, -82]);
    }

    #[test]
    fn check_parse_turns_squash() {
        let input = EXAMPLE_INPUT.trim();
        let turns = parse_turns_squashed(input);
        assert_eq!(turns, [-98, 48, -5, 60, -155, 14, -82]);
    }

    #[rstest]
    #[case::given(3, EXAMPLE_INPUT)]
    #[trace]
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
    #[case::given(6, EXAMPLE_INPUT)]
    #[case::short_input(12, EXAMPLE_SHORT_INPUT)]
    #[case(4, EXAMPLE_BLAH)]
    #[trace]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2_brute, part2_math, part2_math_inline)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        init_logging();

        let input = input.trim();
        assert_eq!(p(input), expected);
    }
}
