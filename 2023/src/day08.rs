use crate::prelude::*;

type Node = [u8; 3];
type NodeMap = HashMap<Node, [Node; 2]>;

fn parse(input: &str) -> (&[u8], NodeMap) {
    let (directions, input) = input.split_once('\n').unwrap();

    const LINE_LEN: usize = 17;
    let n_lines = input.len() / LINE_LEN;
    let input = &input.as_bytes()[1..];

    let mut map = HashMap::with_capacity(n_lines);
    for i in 0..n_lines {
        let line: &[u8] = &input[(i * LINE_LEN)..];

        // Example line:
        //      AAA = (BBB, BBB)
        //     ^      ^    ^
        //     0      7    12
        let here: Node = [line[0], line[1], line[2]];
        let left: Node = [line[7], line[8], line[9]];
        let right: Node = [line[12], line[13], line[14]];

        map.insert(here, [left, right]);
    }

    (directions.as_bytes(), map)
}

fn walk<'a>(directions: &'a [u8], map: &'a NodeMap, mut here: &'a Node) -> i64 {
    for (steps, d) in directions.iter().cycle().enumerate() {
        if here.ends_with(&[b'Z']) {
            return steps as i64;
        }

        match d {
            b'L' => here = &map[here][0],
            b'R' => here = &map[here][1],
            _ => unreachable!("{steps}, {d}"),
        }
    }

    0
}

// Part1 ========================================================================
#[aoc(day8, part1)]
pub fn part1(input: &str) -> i64 {
    let (directions, map) = parse(input);

    walk(directions, &map, b"AAA")
}

// Part2 ========================================================================
#[aoc(day8, part2)]
pub fn part2(input: &str) -> i64 {
    let (directions, map) = parse(input);

    // Walk all ghost 'simultaneously'
    map.keys()
        .filter(|k| k.ends_with(&[b'A']))
        .map(|h| walk(directions, &map, h))
        .reduce(|acc, s| acc.lcm(&s))
        .unwrap_or(0)
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT_P1: &str = r"
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

    #[rstest]
    #[case::given(6, EXAMPLE_INPUT_P1)]
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

    const EXAMPLE_INPUT_P2: &str = r"
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";

    #[rstest]
    #[case::given(6, EXAMPLE_INPUT_P2)]
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
