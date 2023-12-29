#![allow(unused)]

use crate::prelude::*;

use std::cmp::Ordering;
use std::fmt;

use image::Rgb;

fn parse(s: &str) -> Framebuffer<u8> {
    let width = s.lines().next().map(str::len).unwrap_or_default();
    let height = s.lines().count();

    let mut map = Framebuffer::new(width as u32, height as u32);

    for (y, line) in s.lines().enumerate() {
        let y = height - y - 1;
        for (x, heat_loss_byte) in line.chars().enumerate() {
            map[(x, y)] = (heat_loss_byte as u8) - b'0';
        }
    }

    map
}

#[derive(Clone, PartialEq, Eq)]
struct State {
    pos: IVec2,
    heat_loss_so_far: i64,
    last_dir: Cardinal,
    steps_since_turn: i64,
    path: Vec<State>,
    goal: IVec2,
}

impl State {
    fn steps_to_goal(&self) -> i32 {
        let d = self.pos - self.goal;
        d.x.abs() + d.y.abs()
    }

    fn clone_without_path(&self) -> Self {
        Self {
            pos: self.pos,
            heat_loss_so_far: self.heat_loss_so_far,
            last_dir: self.last_dir,
            steps_since_turn: self.steps_since_turn,
            path: vec![],
            goal: self.goal,
        }
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // NOTE: BinaryHeap is a max-heap, so this is REVERSED
        // other.heat_loss_so_far.cmp(&self.heat_loss_so_far)
        let (ox, oy) = other.pos.into();
        let (sx, sy) = self.pos.into();
        (13 * oy + ox).cmp(&(13 * sy + sx))
        // other.steps_to_goal().cmp(&self.steps_to_goal())
        // (other.steps_to_goal(), other.heat_loss_so_far)
        //     .cmp(&(self.steps_to_goal(), self.heat_loss_so_far))
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("State")
            .field("pos", &self.pos)
            .field("heat_loss_so_far", &self.heat_loss_so_far)
            .field("last_dir", &self.last_dir)
            .field("steps_since_turn", &self.steps_since_turn)
            // .field("path", &self.path)
            .field("path.len()", &self.path.len())
            .finish()
    }
}

// Part1 ========================================================================
#[aoc(day17, part1)]
pub fn part1(input: &str) -> i64 {
    let heat_loss_map: Framebuffer<u8> = parse(input);
    let mut state_map: Framebuffer<Option<State>> = Framebuffer::new_matching_size(&heat_loss_map);

    let mut queue: BinaryHeap<State> = BinaryHeap::new();

    let starting_pos = IVec2::new(0, heat_loss_map.height() as i32 - 1);
    let goal = IVec2::new(heat_loss_map.width() as i32 - 1, 0);

    info!("Navigating from {starting_pos:?} to {goal:?}");

    {
        let starting_state = State {
            pos: starting_pos,
            heat_loss_so_far: 0,
            last_dir: Norð,
            steps_since_turn: 0,
            path: vec![],
            goal,
        };
        // state_map[starting_pos] = Some(starting_state.clone());
        queue.push(starting_state);
    }

    let mut search_order: Vec<State> = vec![];

    while let Some(state) = queue.pop() {
        info!("[{:>2}] Exploring from {state:?}", queue.len());
        search_order.push(state.clone());

        // If this is worse than something we've alreay done, bail
        if let Some(ref prev_state) = state_map[state.pos] {
            assert_eq!(state.pos, prev_state.pos);

            if state.heat_loss_so_far >= prev_state.heat_loss_so_far {
                // Some path here already exists and is better, give up
                continue;
            }
        }
        // assert_eq!(state_map[state.pos], None);
        state_map[state.pos] = Some(state.clone_without_path());

        if state.pos == goal {
            // We'll retrieve this info after this loop from `state_map`
            info!("Reached the goal! state={state:?}");
            continue;
        }

        if state.path.len() >= 8 {
            // break;
        }

        for dir in [Norð, Souð, East, West] {
            let pos = state.pos + dir.into();

            if state.path.len() >= 3 {
                // See if we're been moving in a straight line too long
                if state.path[(state.path.len() - 3)..]
                    .iter()
                    .all(|s| s.last_dir == dir)
                {
                    // We cannot continue in this straight line. We MUST turn, so don't explore this direction.
                    continue;
                }
            }

            let last_dir = dir;
            let steps_since_turn = if dir == state.last_dir {
                state.steps_since_turn + 1
            } else {
                1
            };

            let heat_loss_so_far = if let Some(heat_loss) = heat_loss_map.get_v(pos) {
                state.heat_loss_so_far + *heat_loss as i64
            } else {
                // Out of bounds? Don't search it.
                continue;
            };

            let mut path = state
                .path
                .iter()
                .map(|s| s.clone_without_path())
                .collect_vec();
            path.push(state.clone_without_path());

            let mut next_state = State {
                pos,
                heat_loss_so_far,
                last_dir,
                steps_since_turn,
                path,
                goal,
            };

            queue.push(next_state);
        }
    }

    info!("Final {:#?}", state_map[goal]);
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

    final_state.heat_loss_so_far
}

// Part2 ========================================================================
#[aoc(day17, part2)]
pub fn part2(input: &str) -> i64 {
    0
}

fn save_search_history(
    heat_loss_map: &Framebuffer<u8>,
    state_map: &Framebuffer<Option<State>>,
    search_order: Vec<State>,
    goal: (i32, i32),
    maybe_path: Option<&State>,
) {
    use indicatif::ProgressIterator;

    if search_order.len() > 99_999 {
        warn!(
            "Search history is {} long. Skipping the video.",
            search_order.len()
        );
        return;
    }

    let mut frames = vec![];
    let mut seen = HashSet::new();

    for mut state in search_order.into_iter().progress() {
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

        if let Some(State { pos: there, .. }) = state.path.pop() {
            frame[there] = [0xFF, 0x00, 0x00];
            seen.insert(there.as_array());
        }

        for State { pos, .. } in state.path {
            let (x, y) = pos.into();
            frame[(x, y)] = [0x80, 0x00, 0x00];
            seen.insert([x, y]);
        }

        frame[goal] = [0xff, 0xff, 0x66];

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

        for State { pos, .. } in &state.path {
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
        "taget/day17"
    };
    std::fs::create_dir_all(dir_name).unwrap();

    for (num, frame) in frames.into_iter().enumerate().progress() {
        let filename = format!("{dir_name}/history_{num:>05}.png");
        // info!("Saving frame to {filename}");
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

    #[test]
    fn check_state_ordering() {
        let mut a = State {
            pos: IVec2::zero(),
            heat_loss_so_far: 0,
            last_dir: Norð,
            steps_since_turn: 0,
            path: vec![],
            goal: IVec2::zero(),
        };
        let mut b = a.clone();

        a.heat_loss_so_far = 1;
        b.heat_loss_so_far = 2;

        let mut q = BinaryHeap::new();
        q.push(a.clone());
        q.push(b);

        assert_eq!(
            q.pop().unwrap(),
            a,
            "Expected smallest thing first, but it wasn't"
        );
    }

    #[rstest]
    #[case::given(102, EXAMPLE_INPUT)]
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
