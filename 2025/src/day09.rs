#![allow(unused)]

use ultraviolet::Vec2;

use crate::prelude::*;

// Part1 ========================================================================
#[aoc(day9, part1)]
pub fn part1(input: &str) -> i64 {
    let red_tiles: Vec<(i64, i64)> = input.i64s().tuples().collect_vec();
    let n = red_tiles.len();

    let mut area = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            let a = red_tiles[i];
            let b = red_tiles[j];
            let this_area = (1 + (b.1 - a.1).abs()) * (1 + (b.0 - a.0).abs());
            area = area.max(this_area);
        }
    }

    area
}

// Part2 ========================================================================
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Edge {
    a: IVec2,
    ai: usize,

    b: IVec2,
    bi: usize,
}

impl Edge {
    fn new(a: IVec2, ai: usize, b: IVec2, bi: usize) -> Self {
        Self { a, ai, b, bi }
    }

    fn length(&self) -> i64 {
        let d = (self.a - self.b).abs();
        d.x as i64 + d.y as i64
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.length().cmp(&other.length())
    }
}

fn build_dists_heap(verts: &[IVec2]) -> BinaryHeap<Reverse<Edge>> {
    let mut dists = BinaryHeap::with_capacity(verts.len());

    for (i, &a) in verts.iter().enumerate() {
        for (j, &b) in verts[i + 1..].iter().enumerate() {
            let j = j + i + 1;
            let diff: [i32; 2] = std::array::from_fn(|ii| (a[ii] - b[ii]).abs());
            let dist: i32 = diff.iter().sum();
            dists.push(Reverse(Edge::new(a, i, b, j)));
        }
    }

    dists
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> i64 {
    let mut verts: Vec<IVec2> = input
        .i64s()
        .tuples()
        .map(|(x, y)| [x as _, y as _].into())
        .collect_vec();

    let mut all_edges = build_dists_heap(&verts);

    let mut labels: Vec<usize> = (0..verts.len()).collect_vec();
    let mut label_0_count = 1;

    let mut edges = vec![];
    while label_0_count != labels.len() {
        let Some(Reverse(e @ Edge { a, ai, b, bi })) = all_edges.pop() else {
            break;
        };

        let cid_a = labels[ai];
        let cid_b = labels[bi];
        if cid_a != cid_b {
            edges.push(e);
            let new = usize::min(cid_a, cid_b);
            let old = usize::max(cid_a, cid_b);

            for cid in &mut labels {
                if *cid == old {
                    *cid = new;
                    if new == 0 {
                        label_0_count += 1;
                    }
                }
            }
        }
    }

    let mut area = 0;

    assert!(area < 4570351616);

    area
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

    #[rstest]
    #[case::given(50, EXAMPLE_INPUT)]
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

    // #[rstest]
    // #[case::given(24, EXAMPLE_INPUT)]
    // #[trace]
    // #[timeout(Duration::from_millis(1000))]
    // fn check_ex_part_2(
    //     #[notrace]
    //     #[values(part2)]
    //     p: impl FnOnce(&str) -> i64,
    //     #[case] expected: i64,
    //     #[case] input: &str,
    // ) {
    //     init_logging();

    //     let input = input.trim();
    //     assert_eq!(p(input), expected);
    // }
}
