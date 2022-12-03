// The 0s are for padding, leave me alone
#![allow(clippy::identity_op)]

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
fn parse(input: &str) -> String {
    // Pad to 4*N length, make sure to end in newline
    let mut s = input.to_string();
    s.push('\n');
    s
}

// Part1 ========================================================================
#[aoc(day2, part1, as_str)]
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

#[aoc(day2, part1, as_bytes)]
#[inline(never)]
pub fn part1_as_bytes(input: &str) -> i64 {
    input
        .as_bytes()
        .split(|b| *b == b'\n')
        .map(|line| match line {
            b"" => 0,
            b"A X" => 3 + 1, // rock_rock
            b"A Y" => 6 + 2, // rock_paper
            b"A Z" => 0 + 3, // rock_scissors
            //
            b"B X" => 0 + 1, // paper_rock
            b"B Y" => 3 + 2, // paper_paper
            b"B Z" => 6 + 3, // paper_scissors
            //
            b"C X" => 6 + 1, // scissors_rock
            b"C Y" => 0 + 2, // scissors_paper
            b"C Z" => 3 + 3, // scissors_scissors

            _ => unreachable!(),
        })
        .sum()
}

#[aoc(day2, part1, as_u32)]
#[inline(never)]
pub fn part1_as_u32(input: &str) -> i64 {
    let bytes = input.trim_start().as_bytes();
    debug_assert_eq!(bytes.len() % 4, 0);

    let words: &[u32] = unsafe {
        let ptr: *const u32 = bytes.as_ptr() as _;
        let len = bytes.len() / 4;

        std::slice::from_raw_parts(ptr, len)
    };

    const LUT: [u8; 12] = [0, 4, 1, 7, 0, 8, 5, 2, 0, 3, 9, 6];

    let mut score: i64 = 0;
    for word in words {
        let word = ((word >> 14) | word) & 0b1111;

        if cfg!(debug_assertions) {
            score += LUT[word as usize] as i64;
        } else {
            unsafe {
                score += *LUT.get_unchecked(word as usize) as i64;
            }
        }
    }

    fn make_lut() -> [u8; 12] {
        unsafe {
            let mut lut = [0_u8; 12];

            for (word, val) in [
                (std::mem::transmute(*b"A X\n"), 3 + 1),
                (std::mem::transmute(*b"A Y\n"), 6 + 2),
                (std::mem::transmute(*b"A Z\n"), 0 + 3),
                //
                (std::mem::transmute(*b"B X\n"), 0 + 1),
                (std::mem::transmute(*b"B Y\n"), 3 + 2),
                (std::mem::transmute(*b"B Z\n"), 6 + 3),
                //
                (std::mem::transmute(*b"C X\n"), 6 + 1),
                (std::mem::transmute(*b"C Y\n"), 0 + 2),
                (std::mem::transmute(*b"C Z\n"), 3 + 3),
            ] {
                let word: u32 = word;
                let word = word & 0b0000_0000_0011_0000_0000_0000_0011;
                let word = (word >> 14) | (word & 0b11);

                // println!("0b{word:32b} ({word}) -> {val}");
                /*
                    0b1010010110000010000001000001 -> 4
                    0b1010010110010010000001000001 -> 8
                    0b1010010110100010000001000001 -> 3
                    0b1010010110000010000001000010 -> 1
                    0b1010010110010010000001000010 -> 5
                    0b1010010110100010000001000010 -> 9
                    0b1010010110000010000001000011 -> 7
                    0b1010010110010010000001000011 -> 2
                    0b1010010110100010000001000011 -> 6

                    0b0000000000110000000000000011 : mask
                */
                lut[word as usize] = val;
            }

            for x in lut {
                println!("    {x},");
            }

            lut
        }
    }

    if cfg!(debug_assertions) {
        debug_assert_eq!(make_lut(), LUT);
    }

    score
}

