use crate::prelude::*;

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct Hand([u8; 5]);

const FIVE_OF_A_KIND: u8 = 7;
const FOUR_OF_A_KIND: u8 = 6;
const FULL_HOUSE: u8 = 5;
const THREE_OF_A_KIND: u8 = 4;
const TWO_PAIR: u8 = 3;
const ONE_PAIR: u8 = 2;
const HIGH_CARD: u8 = 1;

impl Hand {
    fn new(mut cards: [u8; 5]) -> Self {
        for b in &mut cards {
            *b = match *b {
                b'2'..=b'9' => *b - b'0',
                b'T' => 10,
                b'J' => 11,
                b'Q' => 12,
                b'K' => 13,
                b'A' => 14,
                _ => unreachable!("b={b}, {}", *b as char),
            };
        }

        for c in cards {
            assert!(c < 15);
        }

        Self(cards)
    }

    fn jokers(self) -> usize {
        self.0.iter().filter(|c| **c == 11).count()
    }

    fn kind_p2_v2(self) -> u8 {
        let mut counts = [0_i32; 15];
        for c in self.0 {
            counts[c as usize] += 1;
        }

        let n_jokers = counts[11];

        if n_jokers != 0 {
            // Remove all jokers
            counts[11] = 0;
            // Jokers should take the value of whatever the most common count is, instead of J
            let (hi_card, _) = counts
                .into_iter()
                .enumerate()
                .max_by_key(|(i, count)| (*count, *i))
                .unwrap();

            counts[hi_card] += n_jokers;
        }

        self.kind_from_counts(counts)
    }

    fn kind_p1(self) -> u8 {
        let mut counts: [i32; 15] = [0; 15];

        for c in self.0 {
            counts[c as usize] += 1;
        }

        self.kind_from_counts(counts)
    }

    fn kind_p2(self) -> u8 {
        let mut best_kind = 0;

        let mut queue = VecDeque::new();
        queue.push_back(self);

        while let Some(hand) = queue.pop_front() {
            if let Some(idx) = hand.0.iter().position(|c| *c == 11) {
                // If we can find a J, seek more hand variants to check
                for c in 2..=14 {
                    if c == 11 {
                        continue;
                    }

                    let mut h = hand;
                    h.0[idx] = c;

                    queue.push_back(h);
                }
            } else {
                // Otherwise record this and continue
                best_kind = best_kind.max(hand.kind_p1());
            }
        }

        best_kind
    }

    fn kind_from_counts(self, counts: [i32; 15]) -> u8 {
        // 7 == Five of a kind, where all five cards have the same label: AAAAA
        if counts.contains(&5) {
            return FIVE_OF_A_KIND;
        }

        // 6 == Four of a kind, where four cards have the same label and one card has a different label: AA8AA
        if counts.contains(&4) {
            return FOUR_OF_A_KIND;
        }

        if counts.contains(&3) {
            if counts.contains(&2) {
                // 5 == Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
                return FULL_HOUSE;
            } else {
                // 4 == Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
                return THREE_OF_A_KIND;
            }
        }

        // 3 == Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
        if counts.iter().filter(|c| **c == 2).count() == 2 {
            return TWO_PAIR;
        }

        // 2 == One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
        if counts.iter().filter(|c| **c == 2).count() == 1 {
            return ONE_PAIR;
        }

        // 1 == High card, where all cards' labels are distinct: 23456
        if counts.iter().filter(|c| **c == 1).count() == 5 {
            return HIGH_CARD;
        }

        unreachable!("Unknown hand: {self:?}");
    }
}

pub fn parse(s: &str) -> Vec<(Hand, i64)> {
    s.trim()
        .lines()
        .map(|line| {
            let line = line.trim().as_bytes();
            let hand = Hand::new(line[..5].try_into().unwrap());
            let rank = fast_parse_u32(&line[6..]);

            (hand, rank as _)
        })
        .collect()
}

// Part1 ========================================================================
#[aoc(day7, part1)]
pub fn part1(input: &str) -> i64 {
    let mut hands = parse(input);

    if cfg!(test) {
        for (hand, bid) in &hands {
            let k = hand.kind_p2();
            println!("bid={bid}, kind={k}, {hand:?}");
        }
    }

    hands.sort_by_key(|(hand, _bid)| (hand.kind_p1(), *hand));

    hands
        .iter()
        .enumerate()
        .map(|(i, (_h, bid))| bid * (i + 1) as i64)
        .sum()
}

// Part2 ========================================================================
#[aoc(day7, part2, v1)]
pub fn part2(input: &str) -> i64 {
    let mut hands = parse(input);

    hands.sort_by_key(|(hand, _bid)| {
        // For sorting, J sucks. Replace it with a lower value.
        let mut weak_js = *hand;
        for c in &mut weak_js.0 {
            if *c == 11 {
                *c = 1;
            }
        }
        assert_eq!(weak_js.jokers(), 0);

        // But use the original when computing kind
        (hand.kind_p2(), weak_js)
    });

    if cfg!(test) {
        for (hand, bid) in &hands {
            let k = hand.kind_p2();
            println!("bid={bid}, kind={k}, {hand:?}");
        }
    }

    hands
        .iter()
        .enumerate()
        .map(|(i, (_h, bid))| bid * (i + 1) as i64)
        .sum()
}

