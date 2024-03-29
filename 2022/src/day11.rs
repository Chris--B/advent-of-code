use crate::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Op {
    Mul(u64),
    Add(u64),
    Square,
}
use Op::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Monkey {
    op: Op,
    items: Vec<u64>,
    divisible_by: u64,
    if_true: u64,
    if_false: u64,
    throws: u64,
}

fn parse_first(input: &str) -> Vec<Monkey> {
    input
        .lines()
        .chunks(7)
        .into_iter()
        .map(|chunk| {
            /*
                    [0] Monkey 0:
                    [1]   Starting items: 50, 70, 89, 75, 66, 66
                    [2]   Operation: new = old * 5
                    [3]   Test: divisible by 2
                    [4]     If true: throw to monkey 2
                    [5]     If false: throw to monkey 1
                    [6] <empty line>
            */
            let s: [&str; 7] = iter_to_array_or_default(chunk);
            // Ignore line 0
            debug_assert_eq!(&s[0][..6], "Monkey");

            // Text followed by a list of items
            debug_assert_eq!(&s[1][..17], "  Starting items:");
            let items: Vec<_> = s[1][18..]
                .split(',')
                .map(|e| e.trim().parse().unwrap())
                .collect();

            // Text followed by one of three lines to describe a math op
            debug_assert_eq!(&s[2][..19], "  Operation: new = ");
            let ops: [&str; 3] = iter_to_array(s[2][19..].split_whitespace());
            let op: Op = match ops {
                ["old", "*", "old"] => Square,
                [a, "*", "old"] | ["old", "*", a] => Mul(a.parse().unwrap()),
                [a, "+", "old"] | ["old", "+", a] => Add(a.parse().unwrap()),
                _ => unreachable!("Unrecognized op sequence: {ops:?}"),
            };

            // Text followed by a number
            debug_assert_eq!(&s[3][..20], "  Test: divisible by");
            let divisible_by: u64 = s[3][21..].parse().unwrap();

            // Text followed by a number
            debug_assert_eq!(&s[4][..28], "    If true: throw to monkey");
            let if_true: u64 = s[4][29..].parse().unwrap();

            // Text followed by a number
            debug_assert_eq!(&s[5][..29], "    If false: throw to monkey");
            let if_false: u64 = s[5][30..].parse().unwrap();

            // Trailing empty line
            debug_assert_eq!(s[6].trim(), "");

            Monkey {
                op,
                divisible_by,
                items,
                if_true,
                if_false,
                throws: 0,
            }
        })
        .collect()
}

fn do_monkey_business<const N: u64>(rounds: u16, monkeys: &mut [Monkey]) {
    let m: u64 = monkeys.iter().map(|m| m.divisible_by).product();

    if cfg!(debug_assertions) {
        // print_state(0, monkeys);
    }

    for _round in 1..=rounds {
        // Each monkey inspects each item in order
        for id in 0..monkeys.len() {
            // Note: looping with indices here to satisfy the borrow checker.
            // (We're indexing twice, mutability into the same Monkeys slice)
            for item_idx in 0..monkeys[id].items.len() {
                let mut item: u64 = monkeys[id].items[item_idx];

                // ... getting worried
                item = match monkeys[id].op {
                    Mul(x) => (item * x) % m,
                    Add(x) => (item + x) % m,
                    Square => (item * item) % m,
                };

                // okay phew
                item /= N;

                // uh...
                let next_id = if item % monkeys[id].divisible_by == 0 {
                    monkeys[id].if_true as usize
                } else {
                    monkeys[id].if_false as usize
                };

                // ohno, the monkey threw it
                monkeys[next_id].items.push(item);
            }

            // conservation of... matter...?
            monkeys[id].throws += monkeys[id].items.len() as u64;
            monkeys[id].items.clear();
        }

        // print_state(_round, monkeys);
    }
}

// fn print_state(round: u16, monkeys: &[Monkey]) {
//     if cfg!(debug_assertions) {
//         if round == 0 {
//             println!("=== Starting");
//         } else {
//             println!("=== Round {round}");
//         }

