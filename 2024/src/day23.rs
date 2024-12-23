#![allow(unused)]

use crate::prelude::*;

const N: usize = 26 * 26;
struct ComputePairSet {
    buf: [HashSet<usize>; N],
}

const fn idx(s: &[u8]) -> usize {
    26 * (s[0] - b'a') as usize + (s[1] - b'a') as usize
}

fn disp(idx: usize) -> String {
    let a = (idx / 26) as u8;
    let b = (idx % 26) as u8;
    format!("{}{}", (a + b'a') as char, (b + b'a') as char)
}

impl ComputePairSet {
    pub fn new() -> Self {
        Self {
            buf: std::array::from_fn(|_| HashSet::new()),
        }
    }

    pub fn insert(&mut self, a: &[u8], b: &[u8]) {
        let mut a = idx(a);
        let mut b = idx(b);
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }

        self.buf[a].insert(b);
    }

    pub fn contains(&self, a: &[u8], b: &[u8]) -> bool {
        let mut a = idx(a);
        let mut b = idx(b);

        self.contains_idx(a, b)
    }

    pub fn contains_idx(&self, mut a: usize, mut b: usize) -> bool {
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }

        self.buf[a].contains(&b)
    }

    pub fn connected_to_idx(&self, a: usize) -> impl Iterator<Item = usize> + 'static {
        self.buf[a].iter().copied().collect_vec().into_iter()
    }
}

// Part1 ========================================================================
#[aoc(day23, part1)]
pub fn part1(input: &str) -> i64 {
    let mut pairs = ComputePairSet::new();

    for line in input.lines() {
        let (left, right) = line.split_once('-').unwrap();
        pairs.insert(left.as_bytes(), right.as_bytes());
        debug_assert!(pairs.contains(left.as_bytes(), right.as_bytes()));
        debug_assert!(pairs.contains(right.as_bytes(), left.as_bytes()));
    }

    let mut triplets = HashSet::new();
    for a in idx(b"aa")..=idx(b"zz") {
        for b in idx(b"aa")..a {
            for c in idx(b"ta")..=idx(b"tz") {
                if pairs.contains_idx(a, b) && pairs.contains_idx(b, c) && pairs.contains_idx(c, a)
                {
                    let mut triplet = [a, b, c];
                    triplet.sort();
                    triplets.insert(triplet);
                    // println!("{}, {}, {}", disp(a), disp(b), disp(c));
                }
            }
        }
    }

    triplets.len() as i64
}

// Part2 ========================================================================
fn println_set_sorted<T: std::fmt::Display + std::cmp::Ord>(set: &HashSet<T>) {
    let mut list = set.iter().collect_vec();
    list.sort();

    if !list.is_empty() {
        print!("{}", list[0]);
    }

    for e in list.into_iter().skip(1) {
        print!(", {e}");
    }
    println!();
}

#[aoc(day23, part2)]
pub fn part2(input: &str) -> String {
    let mut pairs: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut computers: HashSet<&str> = HashSet::new();

    for line in input.lines() {
        let (left, right) = line.split_once('-').unwrap();
        pairs.entry(left).or_default().push(right);
        pairs.entry(right).or_default().push(left);

        computers.insert(left);
        computers.insert(right);
    }

    if cfg!(test) {
        println!(
            "Found {} unique pairs",
            pairs.values().map(|conns| conns.len()).sum::<usize>() / 2
        );
        println!("Found {} unique computers", computers.len());
    }

    let mut best_len = 0;
    let mut best_set = HashSet::new();

    let mut seen: HashSet<&str> = HashSet::new();
    for &root in &computers {
        if seen.contains(root) {
            continue;
        }

        let mut connected: HashSet<&str> = pairs[root].iter().copied().collect();
        connected.insert(root);

        // Check each computer that root connects to.
        // We need the intersection of each of these lists with themselves.
        for &other in &pairs[root] {
            if !connected.contains(other) {
                continue;
            }
            // Note: Connection lists don't contain themselves, so make sure we keep `other` here too!
            connected.retain(|&e| (e == other) || pairs[other].contains(&e));
        }

        // Mark this whole set connected now that we've completed it.
        for &other in &connected {
            seen.insert(other);
        }

        if cfg!(test) {
            print!("  + {root:?} w/ {:>2} computers: ", connected.len());
            println_set_sorted(&connected);
        }

        // Track our best one yet!
        if connected.len() > best_len {
            best_len = connected.len();
            best_set = connected;
        }
    }

    // Defer collecting and sorting until the end, when we know we have our winner.
    let mut best = best_set.into_iter().collect_vec();
    best.sort();
    best.join(",")
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
";

    #[rstest]
    #[case::given(7, EXAMPLE_INPUT)]
    #[trace]
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
    #[case::given("co,de,ka,ta", EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2)]
        p: impl FnOnce(&str) -> String,
        #[case] expected: String,
        #[case] input: &str,
    ) {
        init_logging();

        let input = input.trim();
        assert_eq!(p(input), expected);
    }
}