#[cfg(all(feature = "simd", target_feature = "neon"))]
#[aoc(day2, part1, simd)]
#[inline(never)]
pub fn part1_as_simd(input: &str) -> i64 {
    let bytes = input.trim_start().as_bytes();
    debug_assert_eq!(bytes.len() % 4, 0);

    let words: &[u32] = unsafe {
        let ptr: *const u32 = bytes.as_ptr() as _;
        let len = bytes.len() / 4;

        std::slice::from_raw_parts(ptr, len)
    };

    const LUT: [u8; 16] = [0, 4, 1, 7, 0, 8, 5, 2, 0, 3, 9, 6, 0, 0, 0, 0];

    let mut score: i64 = 0;

    fn score_8_game(words: &[u32]) -> u8 {
        debug_assert_eq!(words.len(), 8);

        unsafe {
            use std::arch::aarch64::*;
            use std::arch::asm;

            let lut: uint8x16_t = std::mem::transmute(LUT);
            let mut score: u8;

            // Each "word" is layed out in memory like so:
            //      bits:     0000_0000 0000_0011 0000_0000 0000_0011
            //      LE bytes: [     3 ] [     2 ] [     1 ] [     0 ]
            //      names:          t2       xyz        t1       abc
            // We're only intested in xyz and abc, t1 and t2 can be ignored.
            asm! {
                "LD4.8b         {{ {abc}, {t1}, {xyz}, {t2} }}, [{p_words}]",

                "MOVI.8b        {t1}, #7",
                "AND.8b         {abc}, {abc}, {t1}",
                "AND.8b         {xyz}, {xyz}, {t1}",

                "SHL.8b         {xyz}, {xyz}, #2",
                "ORR.8b         {t1}, {xyz}, {abc}",
                "TBL.8b         {t1}, {{ {lut:v} }}, {t1}",
                "ADDV.8b        {score:b}, {t1}",

                abc = out(vreg) _,
                t1 = out(vreg) _,
                xyz = out(vreg) _,
                t2 = out(vreg) _,

                p_words=in(reg) words.as_ptr(),
                lut=in(vreg) lut,
                score=out(vreg) score,
            };

            score
        }
    }

    let m = 8 * (words.len() / 8);
    for i in (0..m).step_by(8) {
        let j = i + 8;

        score += score_8_game(&words[i..j]) as i64;
    }
    let r = words.len() - m;
    let s = words.len() - r;
    for word in &words[s..] {
        let word = (word >> 14) | (word & 0b11);
        let word = (word & 0b1111) as usize;
        score += LUT[word] as i64;
    }

    fn make_lut() -> [u8; 16] {
        unsafe {
            let mut lut = [0_u8; 16];

            for (word, val) in [
                (std::mem::transmute(*b"A X\n"), 3 + 1),
                (std::mem::transmute(*b"A Y\n"), 6 + 2),
                (std::mem::transmute(*b"A Z\n"), 0 + 3),
                //
                (std::mem::transmute(*b"B X\n"), 0 + 1),
                (std::mem::transmute(*b"B Y\n"), 3 + 2),
                (std::mem::transmute(*b"B Z\n"), 6 + 3),
                //
                (std::mem::transmute(*b"C X\n"), 6 + 1),
                (std::mem::transmute(*b"C Y\n"), 0 + 2),
                (std::mem::transmute(*b"C Z\n"), 3 + 3),
            ] {
                let word: u32 = word;
                let word = word & 0b0000_0000_0011_0000_0000_0000_0011;
                let word = (word >> 14) | (word & 0b11);

                // println!("0b{word:32b} ({word}) -> {val}");
                /*
                    0b1010010110000010000001000001 -> 4
                    0b1010010110010010000001000001 -> 8
                    0b1010010110100010000001000001 -> 3
                    0b1010010110000010000001000010 -> 1
                    0b1010010110010010000001000010 -> 5
                    0b1010010110100010000001000010 -> 9
                    0b1010010110000010000001000011 -> 7
                    0b1010010110010010000001000011 -> 2
                    0b1010010110100010000001000011 -> 6

                    0b0000000000110000000000000011 : mask
                */
                lut[word as usize] = val;
            }

            for x in lut {
                println!("    {x},");
            }

            lut
        }
    }

    if cfg!(debug_assertions) {
        debug_assert_eq!(make_lut(), LUT);
    }

    score
}

// Part2 ========================================================================
#[aoc(day2, part2, as_str)]
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

