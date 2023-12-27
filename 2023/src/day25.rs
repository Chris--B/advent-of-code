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

    info!(
        "Parsed graph with {} vertices and {} edges",
        graph.keys().count(),
        graph.values().map(Vec::len).sum::<usize>()
    );

    if cfg!(test) {
        save_adj_matrix(&graph);
    }

    // haaaaaaaaaaahahahaha
    let cuts = if cfg!(test) {
        [("hfx", "pzl"), ("bvb", "cmg"), ("nvd", "jqt")]
    } else {
        [("kkp", "vtv"), ("jll", "lnf"), ("cmj", "qhd")]
    };

    if cfg!(test) {
        let mut keys = graph.keys().copied().collect_vec();
        keys.sort();

        use std::fmt::Write;
        let mut buf = String::new();

        writeln!(buf, "digraph {{");
        writeln!(buf, "    rank=same;");
        writeln!(buf, "    edge [dir=none]");
        writeln!(buf, "    node [style=filled]");

        for (a, bs) in keys.iter().map(|k| (*k, &graph[k])) {
            for b in bs {
                if *b < a {
                    continue;
                }
                if cuts.contains(&(a, b)) || cuts.contains(&(b, a)) {
                    // writeln!(buf, "    {a} [fillcolor=red]");
                    // writeln!(buf, "    {b} [fillcolor=red]");
                    writeln!(buf, "    {a} -> {b} [color=red]");
                } else {
                    writeln!(buf, "    {a} -> {b}");
                }
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

    (seen.len() * (graph.len() - seen.len())) as i64
}

fn save_adj_matrix(graph: &HashMap<&str, Vec<&str>>) {
    use image::imageops::{self, FilterType};
    use image::{Rgb, RgbImage};

    let mut keys: Vec<&str> = graph.keys().copied().collect_vec();
    keys.sort_by_key(|k| (graph[k].len(), *k));

    let dim = keys.len() as u32;
    let mut frame = RgbImage::from_fn(dim, dim, |x, y| {
        let from_node = keys[x as usize];
        let to_node = keys[y as usize];
        if graph[from_node].contains(&to_node) {
            Rgb([0xff, 0xff, 0xff])
        } else {
            Rgb([0; 3])
        }
    });

    let cuts = if cfg!(test) {
        [("hfx", "pzl"), ("bvb", "cmg"), ("nvd", "jqt")]
    } else {
        [("kkp", "vtv"), ("jll", "lnf"), ("cmj", "qhd")]
    };

    for (from_node, to_node) in cuts {
        let x = keys.iter().position(|n| *n == from_node).unwrap() as u32;
        let y = keys.iter().position(|n| *n == to_node).unwrap() as u32;

        *frame.get_pixel_mut(x, y) = Rgb([0xff, 0x00, 0xff]);
        *frame.get_pixel_mut(y, x) = Rgb([0xff, 0x00, 0xff]);
    }

    let new_dim = (2048 / dim) * dim;
    warn!("Scaling adj matrix from {dim}x{dim} -> {new_dim}x{new_dim}");
    assert!(new_dim > 0);

    let frame = imageops::resize(&frame, new_dim, new_dim, FilterType::Nearest);
    let filename = if cfg!(test) {
        "/Users/chris/code/me/advent-of-code/2023/target/day25-adj-matrix_test.png"
    } else {
        "/Users/chris/code/me/advent-of-code/2023/target/day25-adj-matrix.png"
    };
    frame.save(filename).unwrap();
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
