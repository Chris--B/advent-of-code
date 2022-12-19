use crate::prelude::*;

const BLOCK_AIR: u8 = 0;
const BLOCK_SAND: u8 = 1;
const BLOCK_ROCK: u8 = 2;
const BLOCK_SPAWN: u8 = 3;
const BLOCK_FALLING_SAND: u8 = 4;

struct Interval(());

fn begin_internval(name: &'static str) -> Interval {
    let state = ();

    extern "C" {
        fn platform_begin_interval(x: i32);
    }

    unsafe {
        platform_begin_interval(5);
    }

    Interval(state)
}

impl Drop for Interval {
    fn drop(&mut self) {
        let state = self.0;
    }
}

const fn color(x: u32) -> image::Rgb<u8> {
    let [b, g, r, _a] = x.to_le_bytes();
    image::Rgb([r, g, b])
}

#[inline(always)]
fn save_image(cave: &Framebuffer<u8>, name: &str) {
    const ENABLE: bool = false;

    if ENABLE {
        use image::Rgb;

        let scale_x = 1920 / cave.range_x().len() as u32;
        let scale_y = 1080 / cave.range_y().len() as u32;
        let scale = [scale_x, scale_y, 30].into_iter().min().unwrap();

        let w = scale as usize * cave.range_x().len();
        let h = scale as usize * cave.range_y().len();
        info!(
            "Saving image to \"{name}\"... ({w}x{h}) ~{:.1}M pixels ...",
            (w * h) as f32 / 1_000_000.0
        );

        // https://coolors.co/palette/264653-2a9d8f-e9c46a-f4a261-ad533d
        const PALETTE: [u32; 5] = [0x26_46_53, 0x2A_9D_8F, 0xE9_C4_6A, 0xF4_A2_61, 0xAD_53_3D];
        const AOC_GOLD: Rgb<u8> = color(0xFF_FF_66);

        let img = cave.make_image(scale, |block| match *block {
            BLOCK_AIR => color(PALETTE[0]),
            BLOCK_SAND => color(PALETTE[2]),
            BLOCK_FALLING_SAND => color(PALETTE[3]),
            BLOCK_ROCK => color(PALETTE[4]),
            BLOCK_SPAWN => AOC_GOLD,
            _ => color(0xFF_00_FF),
        });
        img.save(name).unwrap();

        info!("... done");
    }
}

#[derive(PartialEq, Eq)]
enum Floor {
    NoFloor,
    SolidFloor,
}
use Floor::*;

fn parse_cave_to_fb(input: &str, floor: Floor) -> Framebuffer<u8> {
    let mut min_x = 500;
    let mut min_y = 0;

    let mut max_x = 500;
    let mut max_y = 0;

    let mut cave_walls: Vec<Vec<IVec2>> = vec![];

    for line in input.lines() {
        cave_walls.push(vec![]);

        let wall: &mut _ = cave_walls.last_mut().unwrap();
        for s in line.split(" -> ") {
            let [x, y]: [i32; 2] = iter_to_array(s.split(',').map(|ns| ns.parse().unwrap()));
            wall.push((x, y).into());

            min_x = min_x.min(x);
            min_y = min_y.min(y);

            max_x = max_x.max(x);
            max_y = max_y.max(y);
        }
    }

    min_y -= 2;
    max_y += 3;

    if floor == SolidFloor {
        let h = max_y - min_y;
        min_x = 500 - h + 1;
        max_x = 500 + h - 1;
    } else {
        min_x += -1;
        max_x += 1;
    }

    let mut cave = Framebuffer::new_with_ranges(min_x..(max_x + 1), min_y..(max_y + 1));
    cave.set_border_color(Some(BLOCK_AIR));

    for wall in cave_walls {
        for pair in wall.windows(2) {
            if pair[1].y == pair[0].y {
                let y = pair[0].y;

                let xa = [pair[1].x, pair[0].x].into_iter().min().unwrap();
                let xb = [pair[1].x, pair[0].x].into_iter().max().unwrap();

                for x in xa..=xb {
                    cave[(x, y)] = BLOCK_ROCK;
                }
            } else if pair[1].x == pair[0].x {
                let x = pair[0].x;

                let ya = [pair[1].y, pair[0].y].into_iter().min().unwrap();
                let yb = [pair[1].y, pair[0].y].into_iter().max().unwrap();

                for y in ya..=yb {
                    cave[(x, y)] = BLOCK_ROCK;
                }
            } else {
                unreachable!();
            }
        }
    }

    if floor == SolidFloor {
        let floor_y = cave.range_y().end - 2;
        let std::ops::Range { start, end } = cave.range_x();
        for x in start + 1..end - 1 {
            cave[(x, floor_y)] = BLOCK_ROCK;
        }
    }

    cave
}

