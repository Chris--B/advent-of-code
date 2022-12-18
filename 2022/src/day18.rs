use crate::prelude::*;

// Part1 ========================================================================
#[aoc(day18, part1)]
pub fn part1(input: &str) -> i64 {
    let cubes: Vec<_> = input
        .lines()
        .map(|line| {
            let nums: [&str; 3] = iter_to_array(line.split(','));
            IVec3::new(
                nums[0].parse().unwrap(),
                nums[1].parse().unwrap(),
                nums[2].parse().unwrap(),
            )
        })
        .collect();

    let mut faces = 0;

    for a in &cubes {
        for (dx, dy, dz) in [
            (1, 0, 0),
            (0, 1, 0),
            (0, 0, 1),
            (-1, 0, 0),
            (0, -1, 0),
            (0, 0, -1),
        ] {
            let xyz = *a + IVec3::new(dx, dy, dz);
            if !cubes.contains(&xyz) {
                faces += 1;
            }
        }
    }

    faces
}

// Part2 ========================================================================
// #[aoc(day18, part2)]
// pub fn part2(input: &str) -> i64 {
//     unimplemented!();
// }

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
";

    #[rstest]
    #[case::given(64, EXAMPLE_INPUT)]
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

    // #[rstest]
    // #[case::given(999_999, EXAMPLE_INPUT)]
    // #[trace]
    // fn check_ex_part_2(
    //     #[notrace]
    //     #[values(part2)]
    //     p: impl FnOnce(&str) -> i64,
    //     #[case] expected: i64,
    //     #[case] input: &str,
    // ) {
    //     let input = input.trim();
    //     assert_eq!(p(input), expected);
    // }
}
