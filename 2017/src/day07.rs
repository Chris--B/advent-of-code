#![allow(unused)]

use crate::prelude::*;

#[derive(Copy, Clone, Debug)]
struct Prog<'a> {
    name: &'a str,
    weight: i64,
}

// Part1 ========================================================================
#[aoc(day7, part1)]
pub fn part1(input: &str) -> String {
    // usize -> Prog
    let mut tree: Vec<Prog> = vec![];

    for line in input.lines() {
        let (name, rest) = line.split_once(" (").unwrap();
        let (weight, _) = rest.split_once(")").unwrap();
        let weight = weight.parse().unwrap();
        let i = tree.len();
        tree.push(Prog { name, weight });
    }

    // &str -> usize (-> Prog, `tree`)
    let ids: HashMap<&str, usize> = tree
        .iter()
        .enumerate()
        .map(|(i, prog)| (prog.name, i))
        .collect();

    // usize -> &[usize] (-> &[Prog], `tree`)
    //
    // Standing on refers to the prog below them. This prog is unique for each prog.
    let mut standing_on: Vec<usize> = vec![usize::MAX; tree.len()];
    for line in input.lines() {
        let (name, rest) = line.split_once(" (").unwrap();
        let id = ids[name];
        if let Some((_, list)) = rest.split_once("->") {
            for up_name in list.split(",") {
                let up = ids[up_name.trim()];
                debug_assert_eq!(standing_on[up], usize::MAX);
                standing_on[up] = id;
            }
        }
    }

    if cfg!(debug_assertions) {
        assert!(!tree.is_empty());
        assert_eq!(
            1,
            standing_on.iter().filter(|&&id| id >= tree.len()).count()
        );
    }

    let root_id: usize = standing_on.iter().position(|&id| id >= tree.len()).unwrap();
    tree[root_id].name.to_string()
}

// Part2 ========================================================================
#[aoc(day7, part2)]
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
pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)
";

    #[rstest]
    #[case::given("tknk", EXAMPLE_INPUT)]
    #[trace]
    #[timeout(Duration::from_millis(1_500))]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1)]
        p: impl FnOnce(&str) -> String,
        #[case] expected: impl ToString,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(input), expected.to_string());
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
