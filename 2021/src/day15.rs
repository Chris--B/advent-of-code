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

// Part1 ======================================================================
#[aoc(day15, part1)]
#[inline(never)]
pub fn part1(risk_map: &Framebuffer<u8>) -> i64 {
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

// Part2 ======================================================================
// #[aoc(day15, part2)]
// #[inline(never)]
// pub fn part2(input: &[i64]) -> i64 {
//     unimplemented!();
// }

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
}
