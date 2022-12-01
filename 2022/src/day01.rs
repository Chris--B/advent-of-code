use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn parse_input(input: &str) -> Vec<i64> {
    let mut calories_per_elf = vec![0];

    for line in input.lines() {
        if line.trim().is_empty() {
            calories_per_elf.push(0);
        } else {
            let calories: i64 = line.parse().unwrap();
            *calories_per_elf.last_mut().unwrap() += calories;
        }
    }

    calories_per_elf
}

// Part1 ========================================================================
#[aoc(day1, part1)]
#[inline(never)]
pub fn part1(input: &[i64]) -> i64 {
    *input.iter().max().unwrap()
}

// Part2 ========================================================================
#[aoc(day1, part2)]
#[inline(never)]
pub fn part2(input: &[i64]) -> i64 {
    let mut input = Vec::from(input);
    input.sort();
    input.into_iter().rev().take(3).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn check_example_1() {
        assert_eq!(part1(&parse_input(EXAMPLE_INPUT)), 24_000);
    }

    #[test]
    fn check_example_2() {
        assert_eq!(part2(&parse_input(EXAMPLE_INPUT)), 45_000);
    }
}
