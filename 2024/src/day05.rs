use crate::prelude::*;

type SmallVec<T> = smallvec::SmallVec<[T; 32]>;

fn parse(input: &str) -> ([Bitset128; 100], impl Iterator<Item = SmallVec<i32>> + '_) {
    let mut before = [Bitset128::new(); 100];

    let mut lines = input.lines();
    for line in lines.by_ref() {
        if line.trim().is_empty() {
            break;
        }

        let (a, b) = line.split_once('|').unwrap();
        let a: u8 = parse_or_fail(a);
        let b: u8 = parse_or_fail(b);

        before[a as usize].insert(b);
    }

    let updates = lines.map(|line| line.split(",").map(parse_or_fail).collect());

    (before, updates)
}

// Part1 ========================================================================
#[aoc(day5, part1)]
pub fn part1(input: &str) -> i32 {
    let (before, updates) = parse(input);

    let mut answer = 0;
    'updates: for update in updates {
        debug_assert_eq!(update.len() % 2, 1);

        for (&a, &b) in update.iter().tuple_windows() {
            if !before[a as usize].contains(b) {
                continue 'updates;
            }
        }

        answer += update[update.len() / 2];
    }

    answer
}

// Part2 ========================================================================
#[aoc(day5, part2)]
pub fn part2(input: &str) -> i32 {
    let (before, mut updates) = parse(input);

    let mut answer = 0;
    for mut update in &mut updates {
        debug_assert_eq!(update.len() % 2, 1);

        if update.is_sorted_by(|&a, &b| before[a as usize].contains(b)) {
            continue;
        }

        update.sort_unstable_by(|&a, &b| {
            if before[a as usize].contains(b) {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });

        answer += update[update.len() / 2];
    }

    answer
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