// Part1 ========================================================================
#[aoc(day14, part1)]
pub fn part1(input: &str) -> i64 {
    let _interval = begin_internval("part1");
    let mut cave = parse_cave_to_fb(input, NoFloor);

    const DOWN: IVec2 = IVec2::new(0, 1); // I don't know...
    const DOWN_LEFT: IVec2 = IVec2::new(-1, 1);
    const DOWN_RIGHT: IVec2 = IVec2::new(1, 1);

    let mut spawned = 0;
    // Spawning
    'spawning: for _ in 0.. {
        let mut sand = IVec2::new(500, 0) + DOWN;
        debug_assert_ne!(cave[sand], BLOCK_SAND);

        // Falling
        'falling: loop {
            while cave.range_y().contains(&sand.y) && cave[sand + DOWN] == BLOCK_AIR {
                sand += DOWN;
            }

            // We're falling into the void
            if !cave.range_y().contains(&sand.y) {
                break 'spawning;
            }
            debug_assert_eq!(cave[sand], BLOCK_AIR);

            // We're ontop of not-air. Figure out if we're done, or rolling
            if cave[sand + DOWN_LEFT] == BLOCK_AIR {
                sand.x -= 1;
                continue 'falling;
            } else if cave[sand + DOWN_RIGHT] == BLOCK_AIR {
                sand.x += 1;
                continue 'falling;
            } else {
                // We'll rest here just fine
                spawned += 1;

                break 'falling;
            }
        }

        // Fell
        cave[sand] = BLOCK_SAND;
    }

    cave[(500, 0)] = BLOCK_SPAWN;
    save_image(&cave, "day14-pt1.png");

    spawned
}

#[aoc(day14, part1, tracking)]
pub fn part1_tracking(input: &str) -> i64 {
    let _interval = begin_internval("part1_tracking");
    let mut cave = parse_cave_to_fb(input, NoFloor);

    const DOWN: IVec2 = IVec2::new(0, 1); // I don't know...
    const DOWN_LEFT: IVec2 = IVec2::new(-1, 1);
    const DOWN_RIGHT: IVec2 = IVec2::new(1, 1);

    let mut path = vec![IVec2::new(500, 0)];
    let mut spawned = 0;

    // Spawning
    // 'spawning: for _ in 0..5_000 {
    // 'spawning: for _ in 0..25 {
    'spawning: loop {
        // Copy the last block and use it as a starting point
        // We don't need to pop this until it ends up as sand
        let mut sand: IVec2 = if let Some(sand) = path.last() {
            *sand
        } else {
            break 'spawning;
        };
        debug_assert_eq!(cave[sand], BLOCK_AIR, "{sand:?} isn't air but should be");

        // Falling
        'falling: loop {
            while cave.range_y().contains(&sand.y) && cave[sand + DOWN] == BLOCK_AIR {
                sand += DOWN;
                path.push(sand);
            }

            // We're falling into the void
            if !cave.range_y().contains(&sand.y) {
                break 'spawning;
            }
            debug_assert_eq!(cave[sand], BLOCK_AIR, "{sand:?} isn't air but should be");

            // We're ontop of not-air. Figure out if we're done, or rolling
            if cave[sand + DOWN_LEFT] == BLOCK_AIR {
                sand += DOWN_LEFT;
                if cave[sand] == BLOCK_AIR {
                    path.push(sand);
                }

                continue 'falling;
            } else if cave[sand + DOWN_RIGHT] == BLOCK_AIR {
                sand += DOWN_RIGHT;
                if cave[sand] == BLOCK_AIR {
                    path.push(sand);
                }

                continue 'falling;
            } else {
                // We'll rest here just fine
                spawned += 1;
                break 'falling;
            }
        }

        // Fell
        cave[sand] = BLOCK_SAND;

        // Remove this block from out path
        path.pop();
    }

    // If we break early, there's a trail of sand to render!
    while let Some(s) = path.pop() {
        if let Some(x) = cave.get_mut(s.x as isize, s.y as isize) {
            *x = BLOCK_FALLING_SAND;
        }
    }

    cave[(500, 0)] = BLOCK_SPAWN;
    save_image(&cave, "day14_tracking-pt1.png");

    spawned
}

