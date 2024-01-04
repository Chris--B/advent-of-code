#![allow(unused)]

use crate::prelude::*;

use std::cmp::Ordering;
use std::f32::consts::E;
use std::fmt;

use image::Rgb;

const PATH_START: IVec2 = IVec2::new(0, 0);

fn parse(s: &str) -> Framebuffer<u8> {
    let width = s.lines().next().map(str::len).unwrap_or_default();
    let height = s.lines().count();

    let mut map = Framebuffer::new(width as u32, height as u32);

    for (y, line) in s.lines().enumerate() {
        for (x, heat_loss_byte) in line.chars().enumerate() {
            map[(x, y)] = (heat_loss_byte as u8) - b'0';
        }
    }

    map
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Path {
    heat_loss_so_far: i64,
    history: Vec<IVec2>,
    dir_history: Vec<Cardinal>,
    goal: IVec2,
}

impl Path {
    #[track_caller]
    fn pos(&self) -> IVec2 {
        if let Some(pos) = self.history.last() {
            *pos
        } else {
            PATH_START
        }
    }

    #[track_caller]
    fn dir(&self) -> Cardinal {
        if let Some(dir) = self.dir_history.last() {
            *dir
        } else {
            unreachable!()
        }
    }

    fn steps_to_goal(&self) -> i32 {
        let d = self.goal - self.pos();
        d.x.abs() + d.y.abs()
    }

    fn priority(&self) -> i64 {
        // Bigger is handled first
        -self.heat_loss_so_far
    }
}

impl fmt::Debug for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("State")
            .field("heat_loss_so_far", &self.heat_loss_so_far)
            .field("dir_history", &self.dir_history.len())
            .field("history", &self.history.len())
            .field("history.last()", &self.history.last())
            .finish()
    }
}

// Part1 ========================================================================
#[aoc(day17, part1)]
pub fn part1(input: &str) -> i64 {
    let heat_loss_map: Framebuffer<u8> = parse(input);
    let mut state_map: Framebuffer<Option<Path>> = Framebuffer::new_matching_size(&heat_loss_map);

    // We parse the map "upside down", so start is always (0, 0) and goal is always at the top right
    let start = PATH_START;
    let goal = IVec2::new(
        heat_loss_map.width() as i32 - 1,
        heat_loss_map.height() as i32 - 1,
    );

    info!("Navigating from {start:?} to {goal:?}");
    let start_state = Path {
        heat_loss_so_far: 0,
        history: vec![], // don't include the starting cell in history or heat loss
        dir_history: vec![],
        goal,
    };
    state_map[start] = Some(start_state.clone());

    let mut queue: PriorityQueue<Path, _> = PriorityQueue::new();
    queue.push(start_state.clone(), start_state.priority());

    // For saving search history
    let mut search_order: Vec<Path> = vec![];

    while let Some((cur_path, _priority)) = queue.pop() {
        info!(
            "[{n:>2}] Exploring from {cur_path:?}",
            n = search_order.len()
        );
        search_order.push(cur_path.clone());

        if cur_path.pos() == goal {
            // Stop exploring this path if we find a goal, but keep searching
            continue;
        }

        for dir in [Souð, East, West, Norð] {
            let pos = cur_path.pos() + dir.into();

            let steps_since_turn = cur_path
                .dir_history
                .iter()
                .rev()
                .take_while(|d| **d == dir)
                .count();

            if steps_since_turn >= 3 && cur_path.dir_history.len() >= 3 {
                assert_eq!(3, steps_since_turn);
                // We cannot continue in this straight line. We MUST turn, so don't explore this direction.
                continue;
            }

            let heat_loss_just_here = *heat_loss_map.get_v(pos).unwrap_or(&0) as i64;
            let heat_loss_so_far = heat_loss_just_here + cur_path.heat_loss_so_far;

            // See if we're worse than any seen-path to get here.
            // We'll only continue if we're either the first here, or better than the previous.
            if let Some(maybe_other_state) = state_map.get_v(pos) {
                if let Some(other_state) = maybe_other_state {
                    // Not the first ones here, compare notes.
                    if other_state.heat_loss_so_far <= heat_loss_so_far {
                        // The other state is better, don't explore here.
                        continue;
                    }
                }
            } else {
                // Don't explore out of bounds
                continue;
            }

            let mut history = Vec::from_iter(cur_path.history.iter().copied().chain([pos]));
            let mut dir_history = Vec::from_iter(cur_path.dir_history.iter().copied().chain([dir]));
            let mut next_path = Path {
                heat_loss_so_far,
                history,
                dir_history,
                goal,
            };

            queue.push(next_path.clone(), next_path.priority());
            state_map[pos] = Some(next_path);
        }
    }

    info!("Final={:#?}", state_map[goal]);
    if cfg!(test) {
        save_search_history(
            &heat_loss_map,
            &state_map,
            search_order,
            goal.into(),
            state_map[goal].as_ref(),
        );
    }

    let final_state = state_map[goal].as_ref().unwrap();
    assert_eq!(
        final_state.heat_loss_so_far,
        final_state
            .history
            .iter()
            .map(|pos| { heat_loss_map[pos] as i64 })
            .sum()
    );

    if cfg!(test) {
        // Print a map to mimic the example (note you need to reverse the y lines)
        heat_loss_map.print(|x, y, c| {
            if let Some(idx) = final_state
                .history
                .iter()
                .position(|p| [x, y] == p.as_array())
            {
                match final_state.dir_history[idx] {
                    Cardinal::Norð => 'v',
                    Cardinal::Souð => '^',
                    Cardinal::East => '>',
                    Cardinal::West => '<',
                    _ => unreachable!(),
                }
            } else {
                (*c + b'0') as char
            }
        });
    } else {
        // Sanity check against known wrong-answers
        assert!(
            final_state.heat_loss_so_far < 939,
            "final_state.heat_loss_so_far={}",
            final_state.heat_loss_so_far
        );
    }

    final_state.heat_loss_so_far
}

