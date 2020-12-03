use aoc_runner_derive::{aoc, aoc_generator};
use smallvec::SmallVec;

#[aoc_generator(day3)]
pub fn parse_input(input: &str) -> Vec<SmallVec<[u8; 32]>> {
    input
        .lines()
        .map(|line| line.trim().as_bytes().into())
        .collect()
}

fn count_trees(map: &[SmallVec<[u8; 32]>], dx: usize, dy: usize) -> usize {
    let mut tree_count = 0;

    let mut x = 0;
    let mut y = 0;
    loop {
        if y >= map.len() {
            break;
        }

        let xx = x % map[y].len();

        if map[y][xx] == b'#' {
            tree_count += 1;
        }

        x += dx;
        y += dy;
    }

    tree_count
}

#[aoc(day3, part1)]
pub fn part1(input: &[SmallVec<[u8; 32]>]) -> usize {
    count_trees(input, 3, 1)
}

#[aoc(day3, part2)]
pub fn part2(input: &[SmallVec<[u8; 32]>]) -> usize {
    let mut a = 1;

    a *= count_trees(input, 1, 1);
    a *= count_trees(input, 3, 1);
    a *= count_trees(input, 5, 1);
    a *= count_trees(input, 7, 1);
    a *= count_trees(input, 1, 2);

    a
}
