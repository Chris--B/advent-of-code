use aoc_runner_derive::{aoc, aoc_generator};

use crate::framebuffer::Framebuffer;

use std::collections::BinaryHeap;

#[aoc_generator(day15)]
pub fn parse_input(input: &str) -> Framebuffer<u8> {
    let lines = input.as_bytes().split(|b| *b == b'\n');

    let width = lines.clone().next().unwrap().len();
    let height = lines.clone().count();

    let mut fb = Framebuffer::with_dims(width, height);

    for (y, line) in lines.enumerate() {
        for (x, b) in line.iter().enumerate() {
            fb[(x, y)] = *b - b'0';
        }
    }

    fb
}

fn find_min_risk(risk_map: &Framebuffer<u8>) -> i64 {
    let mut total_risk_map: Framebuffer<i64> = Framebuffer::with_dims_of(risk_map);
    total_risk_map.clear(i64::MAX);

    let mut points_to_explore_from: BinaryHeap<(usize, usize)> = BinaryHeap::new();
    // "explore" (0, 0) first
    points_to_explore_from.push((0, 0));
    total_risk_map[(0_u32, 0_u32)] = 0;

    while let Some((prev_x, prev_y)) = points_to_explore_from.pop() {
        // println!(
        //     "Exploring ({prev_x}, {prev_y}) risk={}",
        //     total_risk_map[(prev_x, prev_y)]
        // );

        // Check in all directions for low risk paths
        for (dx, dy) in [
            (1_isize, 0_isize),
            (0, 1),
            (-1, 0),
            (0, -1),
            // No diagonals
        ] {
            let x = (prev_x as isize + dx) as usize;
            let y = (prev_y as isize + dy) as usize;

            if (x < total_risk_map.width()) && (y < total_risk_map.height()) {
                let old_risk = total_risk_map[(x, y)];
                let new_risk = risk_map[(x, y)] as i64 + total_risk_map[(prev_x, prev_y)];

                // If the path we're on is less risky than whatever found this point before, take it
                // (Note: first-explored points have a maximum risk of i64::MAX, so they always get overwritten)
                if new_risk < old_risk {
                    total_risk_map[(x, y)] = new_risk;
                    points_to_explore_from.push((x, y));
                }
            }
        }
    }

    total_risk_map[(risk_map.width() - 1, risk_map.height() - 1)]
}

// Part1 ======================================================================
#[aoc(day15, part1)]
#[inline(never)]
pub fn part1(risk_map: &Framebuffer<u8>) -> i64 {
    find_min_risk(risk_map)
}

// Part2 ======================================================================
#[aoc(day15, part2)]
#[inline(never)]
pub fn part2(risk_map: &Framebuffer<u8>) -> i64 {
    // Scale risk map up. From AOC:
    //      the area you originally scanned is just one tile in a 5x5 tile area
    // that forms the full map. Your original map tile repeats to the right and
    // downward; each time the tile repeats to the right or downward, all of its
    // risk levels are 1 higher than the tile immediately up or left of it.
    // However, risk levels above 9 wrap back around to 1.
    let tile_width = risk_map.width();
    let tile_height = risk_map.height();

    let risk_map = Framebuffer::from_func(5 * tile_width, 5 * tile_height, |x, y| {
        let tile_x = x / tile_width;
        let x = x % tile_width;

        let tile_y = y / tile_height;
        let y = y % tile_height;

        let base_risk = risk_map[(x, y)] as usize;
        let total = base_risk + tile_x + tile_y;

        ((total - 1) % 9) as u8 + 1
    });

    find_min_risk(&risk_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_example_1() {
        let input = r"
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
"
        .trim();
        assert_eq!(part1(&parse_input(input)), 40);
    }

    #[test]
    fn check_example_2() {
        let input = r"
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
"
        .trim();
        assert_eq!(part2(&parse_input(input)), 315);
    }
}
