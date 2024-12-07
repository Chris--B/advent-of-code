use crate::prelude::*;

fn eat_until_mul_p1(input: &mut &str) -> Option<(i32, i32)> {
    while !input.is_empty() {
        // Find the start of a mul(), if there is one
        while !input.starts_with("mul(") {
            *input = input.get(1..)?;
        }
        // Skip over the "mul(" part
        *input = &input[4..];

        let mut first_arg = 0;

        // Walk the arguments until we find two valid i32s, or we determine this arg list is illegal.
        let mut i = 0;
        'parse: while let Some(c) = input[i..].chars().next() {
            // We expect straight numeric until either a ',' or a ')'.
            // Anything else, and this mul() is invalid and should be skipped.
            if c.is_numeric() {
                i += 1;
                continue 'parse;
            }

            if c == ',' {
                // parse the first arg
                if let Ok(arg) = input[..i].parse() {
                    first_arg = arg;

                    // Reset our parsing window
                    *input = &input[(i + 1)..];
                    i = 0;
                    continue 'parse;
                } else if cfg!(test) {
                    unreachable!("Failed to parse arg {:?} (out of {input:?})", &input[..i]);
                }
            }

            if c == ')' {
                // parse the second arg and exit!
                if let Ok(arg) = input[..i].parse() {
                    return Some((first_arg, arg));
                } else if cfg!(test) {
                    unreachable!("Failed to parse arg {:?} (out of {input:?})", &input[..i]);
                }
            }

            // We get here if we have an illegal character, OR if parsing above failed.
            break 'parse;
        }

        // Advance
        i += 1;
        *input = input.get(i..)?;
    }

    // There's nothing left to parse
    None
}

// Part1 ========================================================================
#[aoc(day3, part1)]
pub fn part1(mut input: &str) -> i32 {
    let mut sum = 0;
    while let Some((a, b)) = eat_until_mul_p1(&mut input) {
        if cfg!(test) {
            println!("    + {a}, {b}");
        }
        sum += a * b;
    }

    sum
}

// Part2 ========================================================================
fn eat_until_mul_p2(input: &mut &str) -> Option<(i32, i32)> {
    let mut enabled = true;

    while !input.is_empty() {
        // Find the start of a mul(), if there is one
        #[allow(clippy::nonminimal_bool)]
        while !enabled || (enabled && !input.starts_with("mul(")) {
            if input.starts_with("do()") {
                enabled = true;
                *input = input.get(4..)?;
            } else if input.starts_with("don't()") {
                enabled = false;
                *input = input.get(7..)?;
            } else {
                *input = input.get(1..)?;
            }
        }
        // Skip over the "mul(" part
        *input = &input[4..];

        let mut first_arg = 0;

        // Walk the arguments until we find two valid i32s, or we determine this arg list is illegal.
        let mut i = 0;
        'parse: while let Some(c) = input[i..].chars().next() {
            // We expect straight numeric until either a ',' or a ')'.
            // Anything else, and this mul() is invalid and should be skipped.
            if c.is_numeric() {
                i += 1;
                continue 'parse;
            }

            if c == ',' {
                // parse the first arg
                if let Ok(arg) = input[..i].parse() {
                    first_arg = arg;

                    // Reset our parsing window
                    *input = &input[(i + 1)..];
                    i = 0;
                    continue 'parse;
                } else if cfg!(test) {
                    unreachable!("Failed to parse arg {:?} (out of {input:?})", &input[..i]);
                }
            }

            if c == ')' {
                // parse the second arg and exit!
                if let Ok(arg) = input[..i].parse() {
                    return Some((first_arg, arg));
                } else if cfg!(test) {
                    unreachable!("Failed to parse arg {:?} (out of {input:?})", &input[..i]);
                }
            }

            // We get here if we have an illegal character, OR if parsing above failed.
            break 'parse;
        }

        // Advance
        i += 1;
        *input = input.get(i..)?;
    }

    // There's nothing left to parse
    None
}

#[aoc(day3, part2)]
pub fn part2(mut input: &str) -> i32 {
    let mut sum = 0;
    while let Some((a, b)) = eat_until_mul_p2(&mut input) {
        if cfg!(test) {
            println!("    + {a}, {b}");
        }
        sum += a * b;
    }

    sum
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    // Part 1 expects:
    //       2, 4
    //       5, 5
    //      11, 8
    //       8, 5
    const EXAMPLE_INPUT_1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    // Part 2 expects:
    //       2, 4
    //       8, 5
    const EXAMPLE_INPUT_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[rstest]
    #[case::given(161, EXAMPLE_INPUT_1)]
    #[case::no_mul(0, "yo")]
    #[case::just_one_mul(2, "mul(1,2)")]
    #[case::bad_num(14, "mul(2,7)mul(1,a)")]
    #[case::short_and_good(11, "mul(1,1)mul(10,1)")]
    #[case::fake_out(20, "mul(1,2]mul(10,2)")]
    #[timeout(Duration::from_millis(10))]
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
    #[case::given(48, EXAMPLE_INPUT_2)]
    #[case::just_one_mul(2, "mul(1,2)")]
    #[case::bad_num(14, "mul(2,7)mul(1,a)")]
    #[case::short_and_good(11, "mul(1,1)mul(10,1)")]
    #[case::fake_out(20, "mul(1,2]mul(10,2)")]
    #[case::just_one_dont(0, "don't()")]
    #[case::just_one_dont_and_then_mul(0, "don't()mul(1,2)")]
    #[case::just_one_do_and_then_mul(2, "mul(1,2)")]
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