//         for (id, monkey) in monkeys.iter().enumerate() {
//             println!("Monkey {id}: {:?}", monkey.items);
//         }

//         println!();
//     }
// }

// First Algo ==================================================================
#[aoc(day11, part1, first)]
pub fn part1(input: &str) -> u64 {
    let mut monkeys = parse_first(input);

    do_monkey_business::<3>(20, &mut monkeys);

    let mut counts: SmallVec<[_; 4]> = monkeys.iter().map(|m| m.throws).collect();
    counts.sort();
    counts.reverse();

    // Only need the 2 max, not a full sort
    counts[0] * counts[1]
}

#[aoc(day11, part2, first)]
pub fn part2(input: &str) -> u64 {
    let mut monkeys = parse_first(input);

    // Note: We do a few more rounds than part 1 did
    do_monkey_business::<1>(10_000, &mut monkeys);

    let mut counts: SmallVec<[_; 4]> = monkeys.iter().map(|m| m.throws).collect();
    counts.sort();
    counts.reverse();

    // Only need the 2 max, not a full sort
    counts[0] * counts[1]
}

// Item Tracking ===============================================================
#[derive(Copy, Clone, Debug)]
struct MonkeyInfo {
    op: Op,
    divisible_by: u64,
    if_true: usize,
    if_false: usize,
    throws: u64,
}

#[derive(Copy, Clone, Debug)]
struct Item {
    /// Current modulated worry level for this item
    worry: u64,

    /// Monkey currently holding this item
    monkey: usize,
}

fn parse_tracking(input: &str) -> (Vec<MonkeyInfo>, Vec<Item>) {
    let mut items = vec![];

    let monkeys: Vec<_> = input
        .lines()
        .chunks(7)
        .into_iter()
        .map(|chunk| {
            /*
                    [0] Monkey 0:
                    [1]   Starting items: 50, 70, 89, 75, 66, 66
                    [2]   Operation: new = old * 5
                    [3]   Test: divisible by 2
                    [4]     If true: throw to monkey 2
                    [5]     If false: throw to monkey 1
                    [6] <empty line>
            */
            let s: [&str; 7] = iter_to_array_or_default(chunk);
            debug_assert_eq!(&s[0][..6], "Monkey");
            let monkey = s[0][7..].split(':').next().unwrap().parse().unwrap();

            // Text followed by a list of items
            debug_assert_eq!(&s[1][..17], "  Starting items:");
            for item in s[1][18..].split(',').map(|e| {
                let worry = e.trim().parse().unwrap();
                Item { worry, monkey }
            }) {
                items.push(item);
            }

            // Text followed by one of three lines to describe a math op
            debug_assert_eq!(&s[2][..19], "  Operation: new = ");
            let ops: [&str; 3] = iter_to_array(s[2][19..].split_whitespace());
            let op: Op = match ops {
                ["old", "*", "old"] => Square,
                [a, "*", "old"] | ["old", "*", a] => Mul(a.parse().unwrap()),
                [a, "+", "old"] | ["old", "+", a] => Add(a.parse().unwrap()),
                _ => unreachable!("Unrecognized op sequence: {ops:?}"),
            };

            // Text followed by a number
            debug_assert_eq!(&s[3][..20], "  Test: divisible by");
            let divisible_by = s[3][21..].parse().unwrap();

            // Text followed by a number
            debug_assert_eq!(&s[4][..28], "    If true: throw to monkey");
            let if_true = s[4][29..].parse().unwrap();

            // Text followed by a number
            debug_assert_eq!(&s[5][..29], "    If false: throw to monkey");
            let if_false = s[5][30..].parse().unwrap();

            // Trailing empty line
            debug_assert_eq!(s[6].trim(), "");

            MonkeyInfo {
                op,
                divisible_by,

                if_true,
                if_false,
                throws: 0,
            }
        })
        .collect();

    (monkeys, items)
}

