use crate::prelude::*;

use std::collections::BinaryHeap;

pub struct Day12 {
    pub start: (i32, i32),
    pub end: (i32, i32),
    pub heightmap: Framebuffer<u8>,
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

pub fn find_path(day: &Day12, start: (i32, i32)) -> Framebuffer<i64> {
    let mut total_steps_map: Framebuffer<i64> = Framebuffer::new_matching_size(&day.heightmap);
    total_steps_map.clear(i64::MAX);

    let mut points_to_explore_from: BinaryHeap<(i32, i32)> = BinaryHeap::new();
    points_to_explore_from.push(start);
    total_steps_map[start] = 0;

    while let Some((prev_x, prev_y)) = points_to_explore_from.pop() {
        // println!(
        //     "Exploring ({prev_x}, {prev_y}) dist={}",
        //     total_steps_map[(prev_x, prev_y)]
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

            if total_steps_map.range_x().contains(&x) && total_steps_map.range_y().contains(&y) {
                // Don't need climbing gear, and we can proceed
                if day.heightmap[(x, y)] <= day.heightmap[(prev_x, prev_y)] + 1 {
                    let old_dist = total_steps_map[(x, y)];
                    let new_dist = total_steps_map[(prev_x, prev_y)] as i64 + 1;

                    // If the path we're on is less risky than whatever found this point before, take it
                    // (Note: first-explored points have a maximum distance of i64::MAX, so they always get overwritten)
                    if new_dist < old_dist {
                        total_steps_map[(x, y)] = new_dist;
                        points_to_explore_from.push((x, y));
                    }
                }
            }
        }
    }

    total_steps_map
}

fn find_path_reverse(day: &Day12, end: (i32, i32)) -> Framebuffer<i64> {
    let mut total_steps_map: Framebuffer<i64> = Framebuffer::new_matching_size(&day.heightmap);
    total_steps_map.clear(i64::MAX);

    let mut points_to_explore_from: BinaryHeap<(i32, i32)> = BinaryHeap::new();
    points_to_explore_from.push(end);
    total_steps_map[end] = 0;

    while let Some((prev_x, prev_y)) = points_to_explore_from.pop() {
        // println!(
        //     "Exploring ({prev_x}, {prev_y}) dist={}",
        //     total_steps_map[(prev_x, prev_y)]
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

            if total_steps_map.range_x().contains(&x) && total_steps_map.range_y().contains(&y) {
                // Don't need climbing gear, and we can proceed
                if day.heightmap[(x, y)] + 1 >= day.heightmap[(prev_x, prev_y)] {
                    let old_steps = total_steps_map[(x, y)];
                    let new_steps = total_steps_map[(prev_x, prev_y)] as i64 + 1;

                    // If the path we're on is less risky than whatever found this point before, take it
                    // (Note: first-explored points have a maximum distance of i64::MAX, so they always get overwritten)
                    if new_steps < old_steps {
                        total_steps_map[(x, y)] = new_steps;
                        points_to_explore_from.push((x, y));
                    }
                }
            }
        }
    }

    total_steps_map
}

// Part1 ========================================================================
#[aoc(day12, part1)]
pub fn part1(day: &Day12) -> i64 {
    println!(
        "Height Map size: ({}, {}),",
        day.heightmap.width(),
        day.heightmap.height()
    );

    // Use given start to find shortest path to the end
    let total_steps_map = find_path(day, day.start);

    if cfg!(debug_assertions) {
        for y in total_steps_map.range_y() {
            for x in total_steps_map.range_x() {
                let steps = total_steps_map[(x, y)];
                if steps == i64::MAX {
                    print!("{:>4}", 'X');
                } else {
                    print!("{:>4}", steps);
                }
            }
            println!();
        }
    }

    total_steps_map[day.end]
}

#[aoc(day12, part1, reverse)]
pub fn part1_reverse(day: &Day12) -> i64 {
    println!(
        "Height Map size: ({}, {}),",
        day.heightmap.width(),
        day.heightmap.height()
    );

    // Use given start to find shortest path to the end
    let total_steps_map = find_path_reverse(day, day.end);

    if cfg!(debug_assertions) {
        for y in total_steps_map.range_y() {
            for x in total_steps_map.range_x() {
                let steps = total_steps_map[(x, y)];
                if steps == i64::MAX {
                    print!("{:>4}", 'X');
                } else {
                    print!("{:>4}", steps);
                }
            }
            println!();
        }
    }

    total_steps_map[day.start]
}

// Part2 ========================================================================
#[aoc(day12, part2)]
pub fn part2(day: &Day12) -> i64 {
    println!(
        "Height Map size: ({}, {}),",
        day.heightmap.width(),
        day.heightmap.height()
    );

    // Find a new start with the shortest path to the end, but maintains minimum height for the start.
    let total_steps_map = find_path_reverse(day, day.end);

    if cfg!(debug_assertions) {
        for y in total_steps_map.range_y() {
            for x in total_steps_map.range_x() {
                let steps = total_steps_map[(x, y)];
                if steps == i64::MAX {
                    print!("{:>2} ", '.');
                } else {
                    print!("{:>2} ", steps);
                }
            }
            println!();
        }
    }

    // map now contains the minimum steps from the end to each point.
    total_steps_map
        .iter_coords()
        .filter(|pt| day.heightmap[*pt] == 0)
        .map(|(x, y)| total_steps_map[(x, y)])
        // .inspect(|x| {
        //     dbg!(x);
        // })
        .min()
        .unwrap()
}

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
        #[values(part1, part1_reverse)]
        p: impl FnOnce(&Day12) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(&parse(input)), expected);
    }

    #[rstest]
    #[case::given(29, EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2)]
        p: impl FnOnce(&Day12) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(&parse(input)), expected);
    }
}
