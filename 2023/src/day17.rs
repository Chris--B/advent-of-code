#![allow(unused)]

use crate::prelude::*;

use std::cmp::Ordering;
use std::f32::consts::E;
use std::fmt;

use image::{imageops, Rgb, RgbImage};

const START_POS: IVec2 = IVec2::new(0, 0);

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

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
struct State {
    cost: i64,
    pos: IVec2,
    vel: IVec2,
    steps_since_turn: i8,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Key(IVec2, IVec2, i8);

impl State {
    fn new(cost: i64) -> Self {
        Self {
            cost,
            pos: START_POS,
            vel: IVec2::zero(),
            steps_since_turn: 3,
        }
    }

    fn bad() -> Self {
        Self::new(i64::MAX)
    }

    fn priority(&self) -> i64 {
        assert!(
            self.cost < 1_000,
            "cost={} which is weirdly high",
            self.cost
        );
        -self.cost
    }

    fn key(self) -> Key {
        let Self {
            cost: _,
            pos,
            vel,
            steps_since_turn,
        } = self;
        Key(pos, vel, steps_since_turn)
    }
}

impl Default for State {
    fn default() -> Self {
        Self::bad()
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("State")
            .field("steps_since_turn", &self.steps_since_turn)
            .field("cost", &self.cost)
            .field("pos", &self.pos.as_array())
            .field("vel", &self.vel.as_array())
            .finish()
    }
}

// Part1 ========================================================================
fn has_illegal_runs(path: &[IVec2]) -> bool {
    for w in path.windows(4) {
        let d = w[0] - w[3];
        if d.x.abs() + d.y.abs() >= 4 {
            return true;
        }
    }

    false
}

struct Context {
    heat_loss_map: Framebuffer<u8>,
    state_seen: HashMap<Key, i64>,
    history_map: HashMap<Key, Vec<IVec2>>,
    queue: PriorityQueue<State, i64>,
}

impl Context {
    fn explore_in_direction(
        &mut self,
        mut state: State,
        vel: IVec2,
        steps_since_turn: i8,
    ) -> Option<()> {
        assert!(vel != IVec2::zero());
        assert!(state.vel != IVec2::zero());
        assert!(state.steps_since_turn <= 3);
        assert!(steps_since_turn <= 3);

        let prev = state.key();

        state.pos += vel;
        state.vel = vel;
        state.cost += *self.heat_loss_map.get_v(state.pos)? as i64;
        state.steps_since_turn = steps_since_turn;

        let next = state.key();

        if let Some(cost) = self.state_seen.get_mut(&next) {
            // unreachable!();
        } else {
            // We haven't seen this state before. Save it now so we can explore from it.
            self.state_seen.insert(next, state.cost);
            self.queue.push(state, state.priority());

            let mut history: Vec<IVec2> = self.history_map[&prev].clone();
            history.push(state.pos);
            self.history_map.insert(next, history);
        }

        Some(())
    }
}

#[aoc(day17, part1)]
pub fn part1(input: &str) -> i64 {
    let heat_loss_map: Framebuffer<u8> = parse(input);

    let mut ctx = Context {
        state_seen: HashMap::new(),
        history_map: HashMap::new(),
        queue: PriorityQueue::new(),
        heat_loss_map,
    };

    // We parse the map "upside down", so start is always (0, 0) and goal is always at the "top right"
    let start = START_POS;
    let goal = IVec2::new(
        ctx.heat_loss_map.width() as i32 - 1,
        ctx.heat_loss_map.height() as i32 - 1,
    );

    {
        let mut s = State::bad();
        s.cost = 0;
        s.steps_since_turn = 1;

        // Go "right"
        s.vel = IVec2::new(1, 0);
        ctx.state_seen.insert(s.key(), 0);
        ctx.history_map.insert(s.key(), vec![]);
        ctx.queue.push(s, s.priority());

        // Go "up"
        s.vel = IVec2::new(0, 1);
        ctx.state_seen.insert(s.key(), 0);
        ctx.history_map.insert(s.key(), vec![]);
        ctx.queue.push(s, s.priority());
    }

    info!("Navigating from {start:?} to {goal:?}");
    let mut search_order: Vec<State> = vec![];

    while let Some((state, _priority)) = ctx.queue.pop() {
        // info!(
        //     "[q={l:>3}, p={_priority}] Checking state={state:?}",
        //     l = ctx.queue.len()
        // );
        assert!(state.vel != IVec2::zero());

        search_order.push(state);

        if state.pos == goal {
            info!("Found final state: {state:#?}");
            break;
        }

        // Turn Left, resetting steps_since_turn
        ctx.explore_in_direction(state, left_90(state.vel), 0);

        // Turn Right, resetting steps_since_turn
        ctx.explore_in_direction(state, right_90(state.vel), 0);

        // Step forward, if we haven't hit speed
        if state.steps_since_turn < 3 {
            ctx.explore_in_direction(state, state.vel, state.steps_since_turn + 1);
        }
    }

    // if log_enabled!(Info) && ctx.heat_loss_map.width() == 9 {
    //     save_search_history(&ctx, search_order, goal);
    // }

    let all_goal_paths: Vec<(&Key, &Vec<IVec2>)> = ctx
        .history_map
        .iter()
        .filter(|(key, history)| key.0 == goal)
        .collect_vec();

    if !all_goal_paths.is_empty() {
        let (key, best_goal_path) = all_goal_paths
            .iter()
            .min_by_key(|(key, _history)| dbg!(ctx.state_seen[key]))
            .unwrap();
        for p in *best_goal_path {
            print!("({}, {}), ", p.x, p.y);
        }
        println!();

        {
            let w = ctx.heat_loss_map.width() as u32;
            let h = ctx.heat_loss_map.height() as u32;

            let mut image = RgbImage::from_fn(h, w, |x, y| {
                let xy = IVec2::new(x as i32, y as i32);
                let mut rgb = [0; 3];

                if best_goal_path.contains(&xy) {
                    rgb = [0xff, 0, 0xff];
                }

                Rgb(rgb)
            });
            let image =
                image::imageops::resize(&image, 10 * w, 10 * h, imageops::FilterType::Nearest);

            info!("Saving image");
            image.save(format!("target/day17_{w}x{h}.png")).unwrap();
        }

        ctx.state_seen[key]
    } else {
        error!("No path to goal???");
        unreachable!();
    }
}

// fn save_search_history(ctx: &Context, mut search_order: Vec<State>, goal: impl Into<(i32, i32)>) {
//     use indicatif::ProgressIterator;
//     use rayon::iter::{
//         IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelIterator,
//     };

//     let goal = goal.into();
//     let mut frames = vec![];

//     let max_cost: f32 = search_order.iter().map(|s| s.cost).max().unwrap() as f32;

//     info!("Generating {} frames for video", search_order.len());
//     let mut base_frame: Framebuffer<[u8; 3]> =
//         Framebuffer::new_with_ranges(ctx.state_map.range_x(), ctx.state_map.range_y());

//     for y in ctx.state_map.range_y() {
//         for x in ctx.state_map.range_x() {
//             let g = (255.0 * (ctx.heat_loss_map[(x, y)] as f32) / 9.0) as u8;
//             base_frame[(x, y)] = [g, g, g];
//         }
//     }

//     for mut state in search_order.into_iter().progress() {
//         let mut frame = base_frame.clone();
//         let mut g = (255.0 * (ctx.state_map[state.pos].cost as f32) / max_cost) as u8;
//         // Make more bands
//         g -= (g % 10);

//         for pos in &ctx.history_map[state.pos] {
//             frame[pos] = [g, 0x00, g];
//         }

//         frame[goal] = [0xff, 0xff, 0x66];

//         frames.push(frame.clone());
//     }

//     info!("Saving {} frames for video", frames.len());
//     let w = ctx.heat_loss_map.width();
//     let h = ctx.heat_loss_map.height();
//     let dir_name = if cfg!(test) {
//         format!("target/day17_test_{w}x{h}")
//     } else {
//         format!("target/day17_{w}x{h}")
//     };
//     std::fs::create_dir_all(&dir_name).unwrap();

//     let scale = if cfg!(test) { 50 } else { 2 };
//     frames
//         .par_iter()
//         .enumerate()
//         .progress()
//         .for_each(|(num, frame)| {
//             let mut frame = frame.make_image(scale, |rgb| Rgb(*rgb));
//             let filename = format!("{dir_name}/history_{num:>05}.bmp");
//             frame.save(filename).unwrap();
//         });
// }

#[allow(non_upper_case_globals)]
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
    // #[case::given_sub_02x02(5, EXAMPLE_INPUT_2x2)]
    // #[case::given_sub_03x03(11, EXAMPLE_INPUT_3x3)]
    // #[case::given_sub_04x04(21, EXAMPLE_INPUT_4x4)]
    // #[case::given_sub_05x05(28, EXAMPLE_INPUT_5x5)]
    // #[case::given_sub_06x06(42, EXAMPLE_INPUT_6x6)]
    // #[case::given_sub_07x07(54, EXAMPLE_INPUT_7x7)]
    // #[case::given_sub_08x08(70, EXAMPLE_INPUT_8x8)]
    // #[case::given_sub_09x09(83, EXAMPLE_INPUT_9x9)]
    // #[case::given_sub_10x10(94, EXAMPLE_INPUT_10x10)]
    // #[case::given_sub_11x11(102, EXAMPLE_INPUT_11x11)]
    // #[case::given_sub_12x12(103, EXAMPLE_INPUT_12x12)]
    #[trace]
    // #[timeout(ms(1_000))]
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

