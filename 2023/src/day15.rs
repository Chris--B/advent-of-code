use crate::prelude::*;

fn do_hash(ascii: impl IntoIterator<Item = u8>) -> usize {
    let mut cv = Wrapping(0_u8);

    for b in ascii {
        cv += b;
        cv *= 17;
    }

    cv.0 as usize
}

// Part1 ========================================================================
#[aoc(day15, part1)]
pub fn part1(input: &str) -> i64 {
    input
        .as_bytes()
        .split(|b| *b == b',')
        .map(|line| do_hash(line.iter().copied()) as i64)
        .sum()
}

// Part2 ========================================================================
type LensesBox<'a> = SmallVec<[(&'a str, u8); 8]>;

#[aoc(day15, part2)]
pub fn part2(input: &str) -> i64 {
    let mut boxes: SmallVec<[LensesBox; 256]> = smallvec![];
    boxes.resize_with(256, LensesBox::new);

    let mut ls = usize::MAX;
    let mut le = usize::MAX;
    let mut n = 0;
    let mut op = 0;

    for (i, b) in input.as_bytes().iter().chain([b','].iter()).enumerate() {
        let b = *b;

        match b {
            b'a'..=b'z' => {
                if ls == usize::MAX {
                    ls = i;
                } else {
                    le = i;
                }
            }

            b'0'..=b'9' => {
                assert_eq!(n, 0);
                n = b - b'0';
            }

            b'=' => {
                // add
                op = b'='
            }

            b'-' => {
                // remove
                op = b'-'
            }

            b',' => {
                let label: &str = &input[ls..=le];
                let h = do_hash(label.bytes());
                info!("{label} h={h}, n={n}, {ls:>4}..={le:>4}");

                debug_assert!(ls != usize::MAX);
                debug_assert!(le != usize::MAX);

                if op == b'-' {
                    debug_assert_eq!(n, 0);
                    if let Some(idx) = boxes[h]
                        .iter()
                        .position(|(slot_label, _lense)| *slot_label == label)
                    {
                        boxes[h].remove(idx);
                    }
                } else if op == b'=' {
                    debug_assert_ne!(n, 0);
                    if let Some(idx) = boxes[h]
                        .iter()
                        .position(|(slot_label, _lense)| *slot_label == label)
                    {
                        // Already there, replace it
                        boxes[h][idx] = (label, n);
                    } else {
                        // Not there, push it to the end
                        boxes[h].push((label, n));
                    }
                }

                n = 0;
                ls = usize::MAX;
                le = usize::MAX;
                op = 0;
            }
            _ => unreachable!("Unexpected character: {} (0x{b:x})", b as char),
        }
    }

    let focusing_power: i64 = boxes
        .iter()
        .enumerate()
        .flat_map(|(boxx_idx, boxx)| {
            boxx.iter()
                .enumerate()
                .map(move |(idx, (_lbl, focal))| -> i64 {
                    // The focusing power of a single lens is the result of multiplying together:
                    [
                        // One plus the box number of the lens in question.
                        1 + boxx_idx as i64,
                        // The slot number of the lens within the box: 1 for the first lens, 2 for the second lens, and so on.
                        1 + idx as i64,
                        // The focal length of the lens.
                        *focal as i64,
                    ]
                    .into_iter()
                    .product()
                })
        })
        .sum();

    focusing_power
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    #[test]
    fn check_hash_algo() {
        assert_eq!(do_hash(*b"HASH"), 52);
    }

    const EXAMPLE_INPUT: &str = r"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[rstest]
    #[case::given(1320, EXAMPLE_INPUT)]
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
    #[case::given(145, EXAMPLE_INPUT)]
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