#[aoc(day2, part2, as_bytes)]
#[inline(never)]
pub fn part2_as_bytes(input: &str) -> i64 {
    input
        .as_bytes()
        .split(|b| *b == b'\n')
        .map(|line| match line {
            b"" => 0,

            b"A X" => 0 + 3, // Rock/Lose, pick Scissors(3)
            b"A Y" => 3 + 1, // Rock/Draw, pick Rock(1)
            b"A Z" => 6 + 2, // Rock/Win, pick Paper(2)
            //
            b"B X" => 0 + 1, // Paper/Lose, pick Rock(1)
            b"B Y" => 3 + 2, // Paper/Draw, pick Paper(2)
            b"B Z" => 6 + 3, // Paper/Win, pick Scissors(3)
            //
            b"C X" => 0 + 2, // Scissors/Lose, pick Paper(2)
            b"C Y" => 3 + 3, // Scissors/Draw, pick Scissors(3)
            b"C Z" => 6 + 1, // Scissors/Win, pick Rock(1)

            _ => unreachable!(),
        })
        .sum()
}

#[aoc(day2, part2, as_u32)]
#[inline(never)]
pub fn part2_as_u32(input: &str) -> i64 {
    let bytes = input.trim_start().as_bytes();
    debug_assert_eq!(bytes.len() % 4, 0);

    let words: &[u32] = unsafe {
        let ptr: *const u32 = bytes.as_ptr() as _;
        let len = bytes.len() / 4;

        std::slice::from_raw_parts(ptr, len)
    };

    const LUT: [u8; 12] = [0, 3, 1, 2, 0, 4, 5, 6, 0, 8, 9, 7];

    let mut score: i64 = 0;
    for word in words {
        let word = word & 0b0000_0000_0011_0000_0000_0000_0011;
        let word = (word >> 14) | (word & 0b11);
        score += LUT[word as usize] as i64;
    }

    fn make_lut() -> [u8; 12] {
        unsafe {
            let mut lut = [0_u8; 12];

            for (word, val) in [
                (std::mem::transmute(*b"A X\n"), 0 + 3),
                (std::mem::transmute(*b"A Y\n"), 3 + 1),
                (std::mem::transmute(*b"A Z\n"), 6 + 2),
                //
                (std::mem::transmute(*b"B X\n"), 0 + 1),
                (std::mem::transmute(*b"B Y\n"), 3 + 2),
                (std::mem::transmute(*b"B Z\n"), 6 + 3),
                //
                (std::mem::transmute(*b"C X\n"), 0 + 2),
                (std::mem::transmute(*b"C Y\n"), 3 + 3),
                (std::mem::transmute(*b"C Z\n"), 6 + 1),
            ] {
                let word: u32 = word;
                let word = word & 0b0000_0000_0011_0000_0000_0000_0011;
                let word = (word >> 14) | (word & 0b11);

                // println!("0b{word:32b} ({word}) -> {val}");
                /*
                    0b1010010110000010000001000001 -> 0 + 3
                    0b1010010110010010000001000001 -> 3 + 1
                    0b1010010110100010000001000001 -> 6 + 2
                    0b1010010110000010000001000010 -> 0 + 1
                    0b1010010110010010000001000010 -> 3 + 2
                    0b1010010110100010000001000010 -> 6 + 3
                    0b1010010110000010000001000011 -> 0 + 2
                    0b1010010110010010000001000011 -> 3 + 3
                    0b1010010110100010000001000011 -> 6 + 1

                    0b0000000000110000000000000011 : mask
                */
                lut[word as usize] = val;
            }

            for x in lut {
                println!("    {x},");
            }

            lut
        }
    }

    if cfg!(debug_assertions) {
        debug_assert_eq!(make_lut(), LUT);
    }

    score
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

    // Long enough to trigger SIMD, but not an even multiple of it
    const LONG_EXAMPLE_INPUT: &str = r"
A Y
B X
C Z
A Y
B X
C Z
A Y
B X
C Z
";
    // AX
    // BY
    // CZ

    #[rstest]
    #[case::given(15, EXAMPLE_INPUT)]
    #[case::long_given(3*15, LONG_EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1, part1_as_bytes, part1_as_u32, part1_as_simd)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        let input = input.trim_start();
        assert_eq!(p(input), expected);
    }

    #[rstest]
    #[case::given(12, EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2, part2_as_bytes, part2_as_u32)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        let input = input.trim_start();
        assert_eq!(p(input), expected);
    }
}
