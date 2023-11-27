use crate::prelude::*;

use std::fmt;

type UnresolvedValue = Either<Complex<f64>, (Op, Monkey, Monkey)>;

#[derive(Copy, Clone, Debug, Hash)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Copy, Clone, Debug)]
struct MonkeyInfo {
    name: Monkey,
    value: UnresolvedValue,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Monkey([u8; 4]);

impl Monkey {
    fn name(&self) -> &str {
        std::str::from_utf8(&self.0).unwrap()
    }
}

impl fmt::Display for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", self.name())
    }
}

impl fmt::Debug for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", self.name())
    }
}

fn parse_monkeys(input: &str) -> Vec<MonkeyInfo> {
    let lines: Vec<_> = input.lines().map(|s| s.as_bytes()).collect();

    let mut monkeys = Vec::with_capacity(lines.len());
    for line in lines {
        let mut parts = line.split(|b| *b == b':');
        let name = Monkey(parts.next().unwrap().try_into().unwrap());

        let expr = parts.next().unwrap();
        let value = if expr[1].is_ascii_digit() {
            // ex: " 5"
            Left(Complex::new(fast_parse_u32(&expr[1..]) as f64, 0.))
        } else {
            // Everything is really evenly sized
            // ex: " pppw + sjmn"
            let left = Monkey(expr[1..5].try_into().unwrap());
            let right = Monkey(expr[8..12].try_into().unwrap());
            let op = match expr[6] {
                b'+' => Op::Add,
                b'-' => Op::Sub,
                b'*' => Op::Mul,
                b'/' => Op::Div,
                b => unreachable!("Unexpected 'op': {b}"),
            };

            Right((op, left, right))
        };

        let info = MonkeyInfo { name, value };
        monkeys.push(info);
    }

    monkeys
}

fn resolve_monkey(
    monkeys: &mut [MonkeyInfo],
    idx_lookup: &HashMap<Monkey, usize>,
    idx: usize,
) -> Complex<f64> {
    debug!("resolving {}", monkeys[idx].name);

    let val = match monkeys[idx].value {
        Left(n) => n,
        Right((op, left, right)) => {
            let left = resolve_monkey(monkeys, idx_lookup, idx_lookup[&left]);
            let right = resolve_monkey(monkeys, idx_lookup, idx_lookup[&right]);

            debug!(
                "resolving {name} left={left}, op={op:?} right={right}",
                name = monkeys[idx].name
            );

            match op {
                Op::Add => left + right,
                Op::Sub => left - right,
                Op::Mul => left * right,
                Op::Div => left / right,
            }
        }
    };
    debug!("resolving {} to {val}", monkeys[idx].name);

    monkeys[idx].value = Left(val);
    val
}

// Part1 ========================================================================
#[aoc(day21, part1)]
pub fn part1(input: &str) -> f64 {
    let mut monkeys = parse_monkeys(input);

    // Recursively resolve in-place
    let idx_lookup: HashMap<_, _> = monkeys
        .iter()
        .enumerate()
        .map(|(idx, m)| (m.name, idx))
        .collect();

    let root_idx = idx_lookup[&Monkey(*b"root")];
    resolve_monkey(&mut monkeys, &idx_lookup, root_idx);

    monkeys[root_idx]
        .value
        .left()
        .expect("Unresolved monkey at root")
        .re
}

// Part2 ========================================================================
#[aoc(day21, part2)]
pub fn part2(input: &str) -> f64 {
    let mut monkeys = parse_monkeys(input);

    // Recursively resolve in-place
    let idx_lookup: HashMap<_, _> = monkeys
        .iter()
        .enumerate()
        .map(|(idx, m)| (m.name, idx))
        .collect();

    // This is the only non-real entry in the list. We'll use it as a poor-man's algebraic solver.
    // This only works because no monkey in the list ever shows up twice (and we don't get accidental i**2)
    let humn_idx = idx_lookup[&Monkey(*b"humn")];
    monkeys[humn_idx].value = Left(Complex::new(0., 1.));

    // We need root's two deps to be equal, so save those now.
    let root_idx = idx_lookup[&Monkey(*b"root")];
    let (_op, root_left, root_right) = monkeys[root_idx]
        .value
        .right()
        .expect("root shouldn't be resolved after parsing");

    resolve_monkey(&mut monkeys, &idx_lookup, root_idx);

    // Grab both inputs to root
    let a = monkeys[idx_lookup[&root_left]].value.left().unwrap();
    let b = monkeys[idx_lookup[&root_right]].value.left().unwrap();

    info!("Resolved root_left={root_left} to {a}");
    info!("Resolved root_right={root_right} to {b}");

    // And algebra our way to success!
    (a.re - b.re) / (b.im - a.im)
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
";

    #[rstest]
    #[case::given(152., EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1)]
        p: impl FnOnce(&str) -> f64,
        #[case] expected: f64,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(input), expected);
    }

    #[rstest]
    #[case::given(301., EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2)]
        p: impl FnOnce(&str) -> f64,
        #[case] expected: f64,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(input), expected);
    }
}
