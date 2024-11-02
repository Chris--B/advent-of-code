#![allow(unused)]

use crate::prelude::*;

#[derive(Copy, Clone, Debug, Defaults)]
struct State {
    #[def = "false"]
    seen: bool,
    steps: i64,
}

impl State {
    fn new() -> Self {
        Self::default()
    }

    fn from_char(c: char) -> Self {
        let steps = 0;

        if c != '#' {
            Self { seen: false, steps }
        } else {
            Self { seen: true, steps }
        }
    }
}

// Part1 ========================================================================
#[aoc(day23, part1)]
pub fn part1(input: &str) -> i64 {
    let mut trail_map = Framebuffer::parse_grid_char(input);
    trail_map.set_border_color(Some('#'));

    let mut state_map = Framebuffer::new_matching_size_with(&trail_map, |c| State::from_char(*c));

    let mut queue = PriorityQueue::new();

    let start = IVec2::new(1, trail_map.height() as i32 - 1);
    assert_eq!(trail_map[start], '.');

    let goal = IVec2::new(trail_map.width() as i32 - 2, 0);
    assert_eq!(trail_map[goal], '.');

    queue.push(start, 0);

    let mut n_iters = 0;
    while let Some((cur, _p)) = queue.pop() {
        n_iters += 1;

        let cur_state = *guard!(state_map.get_v(cur); else { continue; });
        if cur == goal {
            info!("goal state={cur_state:#?}");
            continue;
        }

        // info!(
        //     "[{}] Exploring {cur:?}: cur_state={cur_state:?}",
        //     queue.len()
        // );
        state_map[cur].seen = true;

        let mut dirs = Cardinal::ALL_NO_DIAG;
        let mut dirs: &mut [_] = &mut dirs;
        if "v><^".contains(trail_map[cur]) {
            // If this is a slope, we don't have a choice of movement
            dirs = &mut dirs[..1];
            dirs[0] = Cardinal::from_char(trail_map[cur]);
        }

        for dir in dirs.iter().copied() {
            let next = cur + dir.into();
            let next_state = State::new();

            if let Some(state) = state_map.get_mut_v(next) {
                // Need to differentiate between a path getting here after a longer path or the same path oroborousing itself
                if state.seen {
                    // See if we're better, and give up if we aren't
                    if state.steps >= next_state.steps {
                        continue;
                    }
                }

                *state = next_state;

                queue.push(next, state.steps);
            }
        }

        // for y in trail_map.range_y().rev() {
        //     print!("{y:>3} | ");
        //     for x in trail_map.range_x() {
        //         let mut c = format!("{}", trail_map[(x, y)]);

        //         if c == "#" {
        //             print!("{}", c.green());
        //         } else if state_map[(x, y)].seen {
        //             print!("{}", c.bright_blue());
        //         } else {
        //             print!("{}", c.red());
        //         }
        //     }
        //     println!();
        // }
    }

    info!("n_iters={n_iters} state_map[goal]={:#?}", state_map[goal]);
    state_map[goal].steps
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
