#![allow(unused)]

use crate::prelude::*;

fn cost(c: i32) -> Option<i32> {
    if c == i32::MAX {
        None
    } else {
        Some(c)
    }
}

fn steps_to(start: IVec2, end: IVec2, map: &Framebuffer<char>) -> Framebuffer<i32> {
    let mut cost_map: Framebuffer<i32> = Framebuffer::new_matching_size(map);
    cost_map.clear(i32::MAX);
    cost_map[start] = 0;

    let mut queue = vec![start];
    while let Some(curr) = queue.pop() {
        if curr == end {
            continue;
        }

        let curr_cost = cost_map[curr];
        for next in curr.neighbors() {
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

    cost_map
}

// Part1 ========================================================================
#[aoc(day20, part1, bruteforce)]
pub fn part1_bruteforce(input: &str) -> i64 {
    let dims = if cfg!(test) {
        let m = Framebuffer::parse_grid_u8(input);
        IVec2::new(m.width() as i32, m.height() as i32)
    } else {
        IVec2::new(141, 141)
    };
    let limit = if cfg!(test) { 20 } else { 100 };

    let mut start = IVec2::zero();
    let mut end = IVec2::zero();
    let mut walls: Vec<IVec2> = vec![];
    let mut map: Framebuffer<char> = Framebuffer::parse_grid2(input, |info| match info.c {
        '#' => {
            if info.x != 0 && info.y != 0 && info.x + 1 != dims.x && info.y + 1 != dims.y {
                walls.push(IVec2::new(info.x, info.y));
            }
            '#'
        }
        'S' => {
            start = IVec2::new(info.x, info.y);
            '.'
        }
        'E' => {
            end = IVec2::new(info.x, info.y);
            '.'
        }
        '.' => '.',
        c => unreachable!("Unexpected map character: {c:?}"),
    });

    walls = walls
        .into_iter()
        .filter(|w| w.neighbors().into_iter().any(|v| map[v] == '.'))
        .collect_vec();

    println!(
        "Moving from {s:?} -> {e:?}",
        s = start.as_array(),
        e = end.as_array()
    );
    println!("Found {} interior walls", walls.len());

    if cfg!(test) {
        // for w in &walls {
        //     map[w] = '@';
        // }
        map.just_print();
    }

    let base_cost_map = steps_to(start, end, &map);
    let no_cheats_best = cost(base_cost_map[end]).expect("Map has no solutions?");
    if cfg!(test) {
        // println!("Best path without cheats: {no_cheats_best:?}");
        if input.len() == 239 {
            assert_eq!(no_cheats_best, 84);
        }
    }

    if cfg!(test) {
        base_cost_map.print(|x, y, &c| {
            if let Some(cost) = cost(c) {
                b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"[cost as usize / 2]
                    as char
            } else {
                '.'
            }
        });
    }
    let mut cheats = 0;

    for &w in &walls {
        map[w] = '.';
        let cost_map = steps_to(start, end, &map);
        let savings = no_cheats_best - cost_map[end];

        if savings >= limit {
            cheats += 1;
        }
        map[w] = '#';
    }

    cheats
}

// Part2 ========================================================================
#[aoc(day20, part2)]
pub fn part2(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

    #[rstest]
    #[case::given(5, EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1_bruteforce)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        init_logging();

        let input = input.trim();
        assert_eq!(p(input), expected);
    }

    /*
        On Example input:
            There are 32 cheats that save 50 picoseconds.
            There are 31 cheats that save 52 picoseconds.
            There are 29 cheats that save 54 picoseconds.
            There are 39 cheats that save 56 picoseconds.
            There are 25 cheats that save 58 picoseconds.
            There are 23 cheats that save 60 picoseconds.
            There are 20 cheats that save 62 picoseconds.
            There are 19 cheats that save 64 picoseconds.
            There are 12 cheats that save 66 picoseconds.
            There are 14 cheats that save 68 picoseconds.
            There are 12 cheats that save 70 picoseconds.
            There are 22 cheats that save 72 picoseconds.
            There are 4 cheats that save 74 picoseconds.
            There are 3 cheats that save 76 picoseconds
    */
    #[rstest]
    #[case::given(32 + 31 + 29 + 39 + 25 + 23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3, EXAMPLE_INPUT)]
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
