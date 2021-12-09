use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
pub fn parse_input(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '0' => 0,
                    '1' => 1,
                    '2' => 2,
                    '3' => 3,
                    '4' => 4,
                    '5' => 5,
                    '6' => 6,
                    '7' => 7,
                    '8' => 8,
                    '9' => 9,
                    _ => todo!(),
                })
                .collect()
        })
        .collect()
}

// Part1 ======================================================================
#[aoc(day9, part1)]
#[inline(never)]
pub fn part1(input: &[Vec<u64>]) -> u64 {
    let width = input[0].len() as isize;
    let height = input.len() as isize;

    let get = |x: isize, y: isize| -> u64 {
        if x >= 0 && y >= 0 {
            if let Some(row) = input.get(y as usize) {
                if let Some(depth) = row.get(x as usize) {
                    return *depth;
                }
            }
        }

        return u64::MAX;
    };

    let mut risk = 0;

    for y in 0..height {
        for x in 0..width {
            let pts = [get(x, y - 1), get(x, y + 1), get(x - 1, y), get(x + 1, y)];
            let min = pts.into_iter().min().unwrap();
            if min > get(x, y) {
                // found one
                risk += 1 + get(x, y);
            }
        }
    }

    risk
}

// Part2 ======================================================================
#[aoc(day9, part2)]
#[inline(never)]
pub fn part2(input: &[i64]) -> i64 {
    unimplemented!();

#[test]
fn check_example_1() {
    const INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";
    assert_eq!(part1(&parse_input(INPUT)), 15);
}
