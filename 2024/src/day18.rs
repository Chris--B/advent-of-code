use crate::prelude::*;

// Part1 ========================================================================
#[aoc(day18, part1)]
pub fn part1(input: &str) -> i64 {
    let dim = if cfg!(test) { 6 } else { 70 } + 1;
    let first_n = if cfg!(test) { 12 } else { 1024 };

    let mut map = Framebuffer::new(dim as u32, dim as u32);
    map.clear('.');
    map.set_border_color(Some('#'));

    for (x, y) in input.i64s().map(|n| n as i32).tuples().take(first_n) {
        map[(x, y)] = '#';
    }

    let mut graph = AocGridGraph::new(map);
    let start = IVec2::new(0, 0);
    let end = IVec2::new(dim - 1, dim - 1);

    dijkstra(&mut graph, start, Some(end)).expect("No path found?")
}

// Part2 ========================================================================
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Coord((i32, i32));

impl std::fmt::Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

#[aoc(day18, part2)]
pub fn part2(input: &str) -> Coord {
    let dim = if cfg!(test) { 6 } else { 70 } + 1;

    let mut map = Framebuffer::new(dim as u32, dim as u32);
    map.clear('.');
    map.set_border_color(Some('#'));

    // Build the map fully
    let coords = input.i64s().map(|n| n as i32).tuples().collect_vec();
    for &(x, y) in &coords {
        map[(x, y)] = '#';
    }

    if cfg!(test) {
        map.just_print();
    }

    let mut graph = AocGridGraph::new(map);
    let start = IVec2::new(0, 0);
    let end = IVec2::new(dim - 1, dim - 1);
    dijkstra(&mut graph, start, Some(end));

    // Peel off the coordinates one-by-one until we find a path that works
    for (i, &(x, y)) in coords.iter().enumerate().rev() {
        graph.map[(x, y)] = '.';

        // Check the surrounding area that we just removed.
        // If we can find a neighbor that was used previously, we'll use that to resume our search.
        // If we cannot find such a path, then we can never reach this removed block and
        // we can skip it wholesale!
        if let Some(resume_point) = graph
            .neighbors(&IVec2::new(x, y))
            // Note: We need to filter out unreachable areas here.
            .filter(|&n| graph.distance_get(n).is_some())
            .min_by_key(|&n| graph.distance_get(n))
        {
            dijkstra_resume(&mut graph, resume_point, Some(end));
        }

        if graph.dist[end] != i64::MAX {
            // This means blocking coords[i] prevents a valid path.
            return Coord(coords[i]);
        }
    }

    unreachable!("We never found a path?")
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";

    #[rstest]
    #[case::given(22, EXAMPLE_INPUT)]
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
    #[case::given((6,1), EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2)]
        p: impl FnOnce(&str) -> Coord,
        #[case] expected: (i32, i32),
        #[case] input: &str,
    ) {
        init_logging();

        let input = input.trim();
        assert_eq!(p(input), Coord(expected));
    }
}
