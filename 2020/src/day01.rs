use aoc_runner_derive::{aoc, aoc_generator};
// use itertools::Itertools;

#[aoc_generator(day1)]
pub fn parse_input(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[u32]) -> u32 {
    let len = input.len();
    for i in 0..len {
        for j in i..len {
            if input[i] + input[j] == 2020 {
                return input[i] * input[j];
            }
        }
    }

    unreachable!()
}

#[aoc(day1, part2)]
pub fn part2(input: &[u32]) -> u32 {
    let len = input.len();
    for i in 0..len {
        for j in i..len {
            for k in j..len {
                if input[i] + input[j] + input[k] == 2020 {
                    return input[i] * input[j] * input[k];
                }
            }
        }
    }

    unreachable!()
}

#[test]
fn part1_check_ex() {
    let report = [1721, 979, 366, 299, 675, 1456];

    assert_eq!(part1(&report), 514579);
}

#[test]
fn part2_check_ex() {
    let report = [1721, 979, 366, 299, 675, 1456];

    assert_eq!(part2(&report), 241861950);
}
