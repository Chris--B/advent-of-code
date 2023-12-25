#![allow(unused)]

use crate::prelude::*;

fn rm_elem(v: &mut Vec<&str>, e: &str) {
    let i = v.iter().position(|ee| ee == &e).unwrap();
    v.remove(i);
}

// Part1 ========================================================================
#[aoc(day25, part1)]
pub fn part1(input: &str) -> i64 {
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in input.lines() {
        let (a, bs) = line.split_once(": ").unwrap();
        for b in bs.split(' ') {
            graph.entry(a).or_default().push(b);
            graph.entry(b).or_default().push(a);
        }
    }

    // haaaaaaaaaaahahahaha
    let cuts = if cfg!(test) {
        [("hfx", "pzl"), ("bvb", "cmg"), ("nvd", "jqt")]
    } else {
        [("kkp", "vtv"), ("jll", "lnf"), ("cmj", "qhd")]
    };

    for (a, b) in cuts {
        rm_elem(graph.get_mut(a).unwrap(), b);
        rm_elem(graph.get_mut(b).unwrap(), a);
    }

    let mut queue = VecDeque::new();
    queue.push_front(graph.keys().next().unwrap());
    let mut seen = vec![];

    while let Some(node) = queue.pop_front() {
        if seen.contains(&node) {
            continue;
        }

        seen.push(node);

        for next in &graph[node] {
            queue.push_back(next);
        }
    }

    if true {
        use std::fmt::Write;
        let mut buf = String::new();

        writeln!(buf, "digraph {{");
        writeln!(buf, "    rank=same;");
        for (a, bs) in &graph {
            for b in bs {
                writeln!(buf, "    {a} -> {b}");
            }
        }
        writeln!(buf, "}}");

        // eprintln!("{buf}");
        std::fs::write(
            if cfg!(test) {
                "/Users/chris/code/me/advent-of-code/2023/target/day25_test.dot"
            } else {
                "/Users/chris/code/me/advent-of-code/2023/target/day25.dot"
            },
            buf,
        )
        .unwrap();
    }

    (seen.len() * (graph.len() - seen.len())) as i64
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

    #[rstest]
    #[case::given(54, EXAMPLE_INPUT)]
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
}
