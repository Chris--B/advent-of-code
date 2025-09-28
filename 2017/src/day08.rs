#![allow(unused)]
use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Registers<'a> {
    regs: HashMap<&'a str, i32>,
}

impl<'a> Registers<'a> {
    pub fn new() -> Self {
        Self {
            regs: HashMap::new(),
        }
    }

    pub fn get_mut(&mut self, reg: &'a str) -> &mut i32 {
        self.regs.entry(reg).or_insert(0)
    }

    pub fn run(&mut self, line: &'a str) {
        let parts = line.split_ascii_whitespace().collect_vec();

        if parts.is_empty() {
            return;
        }

        if let [out_reg, op, val, kw_if, if_reg, if_op, if_val] = parts[..] {
            let val: i32 = val.parse().unwrap();

            // eval if
            let mut ok = true;
            {
                let if_val: i32 = if_val.parse().unwrap();
                let if_reg = self.get_mut(if_reg);
                ok = match if_op {
                    "<" => *if_reg < if_val,
                    ">" => *if_reg > if_val,
                    "<=" => *if_reg <= if_val,
                    ">=" => *if_reg >= if_val,
                    "==" => *if_reg == if_val,
                    "!=" => *if_reg != if_val,
                    _ => unreachable!("Unexpected if_op={if_op}"),
                };

                if cfg!(debug_assertions) {
                    println!("{if_reg} {if_op} {if_val}:");
                }
            }

            // resolve inc/dec
            if ok {
                let reg = self.get_mut(out_reg);
                match op {
                    "inc" => {
                        if cfg!(debug_assertions) {
                            println!("  [{out_reg}] {reg} + {val};\t{} -> {}", reg, *reg + val);
                        }
                        *reg += val;
                    }
                    "dec" => {
                        if cfg!(debug_assertions) {
                            println!("  [{out_reg}] {reg} - {val};\t{} -> {}", reg, *reg + val);
                        }
                        *reg -= val;
                    }
                    _ => unreachable!("Unexpected op={op}"),
                }
            }
        } else {
            unreachable!("Unable to parse line={line:?}");
        }
    }

    pub fn print_state(&self) {
        let stdout = std::io::stdout();
        self.print_state_to(stdout.lock());
    }

    pub fn state(&self) -> String {
        let mut s = vec![];
        self.print_state_to(&mut s);
        String::from_utf8(s).unwrap()
    }

    pub fn print_state_to(&self, mut w: impl std::io::Write) {
        use std::io::Write;
        writeln!(w);
        writeln!(w, "Registers");
        writeln!(w, "-=======-");

        let mut regs = self.regs.keys().collect_vec();
        if regs.is_empty() {
            writeln!(w, "No registers set");
        } else {
            regs.sort();
            for reg in regs {
                let val = self.regs[reg];
                writeln!(w, "{reg:>4}: {val}");
            }
        }
    }
}

// Part1 ========================================================================
#[aoc(day8, part1)]
pub fn part1(input: &str) -> i64 {
    let mut regs = Registers::new();
    for line in input.lines() {
        regs.run(line);
    }

    if cfg!(debug_assertions) {
        regs.print_state();
    }
    *regs.regs.values().max().unwrap() as i64
}

// Part2 ========================================================================
#[aoc(day8, part2)]
pub fn part2(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10
";

    #[rstest]
    #[case::given([("a",1), ("b", 0), ("c", -10)], EXAMPLE_INPUT)]
    #[trace]
    fn check_register_run<'a>(
        #[case] expected: impl IntoIterator<Item = (&'a str, i32)>,
        #[case] input: &'a str,
    ) {
        let expected = Registers {
            regs: expected.into_iter().collect(),
        };
        let mut actual = Registers::new();
        for line in input.lines() {
            actual.run(line);
        }
        assert_eq!(actual.state(), expected.state());
    }

    #[rstest]
    #[case::given(1, EXAMPLE_INPUT)]
    #[trace]
    #[timeout(Duration::from_millis(1_500))]
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
    #[case::given(999_999, EXAMPLE_INPUT)]
    #[trace]
    #[ignore]
    #[timeout(Duration::from_millis(1_500))]
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
