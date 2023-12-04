#![allow(unused, dead_code)]
use crate::prelude::*;

// Part1 ========================================================================

fn score_line(line: &str) -> i64 {
    let (_, line) = line.split_once(':').unwrap();
    let (winning, player) = line.split_once('|').unwrap();

    let mut winning = winning
        .split_whitespace()
        .map(|s| -> u128 { s.parse().unwrap() })
        .fold(0_u128, |acc, x| acc | (1 << x));

    let mut player = player
        .split_whitespace()
        .map(|s| -> u128 { s.parse().unwrap() })
        .fold(0_u128, |acc, x| acc | (1 << x));

    let matches = (winning & player).count_ones() as i64;

    if matches != 0 {
        1 << (matches - 1)
    } else {
        0
    }
}

#[aoc(day4, part1)]
pub fn part1(input: &str) -> i64 {
    input.lines().map(score_line).sum()
}

// Part2 ========================================================================
type Game = [i64; 100];

#[aoc(day4, part2)]
pub fn part2(input: &str) -> i64 {
    let mut game = [0; 100];

    let mut cards = vec![];

    for line in input.lines() {
        let (card_name, line) = line.split_once(':').unwrap();
        let (_, cid) = card_name.split_once(' ').unwrap();
        let cid: i64 = cid.trim().parse().unwrap();

        let (w, p) = line.split_once('|').unwrap();

        let mut w_set = w
            .split_whitespace()
            .map(|s| -> u128 { s.parse().unwrap() })
            .fold(0_u128, |acc, x| acc | (1 << x));

        let mut p_set = p
            .split_whitespace()
            .map(|s| -> u128 { s.parse().unwrap() })
            .fold(0_u128, |acc, x| acc | (1 << x));

        let score = (w_set & p_set).count_ones();

        cards.push((cid, score));
    }

    let mut r = 0;
    loop {
        dbg!(cards.len());

        if r >= cards.len() {
            break;
        }

        let (cid, score) = cards[r];

        for j in r..cards.len() {
            let c = cards[j];
            for _ in 0..score {
                cards.push(c);
            }
        }

        r += 1;
    }

    r as i64
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    #[rstest]
    #[case::card1(8, "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53")]
    #[case::card2(2, "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19")]
    #[case::card3(2, "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1")]
    #[case::card4(1, "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83")]
    #[case::card5(0, "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36")]
    #[case::card6(0, "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11")]
    #[trace]
    fn check_scoring(#[case] score: i64, #[case] line: &str) {
        let actual = score_line(line);
        assert_eq!(actual, score);
    }

    #[rstest]
    #[case::given(13, EXAMPLE_INPUT)]
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
    #[case::given(30, EXAMPLE_INPUT)]
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
