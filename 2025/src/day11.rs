#![allow(unused)]

use memchr::arch::all;

use crate::prelude::*;

type Path<'a> = Vec<&'a str>;

fn parse(input: &'_ str) -> HashMap<&'_ str, Path<'_>> {
    let mut map: HashMap<&str, Path> = HashMap::new();

    for line in input.lines() {
        let (device, outputs) = line.split_once(": ").unwrap();
        let outputs = outputs.split_whitespace().collect_vec();
        if cfg!(test) {
            println!("{device} -> {outputs:?}");
        }
        map.insert(device, outputs);
    }

    map.insert("out", vec![]);

    map
}

fn invert<'a>(map: &'_ HashMap<&'a str, Path<'a>>) -> HashMap<&'a str, Path<'a>> {
    let mut inv_map: HashMap<&str, Path> = HashMap::new();

    for (&from, tos) in map.iter() {
        for to in tos {
            inv_map.entry(to).or_default().push(from);
        }
    }

    inv_map.insert("svr", vec![]);

    inv_map
}

fn reachable_from<'a>(
    name: &str,
    map: &'_ HashMap<&'a str, Path<'a>>,
    from: impl IntoIterator<Item = &'a str>,
) -> HashSet<&'a str> {
    let mut seen = HashSet::new();
    let from = from.into_iter().collect_vec();

    if cfg!(test) {
        println!("[{name}] Nodes reachable from {from:?}");
    }

    let mut queue = from.clone();
    while let Some(curr) = queue.pop() {
        seen.insert(curr);
        for &to in &map[curr] {
            if !seen.contains(to) {
                queue.push(to);
            }
        }
    }

    if cfg!(test) {
        println!("  + Found {} nodes", seen.len());
        println!();
    }

    seen
}

fn find_all_paths<'a>(
    map: &'_ HashMap<&'a str, Path<'a>>,
    from: impl IntoIterator<Item = &'a str>,
    to: impl IntoIterator<Item = &'a str>,
) -> HashSet<Vec<&'a str>> {
    let from: Vec<_> = from.into_iter().collect();
    let to: Vec<_> = to.into_iter().collect();

    let mut queue: Vec<_> = from.iter().map(|&e| vec![e]).collect();
    let mut all_paths = HashSet::new();

    if cfg!(test) {
        println!("Searching for all paths");
        println!("  + from {from:?}");
        println!("  + to   {to:?}");
    }

    while let Some(path) = queue.pop() {
        let here = *path.last().unwrap();

        if to.contains(&here) {
            // if cfg!(test) {
            //     println!("Found path: {path:?}");
            // }
            all_paths.insert(path);
            continue;
        }

        if !map.contains_key(here) {
            if cfg!(test) {
                println!("Cannot find {here:?}");
            }
            continue;
        }
        for node in &map[here] {
            if !map.contains_key(node) {
                continue;
            }
            let mut next = path.clone();
            next.push(node);

            if !all_paths.contains(&next) {
                queue.push(next);
            }
        }
    }

    if cfg!(test) {
        println!("  + {} paths", all_paths.len());
        println!();
    }

    all_paths
}

// Part1 ========================================================================
#[aoc(day11, part1)]
pub fn part1(input: &str) -> i64 {
    let map = parse(input);

    let mut all_paths = find_all_paths(&map, ["you"], ["out"]);

    if cfg!(test) {
        println!("Found {} paths", all_paths.len());

        for path in &all_paths {
            println!("  + {path:?}");
        }
    }

    all_paths.len() as i64
}

fn write_dot(filename: &str, map: &HashMap<&'_ str, Path<'_>>) {
    let mut lines: Vec<String> = vec![
        "digraph world {".into(),                               // .
        "".into(),                                              // .
        "    rank = same;".into(),                              // .
        "    node [fillcolor=white, style=\"filled\"];".into(), //.
        "    svr [fillcolor=red];".into(),                      // .
        "    dac [fillcolor=red];".into(),                      // .
        "    fft [fillcolor=red];".into(),                      // .
        "    out [fillcolor=red];".into(),                      // .
        "".into(),                                              // .
    ];

    for (from, tos) in map {
        for to in tos {
            lines.push(format!("    \"{from}\" -> {{ \"{to}\"; }}"));
        }
    }
    lines.push("}".into());

    std::fs::write(filename, lines.join("\n")).unwrap();
}

// Part2 ========================================================================
#[aoc(day11, part2)]
pub fn part2(input: &str) -> i64 {
    let fwd = parse(input);
    let bak = invert(&fwd);

    let reaches_dac = reachable_from("forward", &fwd, ["dac"]);
    let dac_reachs = reachable_from("backward", &bak, ["dac"]);
    let good_dac: HashSet<_> = reaches_dac.union(&dac_reachs).copied().collect();

    let reaches_fft = reachable_from("forward", &fwd, ["fft"]);
    let fft_reachs = reachable_from("backward", &bak, ["fft"]);
    let good_fft: HashSet<_> = reaches_fft.union(&fft_reachs).copied().collect();

    let good: HashSet<&str> = good_dac.intersection(&good_fft).copied().collect();

    let fwd: HashMap<&str, _> = fwd
        .into_iter()
        .filter(|(node, _tos)| good.contains(node))
        .collect();
    let bak: HashMap<&str, _> = bak
        .into_iter()
        .filter(|(node, _tos)| good.contains(node))
        .collect();

    if cfg!(test) {
        println!("Reduced to {} nodes", fwd.len());
    }

    print!("Searching sub-graph: svr -> fft");
    let svr_to_fft = find_all_paths(&fwd, ["svr"], ["fft"]);
    println!(" {} nodes", svr_to_fft.len());

    print!("Searching sub-graph: fft -> dac");
    let fft_to_dac = find_all_paths(&fwd, ["fft"], ["dac"]);
    println!(" {} nodes", fft_to_dac.len());

    print!("Searching sub-graph: out -> dac");
    let dac_to_out = find_all_paths(&bak, ["out"], ["dac"]);
    println!(" {} nodes", dac_to_out.len());

    println!();

    (svr_to_fft.len() as i64) * (fft_to_dac.len() as i64) * (dac_to_out.len() as i64)
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT_P1: &str = r"
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

    const EXAMPLE_INPUT_P2: &str = r"
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";

    #[rstest]
    #[case::given(5, EXAMPLE_INPUT_P1)]
    #[trace]
    #[timeout(Duration::from_millis(100))]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        init_logging();

        let input = input.trim();
        assert_eq!(p(input), expected);
    }

    #[rstest]
    #[case::given(2, EXAMPLE_INPUT_P2)]
    #[trace]
    #[timeout(Duration::from_millis(100))]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        init_logging();

        let input = input.trim();
        assert_eq!(p(input), expected);
    }
}
