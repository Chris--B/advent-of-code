use crate::prelude::*;

type SmallVec<T> = smallvec::SmallVec<[T; 32]>;

fn parse(input: &str) -> ([Bitset128; 100], impl Iterator<Item = SmallVec<i32>> + '_) {
    let input = input.as_bytes();

    let mut before = [Bitset128::new(); 100];

    // The input here comes in two parts: rules and updates. We'll use this `i` to track our progress in the rules,
    // and then know where to start parsing for the updates.
    let mut i = 0;
    while input[i] != b'\n' {
        // Each line looks like:
        //      12|34
        //      ^  ^
        //      0  3
        let a = 10 * (input[i + 0] - b'0') as i32 + (input[i + 1] - b'0') as i32;
        let b = 10 * (input[i + 3] - b'0') as i32 + (input[i + 4] - b'0') as i32;
        before[a as usize].insert(b);

        // Each line is exactly 6 long
        i += 6;
    }

    // Step over the final newline
    i += 1;

    let updates = input[i..]
        .split(|&byte| byte == b'\n')
        .map(|line| -> SmallVec<i32> {
            line.split(|&b| b == b',')
                // Each entry here is exactly 2 digits, like above
                .map(|bytes| 10 * (bytes[0] - b'0') as i32 + (bytes[1] - b'0') as i32)
                .collect()
        });

    (before, updates)
}

// Part1 ========================================================================
#[aoc(day5, part1)]
pub fn part1(input: &str) -> i32 {
    let (before, updates) = parse(input);

    updates
        .filter(|update| update.is_sorted_by(|&a, &b| before[a as usize].contains(b)))
        .map(|update| update[update.len() / 2])
        .sum()
}

// Part2 ========================================================================
#[aoc(day5, part2)]
pub fn part2(input: &str) -> i32 {
    let (before, updates) = parse(input);

    updates
        .filter(|update| !update.is_sorted_by(|&a, &b| before[a as usize].contains(b)))
        .map(|mut update| {
            update.sort_unstable_by(|&a, &b| {
                if before[a as usize].contains(b) {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            });
            update
        })
        .map(|update| update[update.len() / 2])
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    #[rstest]
    #[case::given(143, EXAMPLE_INPUT)]
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
    #[case::given(123, EXAMPLE_INPUT)]
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