// Part2 ========================================================================
#[aoc(day14, part2)]
pub fn part2(input: &str) -> i64 {
    let _interval = begin_internval("part2");
    let mut cave = parse_cave_to_fb(input, SolidFloor);

    const DOWN: IVec2 = IVec2::new(0, 1); // I don't know...
    const DOWN_LEFT: IVec2 = IVec2::new(-1, 1);
    const DOWN_RIGHT: IVec2 = IVec2::new(1, 1);

    let mut spawned = 0;
    // Spawning
    'spawning: for _ in 0.. {
        let mut sand = IVec2::new(500, 0);

        // Sand spawned on our spawner and we're done
        if cave[sand] == BLOCK_SAND {
            break;
        }

        // Falling
        'falling: loop {
            while cave.range_y().contains(&sand.y) && cave[sand + DOWN] == BLOCK_AIR {
                sand += DOWN;
            }

            // We're falling into the void
            if !cave.range_y().contains(&sand.y) {
                break 'spawning;
            }
            debug_assert_eq!(cave[sand], BLOCK_AIR);

            // We're ontop of not-air. Figure out if we're done, or rolling
            if cave[sand + DOWN_LEFT] == BLOCK_AIR {
                sand.x -= 1;
                continue 'falling;
            } else if cave[sand + DOWN_RIGHT] == BLOCK_AIR {
                sand.x += 1;
                continue 'falling;
            } else {
                // We'll rest here just fine
                spawned += 1;

                break 'falling;
            }
        }

        // Fell
        cave[sand] = BLOCK_SAND;
    }

    cave[(500, 0)] = BLOCK_SPAWN;
    save_image(&cave, "day14-pt2.png");

    spawned
}

#[aoc(day14, part2, tracking)]
pub fn part2_tracking(input: &str) -> i64 {
    let _interval = begin_internval("part2_tracking");
    let mut cave = parse_cave_to_fb(input, SolidFloor);

    const DOWN: IVec2 = IVec2::new(0, 1); // I don't know...
    const DOWN_LEFT: IVec2 = IVec2::new(-1, 1);
    const DOWN_RIGHT: IVec2 = IVec2::new(1, 1);

    let mut path = vec![IVec2::new(500, 0)];
    let mut spawned = 0;

    // Spawning
    // 'spawning: for _ in 0..5_000 {
    // 'spawning: for _ in 0..25 {
    'spawning: loop {
        // Copy the last block and use it as a starting point
        // We don't need to pop this until it ends up as sand
        let mut sand: IVec2 = if let Some(sand) = path.last() {
            *sand
        } else {
            break 'spawning;
        };
        debug_assert_eq!(cave[sand], BLOCK_AIR, "{sand:?} isn't air but should be");

        // Falling
        'falling: loop {
            while cave.range_y().contains(&sand.y) && cave[sand + DOWN] == BLOCK_AIR {
                sand += DOWN;
                path.push(sand);
            }

            // We're falling into the void
            if !cave.range_y().contains(&sand.y) {
                break 'spawning;
            }
            debug_assert_eq!(cave[sand], BLOCK_AIR, "{sand:?} isn't air but should be");

            // We're ontop of not-air. Figure out if we're done, or rolling
            if cave[sand + DOWN_LEFT] == BLOCK_AIR {
                sand += DOWN_LEFT;
                if cave[sand] == BLOCK_AIR {
                    path.push(sand);
                }

                continue 'falling;
            } else if cave[sand + DOWN_RIGHT] == BLOCK_AIR {
                sand += DOWN_RIGHT;
                if cave[sand] == BLOCK_AIR {
                    path.push(sand);
                }

                continue 'falling;
            } else {
                // We'll rest here just fine
                spawned += 1;
                break 'falling;
            }
        }

        // Fell
        cave[sand] = BLOCK_SAND;

        // Remove this block from out path
        path.pop();
    }

    // If we break early, there's a trail of sand to render!
    while let Some(s) = path.pop() {
        cave[s] = BLOCK_FALLING_SAND;
    }

    cave[(500, 0)] = BLOCK_SPAWN;
    save_image(&cave, "day14_tracking-pt2.png");

    spawned
}

