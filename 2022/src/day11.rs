#![allow(unused_variables)]

use crate::prelude::*;

#[derive(Clone, Copy, Debug)]
enum Op {
    Times(i64),
    Plus(i64),
    Square,
}
use Op::*;

#[derive(Clone)]
struct Monkey {
    items: Vec<i64>,
    op: Op,
    divisible_by: i64,
    if_true: u64,
    if_false: u64,
    count: u64,
}

fn parse(input: &str) -> Vec<Monkey> {
    let lines = input.lines().count();
    // dbg!(lines);

    if lines == 27 {
        vec![
            Monkey {
                items: vec![79, 98],
                op: Times(19),
                divisible_by: 23,
                if_true: 2,
                if_false: 3,
                count: 0,
            },
            Monkey {
                items: vec![54, 65, 75, 74],
                op: Plus(6),
                divisible_by: 19,
                if_true: 2,
                if_false: 0,
                count: 0,
            },
            Monkey {
                items: vec![79, 60, 97],
                op: Square,
                divisible_by: 13,
                if_true: 1,
                if_false: 3,
                count: 0,
            },
            Monkey {
                items: vec![74],
                op: Plus(3),
                divisible_by: 17,
                if_true: 0,
                if_false: 1,
                count: 0,
            },
        ]
    } else {
        vec![
            Monkey {
                items: vec![50, 70, 89, 75, 66, 66],
                op: Times(5),
                divisible_by: 2,
                if_true: 2,
                if_false: 1,
                count: 0,
            },
            Monkey {
                items: vec![85],
                op: Square,
                divisible_by: 7,
                if_true: 3,
                if_false: 6,
                count: 0,
            },
            Monkey {
                items: vec![66, 51, 71, 76, 58, 55, 58, 60],
                op: Plus(1),
                divisible_by: 13,
                if_true: 1,
                if_false: 3,
                count: 0,
            },
            Monkey {
                items: vec![79, 52, 55, 51],
                op: Plus(6),
                divisible_by: 3,
                if_true: 6,
                if_false: 4,
                count: 0,
            },
            Monkey {
                items: vec![69, 92],
                op: Times(17),
                divisible_by: 19,
                if_true: 7,
                if_false: 5,
                count: 0,
            },
            Monkey {
                items: vec![71, 76, 73, 98, 67, 79, 99],
                op: Plus(8),
                divisible_by: 5,
                if_true: 0,
                if_false: 2,
                count: 0,
            },
            Monkey {
                items: vec![82, 76, 69, 69, 57],
                op: Plus(7),
                divisible_by: 11,
                if_true: 7,
                if_false: 4,
                count: 0,
            },
            Monkey {
                items: vec![65, 79, 86],
                op: Plus(5),
                divisible_by: 17,
                if_true: 5,
                if_false: 0,
                count: 0,
            },
        ]
    }
}

fn process_part1_monkeys(monkeys: &mut [Monkey]) {
    let modulo: i64 = monkeys.iter().map(|m| m.divisible_by).product();
    // dbg!(modulo);

    for r in 1..=20 {
        for i in 0..(monkeys.len()) {
            let Monkey {
                items,
                op,
                divisible_by,
                if_true,
                if_false,
                ..
            } = monkeys[i].clone();
            monkeys[i].items.clear();
            for item in items {
                monkeys[i].count += 1;

                // pre-monkey worry level
                let pre_lvl = item;

                // changed level
                let lvl: i64 = match op {
                    Times(x) => item * x,
                    Plus(x) => item + x,
                    Square => item,
                };

                // post-monkey worry level
                let post_lvl = lvl;

                let next_monkey = if post_lvl % divisible_by == 0 {
                    if_true
                } else {
                    if_false
                };
                monkeys[next_monkey as usize].items.push(post_lvl);
            }
        }

        // println!("=== Round {r}");
        for (i, m) in monkeys.iter().enumerate() {
            // print!("Monkey {i}: ");
            for item in &m.items {
                // print!("{item}, ");
            }
            // println!();
        }
        // println!();
    }
}

// Part1 ========================================================================
// #[aoc(day11, part1)]
pub fn part1(input: &str) -> i64 {
    let mut monkeys = parse(input);

    // println!("=== Starting");
    for (i, m) in monkeys.iter().enumerate() {
        // print!("Monkey {}: ", i);
        for item in &m.items {
            // print!("{item}, ");
        }
        // println!();
    }
    // println!();

    process_part1_monkeys(&mut monkeys);

    let mut counts: Vec<_> = monkeys.iter().map(|m| m.count).collect();
    counts.sort();
    counts.reverse();

    // dbg!(&counts);

    (counts[0] * counts[1]) as i64
}

// Part2 ========================================================================
fn process_part2_monkeys(monkeys: &mut [Monkey]) {
    let modulo: i64 = monkeys.iter().map(|m| m.divisible_by).product();
    // dbg!(modulo);

    for r in 1..=10_000 {
        for i in 0..(monkeys.len()) {
            let Monkey {
                items,
                op,
                divisible_by,
                if_true,
                if_false,
                ..
            } = monkeys[i].clone();
            monkeys[i].items.clear();
            for item in items {
                monkeys[i].count += 1;

                // pre-monkey worry level
                let pre_lvl = item;

                // changed level
                // dbg!(item);
                let lvl: i64 = match op {
                    Times(x) => (item % modulo) * (x % modulo),
                    Plus(x) => (item % modulo) + (x % modulo),
                    Square => (item % modulo) * (item % modulo),
                };

                // post-monkey worry level
                let post_lvl = lvl;

                let next_monkey = if post_lvl % divisible_by == 0 {
                    if_true
                } else {
                    if_false
                };
                monkeys[next_monkey as usize].items.push(post_lvl);
            }
        }

        // println!("=== Round {r}");
        for (i, m) in monkeys.iter().enumerate() {
            // print!("Monkey {i}: ");
            for item in &m.items {
                // print!("{item}, ");
            }
            // println!();
        }
        // println!();
    }
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> i64 {
    let mut monkeys = parse(input);

    // println!("=== Starting");
    for (i, m) in monkeys.iter().enumerate() {
        // print!("Monkey {}: ", i);
        for item in &m.items {
            // print!("{item}, ");
        }
        // println!();
    }
    // println!();

    process_part2_monkeys(&mut monkeys);

    let mut counts: Vec<_> = monkeys.iter().map(|m| m.count).collect();
    counts.sort();
    counts.reverse();

    (counts[0] * counts[1]) as i64
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
        #[values(part1)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
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
        #[values(part2)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(input), expected);
    }
}
