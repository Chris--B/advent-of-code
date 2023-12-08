use crate::prelude::*;

fn walk<'a>(
    directions: &'a str,
    map: &'a HashMap<&'a str, [&'a str; 2]>,
    mut here: &'a str,
) -> i64 {
    for (steps, d) in directions.chars().cycle().enumerate() {
        if here.ends_with('Z') {
            return steps as i64;
        }
        match d {
            'L' => here = map[here][0],
            'R' => here = map[here][1],
            _ => unreachable!("{steps}, {d}"),
        }
    }
    unreachable!()
}

// Part1 ========================================================================
#[aoc(day8, part1)]
pub fn part1(input: &str) -> i64 {
    let mut lines = input.lines();

    let directions = lines.next().unwrap().trim();
    let mut map: HashMap<&str, [&str; 2]> = HashMap::new();

    for node in lines.skip(1) {
        // Example line:
        //      AAA = (BBB, BBB)
        let here = &node[..][..3];
        let left = &node[7..][..3];
        let right = &node[12..][..3];

        map.insert(here, [left, right]);
    }

    walk(directions, &map, "AAA")
}

// Part2 ========================================================================
#[aoc(day8, part2)]
pub fn part2(input: &str) -> i64 {
    let mut lines = input.lines();

    let directions = lines.next().unwrap().trim();
    let mut map: HashMap<&str, [&str; 2]> = HashMap::new();

    for node in lines.skip(1) {
        // Example line:
        //      AAA = (BBB, BBB)
        let here = &node[..][..3];
        let left = &node[7..][..3];
        let right = &node[12..][..3];

        map.insert(here, [left, right]);
    }

    map.keys()
        .filter(|k| k.ends_with('A'))
        .map(|h| walk(directions, &map, h))
        .reduce(|acc, s| acc.lcm(&s))
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

    #[rstest]
    #[case::given(6, EXAMPLE_INPUT)]
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

    const EXAMPLE_INPUT_2: &str = r"
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
    #[case::given(6, EXAMPLE_INPUT_2)]
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
