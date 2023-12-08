use crate::prelude::*;

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
        // println!("{node}");
        // println!("{here} := {left}, {right}");

        map.insert(here, [left, right]);
    }

    let mut here = "AAA";

    for (steps, d) in directions.chars().cycle().enumerate() {
        if here == "ZZZ" {
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

// Part2 ========================================================================
#[aoc(day8, part2)]
pub fn part2(input: &str) -> i64 {
    unimplemented!();
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

    // #[rstest]
    // #[case::given(999_999, EXAMPLE_INPUT)]
    // #[trace]
    // fn check_ex_part_2(
    //     #[notrace]
    //     #[values(part2)]
    //     p: impl FnOnce(&str) -> i64,
    //     #[case] expected: i64,
    //     #[case] input: &str,
    // ) {
    //     let input = input.trim();
    //     assert_eq!(p(input), expected);
    // }
}
