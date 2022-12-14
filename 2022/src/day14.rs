use crate::prelude::*;

const BLOCK_AIR: u8 = 0;
const BLOCK_SAND: u8 = 1;
const BLOCK_ROCK: u8 = 2;
const BLOCK_SPAWN: u8 = 255;

// Part1 ========================================================================
#[aoc(day14, part1)]
pub fn part1(input: &str) -> i64 {
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

    // Pad out a layer to make indexing simpler
    min_x += -1;
    min_y += -1;

    // Pad out a layer *and* 1 more for the lack of ..= ranges on our part
    max_x += 2;
    max_y += 2;

    let mut cave: Framebuffer<u8> = Framebuffer::new_with_ranges(min_x..max_x, min_y..max_y);
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

    {
        let img = cave.make_image(1, |block| match *block {
            BLOCK_AIR => image::Luma([230_u8]),
            BLOCK_SAND => image::Luma([85 + 128]),
            BLOCK_ROCK => image::Luma([32]),
            BLOCK_SPAWN => image::Luma([255]),
            _ => image::Luma([0_u8]),
        });
        img.save("day14_out.png").unwrap();
    }

    spawned
}

// Part2 ========================================================================
// #[aoc(day14, part2)]
// pub fn part2(input: &str) -> i64 {
//     unimplemented!();
// }

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
        #[values(part1)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(input), expected);
    }

    // #[rstest]
    // #[case::given(999_999, EXAMPLE_INPUT)]
    // #[trace]
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
