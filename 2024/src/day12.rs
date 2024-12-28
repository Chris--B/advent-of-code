#![allow(unused)]

use crate::prelude::*;

// Part1 ========================================================================
#[aoc(day12, part1)]
pub fn part1(input: &str) -> i64 {
    let grid = Framebuffer::parse_grid_char(input);

    let mut state: Framebuffer<Option<usize>> = Framebuffer::new_matching_size(&grid);
    let mut seen = HashSet::new();
    let mut price: Vec<(char, i64, i64)> = vec![];
    let mut last_id = 1;

    for root in grid.iter_coords() {
        let root: IVec2 = root.into();
        if seen.contains(&root) {
            continue;
        }
        seen.insert(root);
        let id = last_id;
        last_id += 1;

        let lbl = grid[root];
        if cfg!(test) {
            println!("Flooding from {lbl:?}");
        }

        let mut area = 0;
        let mut perimeter = HashSet::new();
        let mut queue = vec![root];
        while let Some(curr) = queue.pop() {
            seen.insert(curr);
            // May have been explored already, so skip it if true
            if state[curr].is_some() {
                continue;
            }

            // Mark this as visited
            // println!("  + Visiting {:?}", curr.as_array());
            state[curr] = Some(id);
            area += 1;

            for dir in Cardinal::ALL_NO_DIAG {
                let next: IVec2 = curr + dir.into();
                if state.get(next.x as _, next.y as _).is_none() {
                    perimeter.insert((curr, dir));
                    continue;
                }

                if grid[next] == lbl {
                    queue.push(next);
                } else {
                    // This neighbors a different plot, so count it in the permimeter calculation
                    perimeter.insert((curr, dir));
                }
            }
        }

        price.push((lbl, area, perimeter.len() as i64));
        assert_eq!(id, price.len());

        if cfg!(test) {
            println!("Permimeter has {} plots", perimeter.len());
            for (p, d) in perimeter {
                println!("  + {:?}, {d:?}", p.as_array());
            }
            println!();
        }
    }

    if cfg!(test) {
        println!("Regions:");
        let mut id_counts = state.counts().into_iter().collect_vec();
        id_counts.sort();
        for (id, count) in &id_counts {
            if let Some(id) = id {
                let (c, a, p) = price[id - 1];
                println!("  + Region {id:?} {c:?} has {count} plots: {a} * {p}");
            } else {
                println!("  + Region {id:?} has {count}");
            }
        }
        println!();
    }

    price.into_iter().map(|(_c, a, p)| a * p).sum()
}

// Part2 ========================================================================
#[aoc(day12, part2)]
pub fn part2(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT_SMALL: &str = r"
AAAA
BBCD
BBCC
EEEC
";

    const EXAMPLE_INPUT: &str = r"
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

    #[rstest]
    #[case::given_small(140, EXAMPLE_INPUT_SMALL)]
    #[case::given_big(1930, EXAMPLE_INPUT)]
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
        println!("{input}");
        println!();

        assert_eq!(p(input), expected);
    }

    const EXAMPLE_AAAAAAAA_BBB: &str = r"
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
";

    const EXAMPLE_BUT_ITS_JUST_AN_E: &str = r"
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
";

    #[rstest]
    #[case::given(80, EXAMPLE_INPUT_SMALL)]
    #[case::given(368, EXAMPLE_AAAAAAAA_BBB)]
    #[case::given(236, EXAMPLE_BUT_ITS_JUST_AN_E)]
    #[case::given(1206, EXAMPLE_INPUT)]
    #[trace]
    #[ignore]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        init_logging();

        let input = input.trim();
        println!("{input}");
        println!();

        assert_eq!(p(input), expected);
    }
}
