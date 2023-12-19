#![allow(unused)]
use crate::prelude::*;

type Part = [i64; 4];
const X: usize = 0;
const M: usize = 1;
const A: usize = 2;
const S: usize = 3;
const DONT: usize = 10;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Op {
    /// Process if Greater Than
    Gt(i64),
    /// Process if Less Than
    Lt(i64),
    /// Process unconditionally
    Jump,
}
use Op::*;

type Rule<'a> = (usize, Op, &'a str);

fn parse_part(line: &str) -> Part {
    let mut part = [0; 4];

    let mut i = 0;
    let groups = line.bytes().group_by(|b| b.is_ascii_digit());
    for (is_num, chars) in &groups {
        if is_num {
            let num = chars.collect_vec();
            part[i] = fast_parse_u64(num.into_iter()) as i64;
            i += 1;
        }
    }

    part
}

fn parse_rule(text: &str) -> Rule {
    debug_assert_eq!(text, text.trim());
    let bytes = text.as_bytes();

    let pos = bytes.iter().position(|b| *b == b':');
    if let Some(pos) = pos {
        // Lt or Gt
        // e.g. "a<2006:qkq"
        let idx = match bytes[0] {
            b'x' => X,
            b'm' => M,
            b'a' => A,
            b's' => S,
            b => unreachable!("Unexpected part value: {b}"),
        };

        let op = match bytes[1] {
            b'<' => Lt(text[2..pos].parse().unwrap()),
            b'>' => Gt(text[2..pos].parse().unwrap()),
            b => unreachable!("Expected < or > but found {b}"),
        };

        let lbl = &text[(pos + 1)..];

        (idx, op, lbl)
    } else {
        let lbl = text;
        (DONT, Jump, lbl)
    }
}

fn parse_workflow_entry(line: &str) -> (&str, Vec<Rule>) {
    let (lbl, rest) = line.split_once('{').unwrap();

    let mut rules = vec![];

    for rule_text in rest.split([',', '}']) {
        if !rule_text.is_empty() {
            rules.push(parse_rule(rule_text));
        }
    }

    (lbl, rules)
}

fn resolve_part(workflows: &HashMap<&str, Vec<Rule>>, mut part: Part) -> bool {
    let mut lbl = "in";

    'workflows: loop {
        info!("Lbl={lbl}");
        if lbl == "A" {
            return true;
        }
        if lbl == "R" {
            return false;
        }

        for (idx, op, target) in &workflows[lbl] {
            match op {
                Jump => {
                    lbl = *target;
                    continue 'workflows;
                }
                Gt(n) => {
                    if part[*idx] > *n {
                        lbl = *target;
                        continue 'workflows;
                    }
                }
                Lt(n) => {
                    if part[*idx] < *n {
                        lbl = *target;
                        continue 'workflows;
                    }
                }
            }
        }
    }

    unreachable!()
}

// Part1 ========================================================================
#[aoc(day19, part1)]
pub fn part1(input: &str) -> i64 {
    let lines = input.lines().collect_vec();
    let empty_line = lines.iter().position(|l| l.is_empty()).unwrap();
    let (workflow_lines, part_lines) = lines.split_at(empty_line);

    info!("Processing {} workflows", workflow_lines.len());
    info!("Processing {} parts", part_lines.len());

    let workflows = workflow_lines
        .iter()
        .map(|line| parse_workflow_entry(line))
        .collect();

    part_lines
        .iter()
        .map(|line| {
            let part = parse_part(line);
            if resolve_part(&workflows, part) {
                part.into_iter().sum::<i64>()
            } else {
                0
            }
        })
        .sum()
}

// Part2 ========================================================================
#[aoc(day19, part2)]
pub fn part2(input: &str) -> i64 {
    #![allow(unused)]

    0
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    #[rstest]
    #[case("{x=787,m=2655,a=1222,s=2876}", [787,2655,1222,2876])]
    #[case("{x=1679,m=44,a=2067,s=496}", [1679,44,2067,496])]
    #[case("{x=2036,m=264,a=79,s=2244}", [2036,264,79,2244])]
    #[case("{x=2461,m=1339,a=466,s=291}", [2461,1339,466,291])]
    #[case("{x=2127,m=1623,a=2188,s=1013}", [2127,1623,2188,1013])]
    fn check_part_parsing(#[case] input: &str, #[case] expected: Part) {
        assert_eq!(parse_part(input), expected);
    }

    #[rstest]
    #[case("A", (DONT, Jump, "A"))]
    #[case("R", (DONT, Jump, "R"))]
    #[case("crn", (DONT, Jump, "crn"))]
    #[case("a<2006:qkq", (A, Lt(2006), "qkq"))]
    #[case("m>1548:A", (M, Gt(1548), "A"))]
    #[case("s<537:gd", (S, Lt(537), "gd"))]
    #[case("x<1416:A", (X, Lt(1416), "A"))]

    fn check_rule_parsing(#[case] input: &str, #[case] expected: Rule) {
        assert_eq!(parse_rule(input), expected);
    }

    const EXAMPLE_INPUT: &str = r"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[rstest]
    #[case::given(19_114, EXAMPLE_INPUT)]
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

    #[ignore]
    #[rstest]
    #[case::given(167_409_079_868_000, EXAMPLE_INPUT)]
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