// Part2 ========================================================================
#[aoc(day7, part2, v2)]
pub fn part2_v2(input: &str) -> i64 {
    let mut hands = parse(input);

    hands.sort_by_key(|(hand, _bid)| {
        // For sorting, J sucks. Replace it with a lower value.
        let mut weak_js = *hand;
        for c in &mut weak_js.0 {
            if *c == 11 {
                *c = 1;
            }
        }
        debug_assert_eq!(weak_js.jokers(), 0);

        // But use the original when computing kind
        (hand.kind_p2_v2(), weak_js)
    });

    if cfg!(test) {
        for (hand, bid) in &hands {
            let k = hand.kind_p2_v2();
            println!("bid={bid}, kind={k}, {hand:?}");
            println!();
        }
    }

    hands
        .iter()
        .enumerate()
        .map(|(i, (_h, bid))| bid * (i + 1) as i64)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

    #[rstest]
    // Example input, line by line
    #[case::ex(ONE_PAIR, "32T3K")]
    #[case::ex(TWO_PAIR, "KK677")]
    #[case::ex(TWO_PAIR, "KTJJT")]
    #[case::ex(THREE_OF_A_KIND, "QQQJA")]
    #[case::ex(THREE_OF_A_KIND, "T55J5")]
    // Examples from rules
    #[case::ex(FIVE_OF_A_KIND, "AAAAA")]
    #[case::ex(FOUR_OF_A_KIND, "AA8AA")]
    #[case::ex(FULL_HOUSE, "23332")]
    #[case::ex(THREE_OF_A_KIND, "TTT98")]
    #[case::ex(TWO_PAIR, "23432")]
    #[case::ex(ONE_PAIR, "A23A4")]
    #[case::ex(HIGH_CARD, "23456")]
    #[trace]
    fn check_card_kind_p1(#[case] kind: u8, #[case] hand: &str) {
        let hand = Hand::new(hand.as_bytes().try_into().unwrap());
        assert_eq!(kind, hand.kind_p1());
    }

    #[rstest]
    // Example input, line by line
    #[case::ex(ONE_PAIR, "32T3K")]
    #[case::ex(TWO_PAIR, "KK677")]
    #[case::ex(FOUR_OF_A_KIND, "T55J5")]
    #[case::ex(FOUR_OF_A_KIND, "QQQJA")]
    #[case::ex(FOUR_OF_A_KIND, "KTJJT")]
    // Examples from rules
    #[case::ex(FIVE_OF_A_KIND, "AAAAA")]
    #[case::ex(FOUR_OF_A_KIND, "AA8AA")]
    #[case::ex(FULL_HOUSE, "23332")]
    #[case::ex(THREE_OF_A_KIND, "TTT98")]
    #[case::ex(TWO_PAIR, "23432")]
    #[case::ex(ONE_PAIR, "A23A4")]
    #[case::ex(FOUR_OF_A_KIND, "QJJQ2")]
    // Examples from rules but with more Js
    #[case::ex(FIVE_OF_A_KIND, "JJJJJ")]
    #[case::ex(FOUR_OF_A_KIND, "J78JJ")]
    #[case::ex(THREE_OF_A_KIND, "JJ234")]
    #[case::ex(THREE_OF_A_KIND, "TTT98")]
    #[case::ex(TWO_PAIR, "23432")]
    #[case::ex(ONE_PAIR, "A23A4")]
    #[case::ex(HIGH_CARD, "23456")]
    // Misc
    #[case::ex(FOUR_OF_A_KIND, "JJJ23")]
    #[case::ex(HIGH_CARD, "2A456")]
    #[case::ex(HIGH_CARD, "32456")]
    #[case::ex(FOUR_OF_A_KIND, "JJJ28")]
    #[case::ex(FIVE_OF_A_KIND, "JJJJK")]
    #[case::ex(FULL_HOUSE, "J2233")]
    #[case::ex(FIVE_OF_A_KIND, "J5JJ5")]
    #[trace]
    fn check_card_kind_p2(
        #[notrace]
        #[values(Hand::kind_p2, Hand::kind_p2_v2)]
        fn_kind: impl FnOnce(Hand) -> u8,

        #[case] kind: u8,
        #[case] hand: &str,
    ) {
        let hand = Hand::new(hand.as_bytes().try_into().unwrap());
        assert_eq!(kind, fn_kind(hand));
    }

    const EXAMPLE_1: &str = r"
3333J 2
2AAAJ 1
";

    const EXAMPLE_2: &str = r"
JKKK2 1
QQQQ2 2
";

    #[rstest]
    #[case::given(6440, EXAMPLE_INPUT)]
    #[case::given(1*1+2*2, EXAMPLE_1)]
    #[case::given(1*1+2*2, EXAMPLE_2)]
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
    #[case::given(5905, EXAMPLE_INPUT)]
    #[case::given(1*1+2*2, EXAMPLE_1)]
    #[case::given(5, EXAMPLE_2)]
    #[trace]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2, part2_v2)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(input), expected);
    }
}
