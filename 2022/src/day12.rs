use crate::prelude::*;

use std::collections::BinaryHeap;

pub struct Day12 {
    start: (i32, i32),
    end: (i32, i32),
    heightmap: Framebuffer<u8>,
}

#[aoc_generator(day12)]
pub fn parse(input: &str) -> Day12 {
    let lines: Vec<_> = input.lines().collect();
    let width = lines[0].len() as i32;
    let height = lines.len() as i32;

    let mut start = None;
    let mut end = None;

    let heightmap = Framebuffer::new_with_ranges_and(0..width, 0..height, |x, y| {
        let c = lines[y as usize].as_bytes()[x as usize] as u8;
        if c == b'S' {
            start = Some((x, y));
            0
        } else if c == b'E' {
            // "the location that should get the best signal (E) has elevation z"
            end = Some((x, y));
            b'z' - b'a'
        } else {
            c - b'a'
        }
    });

    Day12 {
        start: start.unwrap(),
        end: end.unwrap(),
        heightmap,
    }
}

fn find_path(day: &Day12) -> i64 {
    let mut total_dist_map: Framebuffer<i64> = Framebuffer::new_matching_size(&day.heightmap);
    total_dist_map.clear(i64::MAX);

    let mut points_to_explore_from: BinaryHeap<(i32, i32)> = BinaryHeap::new();
    points_to_explore_from.push(day.start);
    total_dist_map[day.start] = 0;

    while let Some((prev_x, prev_y)) = points_to_explore_from.pop() {
        // println!(
        //     "Exploring ({prev_x}, {prev_y}) dist={}",
        //     total_dist_map[(prev_x, prev_y)]
        // );

        // Check in all directions for low distance paths
        for (dx, dy) in [
            (1_i32, 0_i32),
            (0, 1),
            (-1, 0),
            (0, -1),
            // No diagonals
        ] {
            let x = prev_x + dx;
            let y = prev_y + dy;

            if total_dist_map.range_x().contains(&x) && total_dist_map.range_y().contains(&y) {
                // Don't need climbing gear, and we can proceed
                if day.heightmap[(x, y)] <= day.heightmap[(prev_x, prev_y)] + 1 {
                    let old_dist = total_dist_map[(x, y)];
                    let new_dist = total_dist_map[(prev_x, prev_y)] as i64 + 1;

                    // If the path we're on is less risky than whatever found this point before, take it
                    // (Note: first-explored points have a maximum distance of i64::MAX, so they always get overwritten)
                    if new_dist < old_dist {
                        total_dist_map[(x, y)] = new_dist;
                        points_to_explore_from.push((x, y));
                    }
                }
            }
        }
    }

    total_dist_map[day.end]
}

// Part1 ========================================================================
#[aoc(day12, part1)]
pub fn part1(day: &Day12) -> i64 {
    find_path(day)
}

// Part2 ========================================================================
// #[aoc(day12, part2)]
// pub fn part2(input: &Day12) -> i64 {
//     unimplemented!();
// }

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";

    #[rstest]
    #[case::given(31, EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1)]
        p: impl FnOnce(&Day12) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(&parse(input)), expected);
    }

    // #[rstest]
    // #[case::given(999_999, EXAMPLE_INPUT)]
    // #[trace]
    // fn check_ex_part_2(
    //     #[notrace]
    //     #[values(part2)]
    // p: impl FnOnce(&Day12) -> i64,
    //     #[case] expected: i64,
    //     #[case] input: &str,
    // ) {
    //     let input = input.trim();
    //     assert_eq!(p(&parse(input)), expected);
    // }
}
