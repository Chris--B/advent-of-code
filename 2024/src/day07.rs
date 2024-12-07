use crate::prelude::*;

use smallvec::smallvec;
type SmallVec<T> = smallvec::SmallVec<[T; 16]>;

#[derive(Clone, Debug)]
struct CalibEq {
    res: u64,
    args: Vec<u64>,
}

impl CalibEq {
    /// Uses the bits of `pattern` to combine args.
    ///
    /// A    set bit at index i means that arg i+1 is combined with a multiply
    /// An unset bit at index i means that arg i+1 is combined with an add
    /// arg 0 is used to intialize the total.
    ///
    /// If all bits in pattern are used, the return value is:
    ///     Some(true) iff the result matches `res`
    ///     Some(false) iff the result does NOT match `res`
    ///
    /// If some bits in the pattern are unused, the result is None.
    ///     If args.len() == 3, then bits 0 and 1 are used. Any set bits beyond that are an error and cause None to be returned.
    ///     This makes it easy to pump this with a counter and stop when None is returned.
    fn p1_check_with(&self, mut pattern: u16) -> Option<bool> {
        if pattern != 0 {
            let max_set_bit = pattern.ilog2() + 1;
            if max_set_bit >= self.args.len() as _ {
                return None;
            }
        }

        let mut total = self.args[0];
        for arg in &self.args[1..] {
            match pattern % 2 {
                0 => total *= arg,
                1 => total += arg,
                _ => unreachable!(),
            }
            pattern /= 2;

            if total > self.res {
                break;
            }
        }

        Some(total == self.res)
    }

    fn p2_check_with(&self, mut pattern: u32) -> Option<bool> {
        if pattern != 0 {
            let max_set_trit = (pattern as f32).log(3.).ceil() as u32 + 1;
            if max_set_trit > self.args.len() as _ {
                return None;
            }
        };

        let mut total = self.args[0];
        for &arg in &self.args[1..] {
            match pattern % 3 {
                0 => total *= arg,
                1 => total += arg,
                2 => total = 10.pow(arg.ilog10() + 1) * total + arg,
                _ => unreachable!(),
            }
            pattern /= 3;

            if total > self.res {
                break;
            }
        }

        Some(total == self.res)
    }
}

fn parse(input: &str) -> Vec<CalibEq> {
    let calib_eqs: Vec<CalibEq> = input
        .lines()
        .map(|line| {
            let (res, rest) = line.split_once(": ").unwrap();
            let res: u64 = parse_or_fail(res);
            let args: Vec<u64> = rest.split_ascii_whitespace().map(parse_or_fail).collect();

            CalibEq { res, args }
        })
        .collect();

    calib_eqs
}

// Part1 ========================================================================
#[aoc(day7, part1, brute_force)]
pub fn part1(input: &str) -> u64 {
    let calib_eqs = parse(input);

    let mut total = 0;
    for calib_eq in calib_eqs {
        for pattern in 0.. {
            if let Some(ok) = calib_eq.p1_check_with(pattern) {
                if ok {
                    total += calib_eq.res;
                    break;
                }
            } else {
                break;
            }
        }
    }

    total
}

// Part2 ========================================================================
#[aoc(day7, part2, brute_force)]
pub fn part2(input: &str) -> u64 {
    let calib_eqs = parse(input);

    let mut total = 0;
    for calib_eq in calib_eqs {
        for pattern in 0.. {
            if let Some(ok) = calib_eq.p2_check_with(pattern) {
                if ok {
                    total += calib_eq.res;
                    break;
                }
            } else {
                break;
            }
        }
    }

    total
}

#[aoc(day7, part2, smarter)]
pub fn part2_smarter(input: &str) -> u64 {
    let calib_eqs = parse(input);

    let mut total = 0;
    for eq in calib_eqs {
        // Walk the args backwards and track every value we *could* reach
        // Then we can see if the actual result is even possible!
        let mut possible: SmallVec<u64> = smallvec![eq.res];

        for &arg in eq.args[1..].iter().rev() {
            let mut next: SmallVec<u64> = smallvec![];

            for &tmp in &possible {
                if tmp >= arg {
                    next.push(tmp - arg);
                }
                if tmp % arg == 0 {
                    next.push(tmp / arg);
                }
                // Try and un-concatenate our temp state and the latest arg
                let pow10 = 10.pow(arg.ilog10() + 1);
                if tmp > arg && tmp % pow10 == arg {
                    debug_assert_eq!(format!("{}{arg}", tmp / pow10), tmp.to_string());
                    next.push(tmp / pow10);
                }
            }

            possible = next;
        }

        // If the possible states includes our first arg, then we found at least one valid combination of ops!
        if possible.contains(&eq.args[0]) {
            total += eq.res;
        }
    }

    total
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    #[rstest]
    #[case::given(3749, EXAMPLE_INPUT)]
    #[case::given_good_1(190, "190: 10 19")]
    #[case::given_good_2(3267, "3267: 81 40 27")]
    #[case::given_good_3(292, "292: 11 6 16 20")]
    #[case::given_bad_1(0, "83: 17 5")]
    #[case::given_bad_2(0, "7290: 6 8 6 15")]
    #[trace]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1)]
        p: impl FnOnce(&str) -> u64,
        #[case] expected: u64,
        #[case] input: &str,
    ) {
        init_logging();

        let input = input.trim();
        assert_eq!(p(input), expected);
    }

    #[rstest]
    #[case::given(190 + 3267 + 292 + 156 + 7290 + 192, EXAMPLE_INPUT)]
    #[case::given_good_1(190, "190: 10 19")]
    #[case::given_good_2(3267, "3267: 81 40 27")]
    #[case::given_good_3(292, "292: 11 6 16 20")]
    #[case::given_good_4(156, "156: 15 6")]
    #[case::given_good_5(7290, "7290: 6 8 6 15")]
    #[case::given_good_6(192, "192: 17 8 14")]
    #[case::given_bad_1(0, "83: 17 5")]
    #[trace]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2, part2_smarter)]
        p: impl FnOnce(&str) -> u64,
        #[case] expected: u64,
        #[case] input: &str,
    ) {
        init_logging();

        let input = input.trim();
        assert_eq!(p(input), expected);
    }
}
