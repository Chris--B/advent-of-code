use crate::prelude::*;

type Node = [u8; 3];
type NodeMap = fnv::FnvHashMap<Node, [Node; 2]>;

fn parse(input: &str) -> (&[u8], NodeMap) {
    let (directions, input) = input.split_once('\n').unwrap();

    const LINE_LEN: usize = 17;
    let n_lines = input.len() / LINE_LEN;
    let input = &input.as_bytes()[1..];

    let mut map = fnv::FnvHashMap::with_capacity_and_hasher(n_lines, Default::default());
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
        if here.ends_with(b"Z") {
            // dbg!(steps);
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

    if input.lines().count() > 20 && log_enabled!(Info) {
        let filename = "./input.dot";
        info!(
            "Writing out dot file to {filename} w/ {} nodes. Use graphviz to render:\n\tdot -Tpng {filename} -o out.png ",
            map.len() * 2
        );

        let mut lines: Vec<String> = vec![
            "digraph world {".into(),                               // .
            "".into(),                                              // .
            "    rank = same;".into(),                              // .
            "    node [fillcolor=white, style=\"filled\"];".into(), //.
            "".into(),                                              // .
        ];

        for (from, [right, left]) in &map {
            let from = std::str::from_utf8(from).unwrap();
            let right = std::str::from_utf8(right).unwrap();
            let left = std::str::from_utf8(left).unwrap();

            if from == "AAA" {
                lines.push(format!("    \"{from}\" [shape=hexagon, fillcolor=red];"));
                info!("{}", lines.last().unwrap());
            } else if from.ends_with('A') || from.ends_with('Z') {
                lines.push(format!(
                    "    \"{from}\" [shape=hexagon, fillcolor=\"#d9e7ee\"];"
                ));
                info!("{}", lines.last().unwrap());
            }
            lines.push(format!("    \"{from}\" -> {{ \"{left}\"; \"{right}\"; }}"));
        }
        lines.push("}".into());

        std::fs::write(filename, lines.join("\n")).unwrap();
    }

    // Walk all ghost 'simultaneously'
    map.keys()
        .filter(|k| k.ends_with(b"A"))
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
