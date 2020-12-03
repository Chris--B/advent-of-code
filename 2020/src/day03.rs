use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
pub fn parse_input(input: &str) -> Vec<[u8; 31]> {
    use std::convert::TryInto;

    input
        .lines()
        .map(|line| line.trim().as_bytes().try_into().unwrap())
        .collect()
}

fn count_trees(map: &[[u8; 31]], dx: usize, dy: usize) -> usize {
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
pub fn part1(input: &[[u8; 31]]) -> usize {
    count_trees(input, 3, 1)
}

#[aoc(day3, part2)]
pub fn part2(input: &[[u8; 31]]) -> usize {
    let mut a = 1;

    a *= count_trees(input, 1, 1);
    a *= count_trees(input, 3, 1);
    a *= count_trees(input, 5, 1);
    a *= count_trees(input, 7, 1);
    a *= count_trees(input, 1, 2);

    a
}
