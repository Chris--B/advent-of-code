#![allow(unused)]

use crate::prelude::*;

#[derive(Copy, Clone, Debug, Defaults)]
struct State {
    #[def = "false"]
    seen: bool,
}

impl State {
    fn new(c: char) -> Self {
        if c != '#' {
            Self { seen: false }
        } else {
            State { seen: true }
        }
    }
}

// Part1 ========================================================================
#[aoc(day23, part1)]
pub fn part1(input: &str) -> i64 {
    let mut trail_map = Framebuffer::parse_grid_char(input);
    trail_map.set_border_color(Some('#'));

    let mut state_map = Framebuffer::new_matching_size_with(&trail_map, |c| State::new(*c));

    let mut queue = PriorityQueue::new();
    let start = IVec2::new(1, trail_map.height() as i32 - 1);
    assert_eq!(trail_map[start], '.');
    queue.push(start, 0);

    let mut n_iters = 0;
    while let Some((cur, _p)) = queue.pop() {
        n_iters += 1;
        let state = guard!(state_map.get_v(cur); else { continue; });
        info!("[{}] Exploring {cur:?}: state={state:?}", queue.len());

        assert!(!state_map[cur].seen);
        state_map[cur].seen = true;

        for dir in Cardinal::ALL_NO_DIAG {
            let next = cur + dir.into();

            if let Some(state) = state_map.get_mut_v(next) {
                if state.seen {
                    continue;
                }

                queue.push(next, 0);
            }
        }

        for y in trail_map.range_y().rev() {
            print!("{y:>3} | ");
            for x in trail_map.range_x() {
                let mut c = format!("{}", trail_map[(x, y)]);

                if c == "#" {
                    print!("{}", c.green());
                } else if state_map[(x, y)].seen {
                    print!("{}", c.bright_blue());
                } else {
                    print!("{}", c.red());
                }
            }
            println!();
        }
    }
    info!("n_iters={n_iters}");

    0
}

// Part2 ========================================================================
#[aoc(day23, part2)]
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
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
";

    #[rstest]
    #[case::given(94, EXAMPLE_INPUT)]
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

    #[ignore]
    #[rstest]
    #[case::given(999_999, EXAMPLE_INPUT)]
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