    // EXAMPLE_INPUT, but only the lower 2x2 square
    const EXAMPLE_INPUT_2x2: &str = r"
24
32
";

    // EXAMPLE_INPUT, but only the lower 3x3 square
    const EXAMPLE_INPUT_3x3: &str = r"
241
321
325
";

    // EXAMPLE_INPUT, but only the lower 4x4 square
    const EXAMPLE_INPUT_4x4: &str = r"
2413
3215
3255
3446
";

    // EXAMPLE_INPUT, but only the lower 5x5 square
    const EXAMPLE_INPUT_5x5: &str = r"
24134
32154
32552
34465
45466
";

    // EXAMPLE_INPUT, but only the lower 6x6 square
    const EXAMPLE_INPUT_6x6: &str = r"
241343
321545
325524
344658
454665
143859
";

    // EXAMPLE_INPUT, but only the lower 7x7 square
    const EXAMPLE_INPUT_7x7: &str = r"
2413432
3215453
3255245
3446585
4546657
1438598
4457876
";

    // EXAMPLE_INPUT, but only the lower 8x8 square
    const EXAMPLE_INPUT_8x8: &str = r"
24134323
32154535
32552456
34465858
45466578
14385987
44578769
36378779
";

    // EXAMPLE_INPUT, but only the lower 9x9 square
    const EXAMPLE_INPUT_9x9: &str = r"
241343231
321545353
325524565
344658584
454665786
143859879
445787698
363787797
465496798
";

    // EXAMPLE_INPUT, but only the lower 10x10 square
    const EXAMPLE_INPUT_10x10: &str = r"
2413432311
3215453535
3255245654
3446585845
4546657867
1438598798
4457876987
3637877979
4654967986
4564679986
";

    // EXAMPLE_INPUT, but only the lower 11x11 square
    const EXAMPLE_INPUT_11x11: &str = r"
24134323113
32154535356
32552456542
34465858454
45466578675
14385987984
44578769877
36378779796
46549679868
45646799864
12246868655
";

    // EXAMPLE_INPUT, but only the lower 12x12 square
    const EXAMPLE_INPUT_12x12: &str = r"
241343231132
321545353562
325524565425
344658584545
454665786753
143859879845
445787698776
363787797965
465496798688
456467998645
122468686556
254654888773
";

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
