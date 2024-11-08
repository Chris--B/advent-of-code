use crate::prelude::*;

type Node = u32;
type NodeMap = &'static mut [[Node; 2]];

const fn idx_of([a, b, c]: [u8; 3]) -> u32 {
    u32::from_le_bytes([a, b, c, 0])
}

const fn is_start(idx: u32) -> bool {
    let [_a, _b, c, _z] = idx.to_le_bytes();
    c == b'A'
}

const fn is_end(idx: u32) -> bool {
    let [_a, _b, c, _z] = idx.to_le_bytes();
    c == b'Z'
}

// Allocating 48 MB buffers that are zerod does weird things on my M1, so we'll use static memory instead.
// The working theory is that if the OS has zero-pages, it's fast.
// If it does not, it has to zero them for us, which we do not need nor want.
// This shows up when trying to benchmark in hot loops
fn alloc() -> &'static mut [[Node; 2]] {
    use std::ptr::addr_of_mut;
    static mut MEM: [[Node; 2]; N] = [[0, 0]; N];

    unsafe { &mut *addr_of_mut!(MEM) }
}

const N: usize = 1 + idx_of(*b"ZZZ") as usize;

fn parse(input: &str, need_starts: bool) -> (&[u8], Vec<Node>, NodeMap) {
    let (directions, input) = input.split_once('\n').unwrap();

    const LINE_LEN: usize = 17;
    let n_lines = input.len() / LINE_LEN;
    let input = &input.as_bytes()[1..];

    info!(
        "Node map has room for {N} nodes, but we'll only use {n_lines} ({:.3}%) of them",
        100. * n_lines as f32 / N as f32
    );

    let map = alloc();
    let mut starts = vec![];

    for i in 0..n_lines {
        let line: &[u8] = &input[(i * LINE_LEN)..];
        debug!("line='{}'", std::str::from_utf8(line).unwrap());
        debug_assert!(line[0].is_ascii_alphanumeric());

        // Example line:
        //      AAA = (BBB, BBB)
        //     ^      ^    ^
        //     0      7    12
        let here: Node = idx_of([line[0], line[1], line[2]]);
        let left: Node = idx_of([line[7], line[8], line[9]]);
        let right: Node = idx_of([line[12], line[13], line[14]]);

        if need_starts && is_start(here) {
            starts.push(here);
        }

        map[here as usize] = [left, right];
    }

    info!("{} directions", directions.len());

    (directions.as_bytes(), starts, map)
}

fn walk<'a>(directions: &'a [u8], map: &'a [[Node; 2]], mut here: Node) -> i64 {
    for (steps, d) in directions.iter().cycle().enumerate() {
        if is_end(here) {
            info!("steps={steps}");
            return steps as i64;
        }

        match d {
            b'L' => here = map[here as usize][0],
            b'R' => here = map[here as usize][1],
            _ => unreachable!("{steps}, {d}"),
        }
    }

    0
}

// Part1 ========================================================================
#[aoc(day8, part1, buf)]
pub fn part1(input: &str) -> i64 {
    let (directions, _starts, map) = parse(input, false);

    walk(directions, map, idx_of(*b"AAA"))
}

// Part2 ========================================================================
#[aoc(day8, part2, buf)]
pub fn part2(input: &str) -> i64 {
    let (directions, starts, map) = parse(input, true);

    assert!(!starts.is_empty());

    // Walk all ghost 'simultaneously'
    starts
        .into_iter()
        .map(|h| walk(directions, map, h))
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
