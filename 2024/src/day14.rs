use image::{imageops::FilterType, RgbImage};

use crate::prelude::*;

const DIMS: IVec2 = if cfg!(test) {
    IVec2::new(11, 7)
} else {
    IVec2::new(101, 103)
};

// Part1 ========================================================================
#[aoc(day14, part1)]
pub fn part1(input: &str) -> i64 {
    let mut quadrants = [0; 4];

    for (px, py, vx, vy) in input.i64s().map(|n| n as i32).tuples() {
        let mut p = IVec2::new(px, py);
        let v = IVec2::new(vx, vy);
        debug_assert!(v.x < DIMS.x);
        debug_assert!(v.y < DIMS.y);

        p += 100 * v;
        p.x %= DIMS.x;
        p.y %= DIMS.y;

        p += DIMS;
        p.x %= DIMS.x;
        p.y %= DIMS.y;

        debug_assert!(p.x >= 0);
        debug_assert!(p.y >= 0);
        if (p.x == DIMS.x / 2) || p.y == DIMS.y / 2 {
            continue;
        }

        let qidx = ((p.x < DIMS.x / 2) as usize) << 1 | ((p.y < DIMS.y / 2) as usize);
        quadrants[qidx] += 1;
    }

    if cfg!(debug_assertions) {
        println!("quadrants={quadrants:?}");
    }

    quadrants.iter().product()
}

// Part2 ========================================================================
#[aoc(day14, part2)]
pub fn part2(input: &str) -> i64 {
    let mut bots = input
        .i64s()
        .map(|n| n as i32)
        .tuples()
        .map(|(px, py, vx, vy)| (IVec2::new(px, py), IVec2::new(vx, vy)))
        .collect_vec();

    let mut tree_time = 0;
    let mut densest = 0;

    for seconds in 1..=(DIMS.x * DIMS.y) {
        let mut tiles = [[0; 4]; 4];
        let tile_dims = DIMS / 4;

        for (p, v) in &mut bots {
            *p += *v;
            p.x = (p.x + DIMS.x) % DIMS.x;
            p.y = (p.y + DIMS.y) % DIMS.y;

            debug_assert!(p.x >= 0);
            debug_assert!(p.y >= 0);

            let t = ((p.y / tile_dims.y) as usize, (p.x / tile_dims.x) as usize);
            if t.0 < tiles.len() && t.1 < tiles[0].len() {
                tiles[t.0][t.1] += 1;
            }
        }

        let this_density = *tiles.iter().flatten().max().unwrap();
        if this_density > densest {
            if cfg!(debug_assertions) {
                println!("[{seconds:>4}] Densest={this_density}")
            };
            densest = this_density;
            tree_time = seconds as i64;
        }

        // Make pretty picture I guess (we know where the tree isn't from failed attempts)
        if false {
            std::fs::create_dir_all("day14-images/").unwrap();
            let mut img = RgbImage::from_fn(DIMS.x as _, DIMS.y as _, |_x, _y| AOC_BLUE);

            for (p, _v) in &bots {
                img.put_pixel(p.x as _, p.y as _, AOC_GOLD);
            }

            let img = image::imageops::resize(
                &img,
                2 * DIMS.x as u32,
                2 * DIMS.y as u32,
                FilterType::Triangle,
            );

            img.save(format!("day14-images/day14-{seconds:04}.png"))
                .unwrap();
        }
    }

    tree_time
}

#[aoc(day14, part2, cycles)]
pub fn part2_cycles(input: &str) -> i64 {
    let mut bots = input
        .i64s()
        .map(|n| n as i32)
        .tuples()
        .map(|(px, py, vx, vy)| (IVec2::new(px, py), IVec2::new(px, py), IVec2::new(vx, vy)))
        .collect_vec();

    // First pass - find our cycle offsets
    let mut max_pop = [30, 30];
    let mut offsets = [0, 0];
    for seconds in 1..=DIMS.component_min() {
        // Board is always wider than tall
        let mut pop = [[0; DIMS.y as usize]; 2];
        for (_start, pos, vel) in &mut bots {
            *pos += *vel;
            pos.x = (pos.x + DIMS.x) % DIMS.x;
            pos.y = (pos.y + DIMS.y) % DIMS.y;

            pop[0][pos.x as usize] += 1;
            pop[1][pos.y as usize] += 1;
        }

        // Look for horizontal/vertical oddities
        for i in [0, 1_usize] {
            let max = *pop[i].iter().max().unwrap();
            if max > max_pop[i] {
                max_pop[i] = max;
                offsets[i] = seconds;
            }
        }
    }

    if cfg!(debug_assertions) {
        println!("cycles={offsets:?}",);
        println!("Tree appears at t, for some a & b");
        println!(
            "    t == {dims_x}a + {offset0} == {dims_y}b + {offset1}",
            dims_x = DIMS.x,
            dims_y = DIMS.y,
            offset0 = offsets[0],
            offset1 = offsets[1],
        );
    }

    // TODO: Sunzi's Theorem / CRT
    for a in 0..=100 {
        if (DIMS.x * a + offsets[0] - offsets[1]) % DIMS.y != 0 {
            continue;
        }
        if cfg!(debug_assertions) {
            let b = (DIMS.x * a + offsets[0] - offsets[1]) / DIMS.y;
            debug_assert_eq!(DIMS.x * a + offsets[0], DIMS.y * b + offsets[1]);
        }

        return (DIMS.x * a + offsets[0]) as i64;
    }

    unreachable!("No solutions found")
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

    #[rstest]
    #[case::given(1 * 3 * 4 * 1, EXAMPLE_INPUT)]
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

    // There is no example for part 2, just run it on real input and see if it works.
}