fn save_search_history(
    heat_loss_map: &Framebuffer<u8>,
    state_map: &Framebuffer<Option<Path>>,
    mut search_order: Vec<Path>,
    goal: (i32, i32),
    maybe_path: Option<&Path>,
) {
    use indicatif::ProgressIterator;

    let n = 200;
    if search_order.len() > n {
        warn!(
            "Search history is {} long. Skipping some frames.",
            search_order.len()
        );

        search_order.drain(n..);
    }

    let mut frames = vec![];
    let mut seen = HashSet::new();

    for mut state in search_order.iter().progress() {
        let mut frame: Framebuffer<[u8; 3]> =
            Framebuffer::new_with_ranges(state_map.range_x(), state_map.range_y());

        for y in state_map.range_y() {
            for x in state_map.range_x() {
                let g = (255.0 * (heat_loss_map[(x, y)] as f32) / 9.0) as u8;

                frame[(x, y)] = if seen.contains(&[x, y]) {
                    [g, 0x00, g]
                } else {
                    [g; 3]
                }
            }
        }

        for pos in &state.history {
            let [x, y] = pos.as_array();
            frame[(x, y)] = [0x80, 0x00, 0x00];
            seen.insert([x, y]);
        }

        frame[goal] = [0xff, 0xff, 0x66];
        {
            let there = state.pos();
            frame[there] = [0xFF, 0x00, 0x00];
            seen.insert(there.as_array());
        }

        let mut frame = frame.make_image(50, |rgb| Rgb(*rgb));
        // Our origin is bottom left, `image`'s is top left
        image::imageops::flip_vertical_in_place(&mut frame);
        frames.push(frame);
    }

    if let Some(state) = maybe_path {
        let mut frame: Framebuffer<[u8; 3]> =
            Framebuffer::new_with_ranges(state_map.range_x(), state_map.range_y());

        for y in state_map.range_y() {
            for x in state_map.range_x() {
                let g = (255.0 * (heat_loss_map[(x, y)] as f32) / 9.0) as u8;

                frame[(x, y)] = if seen.contains(&[x, y]) {
                    [g, 0x00, g]
                } else {
                    [g; 3]
                }
            }
        }

        for pos in &state.history {
            frame[pos] = [0xcc, 0xcc, 0x53];
        }
        frame[goal] = [0xff, 0xff, 0x66];

        let mut frame = frame.make_image(50, |rgb| Rgb(*rgb));
        // Our origin is bottom left, `image`'s is top left
        image::imageops::flip_vertical_in_place(&mut frame);
        frames.push(frame);
    }

    info!("Saving {} frames for video", frames.len());
    let dir_name = if cfg!(test) {
        "target/day17_test"
    } else {
        "target/day17"
    };
    std::fs::create_dir_all(dir_name).unwrap();

    for (num, frame) in frames.into_iter().enumerate().progress() {
        let filename = format!("{dir_name}/history_{num:>05}.png");

        frame.save(filename).unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[rstest]
    #[case::given(102, EXAMPLE_INPUT)]
    #[trace]
    // #[timeout(ms(10_000))]
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

    // #[ignore]
    // #[rstest]
    // #[case::given(999_999, EXAMPLE_INPUT)]
    // #[trace]
    // #[timeout(ms(1_000))]
    // fn check_ex_part_2(
    //     #[notrace]
    //     #[values(part2)]
    //     p: impl FnOnce(&str) -> i64,
    //     #[case] expected: i64,
    //     #[case] input: &str,
    // ) {
    //     let input = input.trim();
    //     assert_eq!(p(input), expected);
    // }
}