fn do_monkey_business_tracking<const N: u64>(
    rounds: u16,
    monkeys: &mut [MonkeyInfo],
    items: &mut [Item],
) {
    let m: u64 = monkeys.iter().map(|m| m.divisible_by).product();

    for item in items.iter_mut() {
        for _ in 1..=rounds {
            loop {
                let monkey = &mut monkeys[item.monkey];

                // ... getting worried
                item.worry = match monkey.op {
                    Mul(x) => (item.worry * x) % m,
                    Add(x) => (item.worry + x) % m,
                    Square => (item.worry * item.worry) % m,
                };

                // okay phew
                item.worry /= N;

                // uh...
                let prev_id = item.monkey;
                let next_id = if item.worry % monkey.divisible_by == 0 {
                    monkey.if_true
                } else {
                    monkey.if_false
                };

                // ohno, the monkey threw it
                item.monkey = next_id;
                monkey.throws += 1;

                // If we threw the item to a monkey earlier in the round, we won't be updated
                // again this round and we can count this round as over.
                if next_id < prev_id {
                    break;
                }
            }
        }
    }
}

#[aoc(day11, part1, tracking)]
pub fn part1_tracking(input: &str) -> u64 {
    let (mut monkeys, mut items) = parse_tracking(input);

    do_monkey_business_tracking::<3>(20, &mut monkeys, &mut items);

    let mut counts: SmallVec<[_; 4]> = monkeys.iter().map(|m| m.throws).collect();
    counts.sort();
    counts.reverse();

    // Only need the 2 max, not a full sort
    counts[0] * counts[1]
}

#[aoc(day11, part2, tracking)]
pub fn part2_tracking(input: &str) -> u64 {
    let (mut monkeys, mut items) = parse_tracking(input);

    do_monkey_business_tracking::<1>(10_000, &mut monkeys, &mut items);

    let mut counts: SmallVec<[_; 4]> = monkeys.iter().map(|m| m.throws).collect();
    counts.sort();
    counts.reverse();

    // Only need the 2 max, not a full sort
    counts[0] * counts[1]
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

    #[rstest]
    #[case::given(10605, EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1, part1_tracking)]
        p: impl FnOnce(&str) -> u64,
        #[case] expected: u64,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(input), expected);
    }

    #[rstest]
    #[case::given(2713310158, EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2, part2_tracking)]
        p: impl FnOnce(&str) -> u64,
        #[case] expected: u64,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(input), expected);
    }

    #[test]
    fn check_ex_part_1_counts() {
        let mut monkeys = parse_first(EXAMPLE_INPUT.trim());
        do_monkey_business::<3>(20, &mut monkeys);

        let counts: Vec<_> = monkeys.iter().map(|m| m.throws).collect();
        assert_eq!(counts, [101, 95, 7, 105]);
    }

    #[test]
    fn check_ex_part_2_counts() {
        let mut monkeys = parse_first(EXAMPLE_INPUT.trim());
        do_monkey_business::<1>(10_000, &mut monkeys);

        let counts: Vec<_> = monkeys.iter().map(|m| m.throws).collect();
        assert_eq!(counts, [52166, 47830, 1938, 52013]);
    }

    #[test]
    fn check_monkey_parse() {
        let monkeys = parse_first(EXAMPLE_INPUT.trim());
        assert_eq!(
            monkeys,
            [
                Monkey {
                    items: vec![79, 98],
                    op: Mul(19),
                    divisible_by: 23,
                    if_true: 2,
                    if_false: 3,
                    throws: 0,
                },
                Monkey {
                    items: vec![54, 65, 75, 74],
                    op: Add(6),
                    divisible_by: 19,
                    if_true: 2,
                    if_false: 0,
                    throws: 0,
                },
                Monkey {
                    items: vec![79, 60, 97],
                    op: Square,
                    divisible_by: 13,
                    if_true: 1,
                    if_false: 3,
                    throws: 0,
                },
                Monkey {
                    items: vec![74],
                    op: Add(3),
                    divisible_by: 17,
                    if_true: 0,
                    if_false: 1,
                    throws: 0,
                },
            ]
        )
    }
}
