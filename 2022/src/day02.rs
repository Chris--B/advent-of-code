// The 0s are for padding, leave me alone
#![allow(clippy::identity_op)]

use aoc_runner_derive::aoc;

// Part1 ========================================================================
#[aoc(day2, part1)]
#[inline(never)]
pub fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| match line {
            "" => 0, // skip empty lines
            //
            "A X" => 3 + 1, // rock_rock
            "A Y" => 6 + 2, // rock_paper
            "A Z" => 0 + 3, // rock_scissors
            //
            "B X" => 0 + 1, // paper_rock
            "B Y" => 3 + 2, // paper_paper
            "B Z" => 6 + 3, // paper_scissors
            //
            "C X" => 6 + 1, // scissors_rock
            "C Y" => 0 + 2, // scissors_paper
            "C Z" => 3 + 3, // scissors_scissors

            _ => unreachable!(),
        })
        .sum()
}

// Part2 ========================================================================
#[aoc(day2, part2)]
#[inline(never)]
pub fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(|line| match line {
            "" => 0, // skip empty lines
            //
            "A X" => 0 + 3, // Rock/Lose, pick Scissors(3)
            "A Y" => 3 + 1, // Rock/Draw, pick Rock(1)
            "A Z" => 6 + 2, // Rock/Win, pick Paper(2)
            //
            "B X" => 0 + 1, // Paper/Lose, pick Rock(1)
            "B Y" => 3 + 2, // Paper/Draw, pick Paper(2)
            "B Z" => 6 + 3, // Paper/Win, pick Scissors(3)
            //
            "C X" => 0 + 2, // Scissors/Lose, pick Paper(2)
            "C Y" => 3 + 3, // Scissors/Draw, pick Scissors(3)
            "C Z" => 6 + 1, // Scissors/Win, pick Rock(1)

            _ => unreachable!(),
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
A Y
B X
C Z
";

    // AX
    // BY
    // CZ

    #[rstest]
    #[case::given(15, EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(input), expected);
    }

    #[rstest]
    #[case::given(12, EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(input), expected);
    }
}
