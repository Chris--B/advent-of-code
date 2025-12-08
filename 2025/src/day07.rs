#![allow(unused)]

use crate::prelude::*;

// Part1 ========================================================================
#[aoc(day7, part1)]
pub fn part1(input: &str) -> i64 {
    let mut map = Framebuffer::parse_grid_char(input);
    let mut beams = Bitset256::new();
    beams.insert(map.width() as u32 / 2);

    let mut splits = 0;

    for y in map.range_y() {
        let y = map.height() as i32 - y - 1;

        let mut next = Bitset256::new();
        for x in map.range_x() {
            let x = x as i64;
            if beams.contains(x) {
                if map[(x as i32, y)] == '^' {
                    splits += 1;
                    // println!("Found splitter at ({x}, {y})");
                    next.insert(x - 1);
                    next.insert(x + 1);
                } else {
                    next.insert(x);
                }
            }
        }
        beams = next;
    }

    splits
}

// Part2 ========================================================================
#[derive(Debug)]
struct SplitterRow {
    y: usize,
    coords: Vec<(usize, usize)>,
}

impl SplitterRow {
    fn has_splitter_at(&self, x: usize) -> bool {
        self.coords.contains(&(x, self.y))
    }
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> i64 {
    let mut rows: Vec<SplitterRow> = input
        .lines()
        .enumerate()
        .filter_map(|(y, l)| {
            if l.as_bytes().contains(&b'^') {
                Some((y, l.as_bytes()))
            } else {
                None
            }
        })
        .map(|(y, l)| {
            let coords = l
                .iter()
                .enumerate()
                .filter(|&(_x, &b)| b == b'^')
                .map(|(x, _)| (x, y))
                .collect_vec();

            SplitterRow { y, coords }
        })
        .collect_vec();

    // Add a fake row of all splitters so we can track how many paths exit
    {
        let last = rows.last().unwrap();
        let y = last.y + 1;
        let min_x = last.coords[0].0 - 1;
        let max_x = last.coords.last().unwrap().0 + 1;
        let coords = (min_x..=max_x).map(|x| (x, y)).collect_vec();
        rows.push(SplitterRow { y, coords });
    }

    let mut links: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
    debug_assert_eq!(rows[0].coords.len(), 1, "Only one starting node please");

    // Walk the list of splitters and build connection chains
    for (idx, row) in rows.iter().enumerate() {
        if idx + 1 >= rows.len() {
            // Nothing to check against, we done.
            break;
        }

        // Find splitters on the next row(s) that our splitters can hit
        for &(x, y) in &row.coords {
            if cfg!(test) {
                println!("Searching from {:?}", (x, y));
            }
            for next_x in [x - 1, x + 1] {
                'rows: for next in &rows[idx + 1..] {
                    if next.has_splitter_at(next_x) {
                        links
                            .entry((x, y)) //
                            .or_default() //
                            .push((next_x, next.y));
                        if cfg!(test) {
                            println!("  + Found splitter at {:?}", (next_x, next.y));
                        }
                        break 'rows;
                    } else {
                        if cfg!(test) {
                            println!("  + No splitter at    {:?}", (next_x, next.y));
                        }
                    }
                }
            }
        }
    }

    if cfg!(test) {
        println!("Links:");
        for (&from, to) in &links {
            println!("  +{from:?} -> {to:?}");
        }
        println!();
    }

    for &coord in &rows.last().unwrap().coords {
        let old = links.insert(coord, vec![]);
        debug_assert_eq!(old, None, "Final row isn't supposed to connect to anything");
    }

    let last_y = rows.last().unwrap().y;

    let mut paths: HashMap<(usize, usize), i64> = HashMap::new();
    paths.insert(rows[0].coords[0], 1);

    for row in &rows {
        if cfg!(test) {
            println!("Checking row y={} ({})", row.y, row.coords.len());
        }
        for &xy in &row.coords {
            if cfg!(test) {
                println!("  + Counting paths from {xy:?}");
            }
            if let Some(links_from_xy) = links.get(&xy) {
                for &into in links_from_xy {
                    if paths.contains_key(&xy) {
                        if cfg!(test) {
                            println!(
                                "    + Found {} new paths to {into:?} (from {xy:?})",
                                paths[&xy]
                            );
                        }
                        *paths.entry(into).or_default() += paths[&xy];
                    }
                }
            } else {
                // debug_assert_eq!(
                //     xy.1, last_y,
                //     "Can't find a path to {xy:?} even though it's not the final row"
                // );
            }
        }
        if cfg!(test) {
            println!();
        }
    }
    if cfg!(test) {
        println!();

        println!("Path Counts:");
        {
            let mut paths = paths.iter().collect_vec();
            paths.sort_by_key(|((x, y), _)| (*y, *x));
            for (into, count) in &paths {
                println!("  +{into:?}: {count} paths");
            }
            println!();
        }
    }

    let mut total = 0;

    for (into, count) in &paths {
        if into.1 == last_y {
            total += count;
        }
    }

    total
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

    const SMOL_INPUT: &str = r"
..S..
..^..
.^.^.
";

    const SMOL_INPUT_2: &str = r"
...S...
...^...
..^.^..
.^.^.^.
";

    const SMOL_BUT_MISSES_INPUT: &str = r"
...S..
...^..
.^....
....^.
";

    #[rstest]
    #[case::given(21, EXAMPLE_INPUT)]
    #[case::smol(3, SMOL_INPUT)]
    #[timeout(Duration::from_millis(1))]
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
    #[case::given(40, EXAMPLE_INPUT)]
    #[case::smol1(4, SMOL_INPUT)]
    #[case::smol2(1+3+3+1, SMOL_INPUT_2)]
    // #[case::smol_but_misses(2, SMOL_BUT_MISSES_INPUT)] // seems sus
    #[timeout(Duration::from_millis(100))]
    #[trace]
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
