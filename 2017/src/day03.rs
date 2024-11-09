#![allow(unused)]

use crate::prelude::*;

// Part1 ========================================================================
#[aoc(day3, part1)]
pub fn part1(input: &str) -> i32 {
    let num: i32 = input.parse().unwrap();

    let mut dir = [East, Norð, West, Souð]
        .into_iter()
        .map(Into::<IVec2>::into)
        .cycle()
        .peekable();

    // We turn on a cadence related to triangle numbers.
    // This three-tiered setup to track variables works, since triangle numbers are quadratic
    let mut steps_till_turn = 1;
    let mut next = 1;
    let mut iunno = 0;

    let mut p = IVec2::new(0, 0);
    for step in 1..num {
        // Step one every time
        p += *dir.peek().unwrap();
        steps_till_turn -= 1;

        // With decreasing frequency, we want to change directions
        if steps_till_turn == 0 {
            dir.next();
            steps_till_turn = next;
            if iunno % 2 == 0 {
                next += 1;
            }
            iunno += 1;
        }
    }

    p.x.abs() + p.y.abs()
}

// Part2 ========================================================================
#[aoc(day3, part2)]
pub fn part2(input: &str) -> i32 {
    use std::collections::HashMap;

    let num: i32 = input.parse().unwrap();
    let mut spiral_path = vec![];
    let mut grid: HashMap<IVec2, usize> = HashMap::new();

    // Populate our cache
    {
        let mut dir = [East, Norð, West, Souð]
            .into_iter()
            .map(Into::<IVec2>::into)
            .cycle()
            .peekable();

        // We turn on a cadence related to triangle numbers.
        // This three-tiered setup to track variables works, since triangle numbers are quadratic
        let mut steps_till_turn = 1;
        let mut next = 1;
        let mut iunno = 0;

        let mut p = IVec2::new(0, 0);

        spiral_path.push(p);
        grid.insert(p, 0);

        for step in 1..(num as usize) {
            // Step one every time
            p += *dir.peek().unwrap();
            steps_till_turn -= 1;

            // With decreasing frequency, we want to change directions
            if steps_till_turn == 0 {
                dir.next();
                steps_till_turn = next;
                if iunno % 2 == 0 {
                    next += 1;
                }
                iunno += 1;
            }

            spiral_path.push(p);
            grid.insert(p, step);
        }
    }

    // For each num, try and back fill our buffer
    let mut spiral_values = vec![0; spiral_path.len()];

    // Starts off simple enough
    spiral_values[0] = 1;

    // Skip 1 since we just hard-coded it.
    for i in 1..spiral_path.len() {
        // For each neighbor of this point, grab the spiral_values and add them in.
        // Note, there are a few hoops here:
        let point = spiral_path[i];
        spiral_values[i] = Cardinal::all_diag()
            .into_iter()
            .filter_map(|d| {
                let target = d.ivec2() + point;
                Some(spiral_values[*grid.get(&target)?])
            })
            .sum();

        // We only really need the first value bigger than our input, so we can run this until we find it.
        if spiral_values[i] > num {
            dbg!(i);
            return spiral_values[i];
        }
    }

    unreachable!();
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    /*
       Part 1 Spiral:
            17   16   15   14   13
            18    5    4    3   12
            19    6    1    2   11
            20    7    8    9   10
            21   22   23  ---> ...
    */
    #[rstest]
    #[case::given(0, "1")]
    #[case::given(3, "12")]
    #[case::given(2, "23")]
    #[case::given(31, "1024")]
    #[trace]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1)]
        p: impl FnOnce(&str) -> i32,
        #[case] expected: i32,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(input), expected);
    }

    // These tests test if we generate the spiral numbers right, but we don't use that for part2.
    // Easier to just comment them out I guess
    /*
        Part 2 Spiral:
            147  142  133  122   59
            304    5    4    2   57
            330   10    1    1   54
            351   11   23   25   26
            362  747  806  --->  ...
    */
    // #[rstest]
    // #[case::given("1", 1)]
    // #[case::given("2", 1)]
    // #[case::given("3", 2)]
    // #[case::given("4", 4)]
    // #[case::given("5", 5)]
    // #[case::given("7", 11)]
    // #[case::given("22", 747)]
    // #[case::given("23", 806)]
    // #[trace]
    // fn check_ex_part_2(
    //     #[notrace]
    //     #[values(part2)]
    //     p: impl FnOnce(&str) -> i32,
    //     #[case] input: &str,
    //     #[case] expected: i32,
    // ) {
    //     let input = input.trim();
    //     assert_eq!(p(input), expected);
    // }
}
