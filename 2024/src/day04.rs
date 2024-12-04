#![allow(unused)]

use crate::prelude::*;

// Part1 ========================================================================
#[aoc(day4, part1)]
pub fn part1(input: &str) -> i64 {
    let mut crossword = Framebuffer::parse_grid(input, |c| c as u8);
    crossword.set_border_color(Some(b'.'));

    if cfg!(test) {
        crossword.print(|_, _, &b| b as char);
    }

    let mut count = 0;

    for (x, y) in crossword.iter_coords() {
        let horizontal = [
            crossword[(x, y)],
            crossword[(x + 1, y)],
            crossword[(x + 2, y)],
            crossword[(x + 3, y)],
        ];
        if &horizontal == b"XMAS" || &horizontal == b"SAMX" {
            count += 1;
        }

        let vertical = [
            crossword[(x, y)],
            crossword[(x, y + 1)],
            crossword[(x, y + 2)],
            crossword[(x, y + 3)],
        ];
        if &vertical == b"XMAS" || &vertical == b"SAMX" {
            count += 1;
        }

        let diag_up = [
            crossword[(x, y)],
            crossword[(x + 1, y + 1)],
            crossword[(x + 2, y + 2)],
            crossword[(x + 3, y + 3)],
        ];
        if &diag_up == b"XMAS" || &diag_up == b"SAMX" {
            count += 1;
        }

        let diag_down = [
            crossword[(x, y)],
            crossword[(x + 1, y - 1)],
            crossword[(x + 2, y - 2)],
            crossword[(x + 3, y - 3)],
        ];
        if &diag_down == b"XMAS" || &diag_down == b"SAMX" {
            count += 1;
        }
    }

    count
}

// Part2 ========================================================================
#[aoc(day4, part2)]
pub fn part2(input: &str) -> i64 {
    let mut crossword = Framebuffer::parse_grid(input, |c| c as u8);
    crossword.set_border_color(Some(b'.'));

    if cfg!(test) {
        crossword.print(|_, _, &b| b as char);
    }

    let mut count = 0;

    for (x, y) in crossword.iter_coords() {
        // (x,y) is @:
        //      ..S
        //      .@.
        //      M..
        let diag_up = [
            crossword[(x - 1, y - 1)],
            crossword[(x, y)],
            crossword[(x + 1, y + 1)],
        ];

        // (x,y) is @:
        //      S..
        //      .@.
        //      ..M
        let diag_down = [
            crossword[(x - 1, y + 1)],
            crossword[(x, y)],
            crossword[(x + 1, y - 1)],
        ];

        if (&diag_down == b"MAS" || &diag_down == b"SAM")
            && (&diag_up == b"MAS" || &diag_up == b"SAM")
        {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

    // Smallest test cases we can possible get right
    const SMOLL_P1_HORIZONTAL: &str = "XMAS";
    const SMOLL_P1_VERTICAL: &str = r"
X
M
A
S
";

    const SMOLL_P1_UP_DIAG: &str = r"
...S
..A.
.M..
X...
";
    const SMOLL_P1_DOWN_DIAG: &str = r"
X...
.M..
..A.
...S
";

    #[rstest]
    #[case::given(18, EXAMPLE_INPUT)]
    #[case::smoll_horizontal(1, SMOLL_P1_HORIZONTAL)]
    #[case::smoll_vertical(1, SMOLL_P1_VERTICAL)]
    #[case::smoll_up_diag(1, SMOLL_P1_UP_DIAG)]
    #[case::smoll_down_diag(1, SMOLL_P1_DOWN_DIAG)]
    #[trace]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        init_logging();

        let input = input.trim();
        assert_eq!(p(input), expected);
    }

    const SMOLL_P2_CROSS_MAS: &str = r"
M.S
.A.
M.S
";

    #[rstest]
    #[case::given(9, EXAMPLE_INPUT)]
    #[case::given_smoll(1, SMOLL_P2_CROSS_MAS)]
    #[trace]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        init_logging();

        let input = input.trim();
        assert_eq!(p(input), expected);
    }
}
