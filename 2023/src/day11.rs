use crate::prelude::*;

fn parse(input: &str) -> Vec<(i64, i64)> {
    let mut galaxies: Vec<(i64, i64)> = vec![];

    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c != '.' {
                galaxies.push((x as i64, y as i64));
            }
        }
    }

    info!("parsed {} galaxies", galaxies.len());

    galaxies
}

fn do_with_expansion(galaxies: &mut [(i64, i64)], expansion: i64) -> i64 {
    let mut is_expanded_x = [true; 200];
    let mut is_expanded_y = [true; 200];

    for (x, y) in &*galaxies {
        is_expanded_x[*x as usize] = false;
        is_expanded_y[*y as usize] = false;
    }

    for p in &mut *galaxies {
        let mut warped_x = 0;
        for x in 0..p.0 {
            if is_expanded_x[x as usize] {
                warped_x += expansion;
            } else {
                warped_x += 1;
            }
        }

        let mut warped_y = 0;
        for y in 0..p.1 {
            if is_expanded_y[y as usize] {
                warped_y += expansion;
            } else {
                warped_y += 1;
            }
        }

        // Modify in-place, after we do all of our looping
        *p = (warped_x, warped_y)
    }

    let mut total = 0;
    for (a, b) in galaxies.iter().cartesian_product(galaxies.iter()) {
        total += (b.0 - a.0).abs() + (b.1 - a.1).abs();
    }

    if expansion == 1_000_000 {
        let r = total / 2;
        assert!(r > 1016799176, "{r} is too low");
    }

    total / 2
}

// Part1 ========================================================================
#[aoc(day11, part1)]
#[no_mangle]
pub fn part1(input: &str) -> i64 {
    let mut galaxies = parse(input);
    do_with_expansion(&mut galaxies, 2)
}

// Part2 ========================================================================
#[aoc(day11, part2)]
pub fn part2(input: &str) -> i64 {
    let mut galaxies = parse(input);
    do_with_expansion(&mut galaxies, 1_000_000)
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

    #[rstest]
    #[case::given(374, EXAMPLE_INPUT)]
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
    #[case::given(10, 1030, EXAMPLE_INPUT)]
    #[case::given(100, 8410, EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_2(#[case] expansion: i64, #[case] expected: i64, #[case] input: &str) {
        let input = input.trim();
        let mut galaxies = parse(input);

        assert_eq!(do_with_expansion(&mut galaxies, expansion), expected);
    }
}
