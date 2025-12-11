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

fn find_all_paths<'a>(
    map: &'_ HashMap<&'a str, Path<'a>>,
    from: impl IntoIterator<Item = &'a str>,
    to: impl IntoIterator<Item = &'a str>,
) -> HashSet<Vec<&'a str>> {
    let from: Vec<_> = from.into_iter().collect();
    let to: Vec<_> = to.into_iter().collect();

    let mut queue: Vec<_> = from.iter().map(|&e| vec![e]).collect();
    let mut all_paths = HashSet::new();

    while let Some(path) = queue.pop() {
        let here = *path.last().unwrap();

        if path.len() > 10 {
            continue;
        }

        if to.contains(&here) {
            // if cfg!(test) {
            //     println!("Found path: {path:?}");
            // }
            all_paths.insert(path);
            if cfg!(test) {
                println!("Found {} paths", all_paths.len());
            }
            continue;
        }

        if !map.contains_key(here) {
            println!("Cannot find {here:?}");
        }
        for node in &map[here] {
            let mut next = path.clone();
            next.push(node);

            if !all_paths.contains(&next) {
                queue.push(next);
            }
        }
    }

    println!("Found {} paths", all_paths.len());
    println!("  + from {from:?}");
    println!("  + to   {to:?}");

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

// Part2 ========================================================================
#[aoc(day11, part2)]
pub fn part2(input: &str) -> i64 {
0
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
    #[ignore]
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
