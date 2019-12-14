use aoc_runner_derive::{aoc, aoc_generator};

use smallvec::SmallVec;
use std::cmp::{Ord, Ordering};

#[derive(Clone, Debug, Eq, Ord)]
struct WipPassword {
    pwd: SmallVec<[u8; 6]>,
    has_double: bool,
}

impl WipPassword {
    /// Construct a new wip password. A starter character must be provided
    fn new(first: u8) -> WipPassword {
        // Sanity check
        debug_assert!(first <= 9);

        let mut pwd = SmallVec::new();
        pwd.push(first);

        WipPassword {
            pwd,
            has_double: false,
        }
    }

    fn from_full_pwd(full: u32) -> WipPassword {
        let mut n = full;
        let mut pwd = SmallVec::new();

        while n != 0 {
            // Pull the least-significant digit each iteration
            let d = (n % 10) as u8;
            n /= 10;
            pwd.push(d);
        }

        // Flip into BigEndian for humans
        pwd.reverse();

        WipPassword {
            pwd,
            has_double: false, // Un-used for these
        }
    }

    // Last digit appended
    fn last(&self) -> u8 {
        *self.pwd.last().unwrap()
    }

    /// Whether or not this password is valid, according to the rules given.
    fn check(&self) -> bool {
        // Never should happen
        debug_assert!(!self.pwd.spilled());

        self.pwd.len() == 6 && self.has_double
    }
}

impl PartialEq for WipPassword {
    fn eq(&self, other: &Self) -> bool {
        self.pwd == other.pwd
    }
}

impl PartialOrd for WipPassword {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // When the two passwords are not equal length, we want to do a prefix compare
        // This way, a password of "2" >= "234567", which is how the puzzle works.
        let len = usize::min(self.pwd.len(), other.pwd.len());
        self.pwd[..len].partial_cmp(&other.pwd[..len])
    }
}

#[aoc_generator(day4)]
pub fn parse_range(input: &str) -> (u32, u32) {
    let mut iter = input.split('-');

    let lo = iter.next().unwrap().parse().expect("Invalid number for lo");
    let hi = iter.next().unwrap().parse().expect("Invalid number for hi");

    // Exactly 2 inputs
    debug_assert!(iter.next().is_none());

    (lo, hi)
}

#[aoc(day4, part1)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn p1_simple(range: &(u32, u32)) -> usize {
    let lo = WipPassword::from_full_pwd(range.0);
    let hi = WipPassword::from_full_pwd(range.1);
    debug_assert!(lo < hi);

    // Source of passwords to expand on
    let mut tasks: Vec<WipPassword> = (0..=9)
        .map(WipPassword::new)
        .filter(|pwd| lo <= *pwd && *pwd <= hi)
        .collect();
    tasks.reserve(1_000);
    // working set to modify as we iterate over tasks
    let mut stage: Vec<WipPassword> = vec![];
    stage.reserve(1_000);

    for _ in 0..5 {
        for curr_pwd in &tasks {
            // Only add digits that may be valid
            for next_digit in curr_pwd.last()..=9 {
                let mut next_pwd = curr_pwd.clone();

                // Add the next digit
                next_pwd.pwd.push(next_digit);

                // If we just created a double, mark it
                if curr_pwd.last() == next_digit {
                    next_pwd.has_double = true;
                }

                // Save to stage
                if lo <= next_pwd && next_pwd <= hi {
                    stage.push(next_pwd);
                }
            }
        }

        std::mem::swap(&mut tasks, &mut stage);
        stage.clear();
    }

    tasks.iter().filter(|pwd| pwd.check()).count()
}

// Assumes input is monotonic
fn has_double_run(bytes: &[u8]) -> bool {
    // We need at least 2 bytes to have a run of 2, so skip the loop
    if bytes.len() < 2 {
        return false;
    }

    let mut run_byte = bytes[0];
    let mut run_len = 1;

    for b in &bytes[1..] {
        let b = *b;
        if b == run_byte {
            // Continue the run
            run_len += 1;
        } else {
            // The run has broken, if it's exactly 2 long we can return early.
            // Otherwise, this run is no good and we must continue
            if run_len == 2 {
                return true;
            } else {
                run_byte = b;
                run_len = 1;
            }
        }
    }

    // Check this once more at the end - incase the run of exactly 2 terminates
    // the sequence we're given.
    // Otherwise, there is no run of 2.
    run_len == 2
}

#[cfg(test)]
#[test]
fn check_double_runs_0() {
    assert_eq!(has_double_run(&[1, 1, 2, 2, 3, 3]), true);
}

#[cfg(test)]
#[test]
fn check_double_runs_1() {
    assert_eq!(has_double_run(&[1, 2, 3, 4, 4, 4]), false);
}

#[cfg(test)]
#[test]
fn check_double_runs_2() {
    assert_eq!(has_double_run(&[1, 1, 1, 1, 2, 2]), true);
}

#[aoc(day4, part2)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn p2_simple(range: &(u32, u32)) -> usize {
    let lo = WipPassword::from_full_pwd(range.0);
    let hi = WipPassword::from_full_pwd(range.1);
    debug_assert!(lo < hi);

    // Source of passwords to expand on
    let mut tasks: Vec<WipPassword> = (0..=9)
        .map(WipPassword::new)
        .filter(|pwd| lo <= *pwd && *pwd <= hi)
        .collect();
    tasks.reserve(1_000);
    // working set to modify as we iterate over tasks
    let mut stage: Vec<WipPassword> = vec![];
    stage.reserve(1_000);

    for _ in 0..5 {
        for curr_pwd in &tasks {
            // Only add digits that may be valid
            for next_digit in curr_pwd.last()..=9 {
                let mut next_pwd = curr_pwd.clone();

                // Add the next digit
                next_pwd.pwd.push(next_digit);

                // Save to stage
                if lo <= next_pwd && next_pwd <= hi {
                    stage.push(next_pwd);
                }
            }
        }

        std::mem::swap(&mut tasks, &mut stage);
        stage.clear();
    }

    tasks.iter().filter(|pwd| has_double_run(&pwd.pwd)).count()
}
