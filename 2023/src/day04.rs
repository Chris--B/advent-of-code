use crate::prelude::*;

fn parse_matches_count(input: &'_ str) -> impl Iterator<Item = usize> + '_ {
    input.lines().filter(|l| !l.is_empty()).map(|line| {
        let (_, line) = line.split_once(':').unwrap();
        let (winning, player) = line.split_once('|').unwrap();

        let winning = winning
            .split_whitespace()
            .map(|s| -> u128 { s.parse::<u8>().unwrap() as _ })
            .fold(0_u128, |acc, x| acc | (1 << x));

        let player = player
            .split_whitespace()
            .map(|s| -> u128 { s.parse::<u8>().unwrap() as _ })
            .fold(0_u128, |acc, x| acc | (1 << x));

        (winning & player).count_ones() as usize
    })
}

// Part1 ========================================================================
#[aoc(day4, part1)]
pub fn part1(input: &str) -> i64 {
    parse_matches_count(input)
        .filter(|m| *m != 0)
        .map(|m| 1 << (m - 1))
        .sum()
}

// Part2 ========================================================================
#[aoc(day4, part2)]
pub fn part2(input: &str) -> i64 {
    let card_mcount: Vec<_> = parse_matches_count(input).collect();
    let n_cards = card_mcount.len();
    let mut card_copies = vec![1; n_cards];

    for cid in 0..(n_cards - 1) {
        let copies = card_copies[cid];
        let mcount = card_mcount[cid];

        for c in ((cid + 1)..).take(mcount) {
            if cfg!(debug_assertions) {
                println!("[{cid}] {copies} instances of Card {cid} have {mcount} matching numbers, so you win {copies}");
            }
            card_copies[c] += copies;
        }
        if cfg!(debug_assertions) {
            if mcount == 0 {
                println!("[{cid}] No matches");
            }

            println!(
                "[{cid}] {{\n    {}\n}}",
                card_copies
                    .iter()
                    .enumerate()
                    .map(|(i, n)| format!("Card {i}: {n}"))
                    .join("\n    ")
            );
            println!();
        }
    }

    card_copies.iter().sum()
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
