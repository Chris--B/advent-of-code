use crate::prelude::*;

fn steps_to(start: IVec2, end: IVec2, map: &Framebuffer<char>) -> Option<i32> {
    let mut cost_map: Framebuffer<i32> = Framebuffer::new_matching_size(map);
    cost_map.clear(i32::MAX);
    cost_map[start] = 0;

    let mut queue = vec![start];
    while let Some(curr) = queue.pop() {
        if curr == end {
            continue;
        }

        let curr_cost = cost_map[curr];
        for dir in Cardinal::ALL_NO_DIAG {
            let next = curr + dir.into();
            let next_cost = curr_cost + 1;

            if let Some(&cost) = cost_map.get(next.x as _, next.y as _) {
                if (map[next] == '.') && (next_cost < cost) {
                    // Better deal
                    cost_map[next] = next_cost;
                    queue.push(next);
                }
            }
        }
    }

    if cost_map[end] == i32::MAX {
        None
    } else {
        Some(cost_map[end])
    }
}

fn steps_to_partial(
    start: IVec2,
    end: IVec2,
    map: &Framebuffer<char>,
    cost_map: &mut Framebuffer<i32>,
) {
    let mut seen = HashSet::new();
    let mut queue = vec![start];
    while let Some(curr) = queue.pop() {
        if curr == end {
            // break;
        }
        if seen.contains(&curr) {
            continue;
        }
        seen.insert(curr);

        let curr_cost = cost_map[curr];
        for dir in Cardinal::ALL_NO_DIAG {
            let next = curr + dir.into();
            let next_cost = curr_cost + 1;

            if let Some(&cost) = cost_map.get(next.x as _, next.y as _) {
                if (map[next] == '.') && (next_cost <= cost) {
                    cost_map[next] = next_cost;
                    queue.push(next);
                }
            }
        }
    }
}

// Part1 ========================================================================
#[aoc(day18, part1)]
pub fn part1(input: &str) -> i32 {
    let dim = if cfg!(test) { 6 } else { 70 } + 1;
    let first_n = if cfg!(test) { 12 } else { 1024 };

    let mut map = Framebuffer::new(dim as u32, dim as u32);
    map.clear('.');

    if cfg!(test) {
        println!("After the corruption:");
        map.just_print();
    }

    for (x, y) in input.i64s().map(|n| n as i32).tuples().take(first_n) {
        let y = dim - y - 1;
        map[(x, y)] = '#';
    }

    let start = IVec2::new(0, dim - 1);
    let end = IVec2::new(dim - 1, 0);

    steps_to(start, end, &map).expect("No path found?")
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

    let start = IVec2::new(0, dim - 1);
    let end = IVec2::new(dim - 1, 0);

    // Build the map fully
    let coords = input.i64s().map(|n| n as i32).tuples().collect_vec();
    for &(x, y) in &coords {
        map[(x, dim - y - 1)] = '#';
    }

    let mut cost_map: Framebuffer<i32> = Framebuffer::new_matching_size(&map);
    cost_map.clear(i32::MAX);
    cost_map[start] = 0;

    // Peel off the coordinates one-by-one until we find a path that works
    for (i, &(x, y)) in coords.iter().enumerate().rev() {
        map[(x, dim - y - 1)] = '.';
        steps_to_partial(start, end, &map, &mut cost_map);

        if cost_map[end] != i32::MAX {
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
        p: impl FnOnce(&str) -> i32,
        #[case] expected: i32,
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