#[aoc(day14, part2, smol_tracking)]
pub fn part2_smol_tracking(input: &str) -> i64 {
    let _interval = begin_internval("part2_smol_tracking");
    let mut cave = parse_cave_to_fb(input, NoFloor);
    cave.set_border_color(Some(BLOCK_ROCK));

    // Add our own floor
    let floor_y = cave.range_y().end - 2;
    let std::ops::Range { start, end } = cave.range_x();
    for x in start..end {
        cave[(x, floor_y)] = BLOCK_ROCK;
    }

    const DOWN: IVec2 = IVec2::new(0, 1); // I don't know...
    const DOWN_LEFT: IVec2 = IVec2::new(-1, 1);
    const DOWN_RIGHT: IVec2 = IVec2::new(1, 1);

    let mut path = vec![IVec2::new(500, 0)];
    let mut spawned = 0;

    let mut left_h = 0;
    let mut right_h = 0;

    let min_x = cave.range_x().start;
    let max_x = cave.range_x().end;

    // Spawning
    // 'spawning: for _ in 0..5_000 {
    // 'spawning: for _ in 0..25 {
    'spawning: loop {
        // Copy the last block and use it as a starting point
        // We don't need to pop this until it ends up as sand
        let mut sand: IVec2 = if let Some(sand) = path.last() {
            *sand
        } else {
            break 'spawning;
        };
        debug_assert_eq!(cave[sand], BLOCK_AIR, "{sand:?} isn't air but should be");

        // Falling
        'falling: loop {
            while cave.range_y().contains(&sand.y) && cave[sand + DOWN] == BLOCK_AIR {
                sand += DOWN;
                path.push(sand);
            }

            // We're falling into the void
            if !cave.range_y().contains(&sand.y) {
                break 'spawning;
            }
            debug_assert_eq!(cave[sand], BLOCK_AIR, "{sand:?} isn't air but should be");

            // We're ontop of not-air. Figure out if we're done, or rolling
            if cave[sand + DOWN_LEFT] == BLOCK_AIR {
                sand += DOWN_LEFT;
                if cave[sand] == BLOCK_AIR {
                    path.push(sand);
                }

                continue 'falling;
            } else if cave[sand + DOWN_RIGHT] == BLOCK_AIR {
                sand += DOWN_RIGHT;
                if cave[sand] == BLOCK_AIR {
                    path.push(sand);
                }

                continue 'falling;
            } else {
                // We'll rest here just fine
                spawned += 1;
                break 'falling;
            }
        }

        // Fell
        cave[sand] = BLOCK_SAND;

        if sand.x == min_x {
            left_h = sand.y;
        }

        if sand.x + 1 == max_x {
            right_h = sand.y;
        }

        // Remove this block from out path
        path.pop();
    }

    // If we break early, there's a trail of sand to render!
    while let Some(s) = path.pop() {
        if let Some(x) = cave.get_mut(s.x as isize, s.y as isize) {
            *x = BLOCK_FALLING_SAND;
        }
    }

    spawned += (left_h * (left_h - 1)) as i64 / 2;
    spawned += (right_h * (right_h - 1)) as i64 / 2;

    cave[(500, 0)] = BLOCK_SPAWN;
    save_image(&cave, "day14_smol-tracking-pt2.png");

    spawned
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";

    #[rstest]
    #[case::given(24, EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1, part1_tracking)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(input), expected);
    }

    #[rstest]
    #[case::given(93, EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2, part2_tracking, part2_smol_tracking)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(input), expected);
    }
}
