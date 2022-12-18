use crate::prelude::*;

use std::collections::VecDeque;

fn parse(input: &str) -> impl Iterator<Item = IVec3> + '_ {
    input.lines().map(|line| {
        let nums: [&str; 3] = iter_to_array(line.split(','));
        IVec3::new(
            nums[0].parse().unwrap(),
            nums[1].parse().unwrap(),
            nums[2].parse().unwrap(),
        )
    })
}

// Part1 ========================================================================
#[aoc(day18, part1)]
pub fn part1(input: &str) -> i64 {
    let cubes: HashSet<_> = parse(input).collect();
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
fn in_bounds(xyz: IVec3, min_bounds: IVec3, max_bounds: IVec3) -> bool {
    (min_bounds.x <= xyz.x && xyz.x <= max_bounds.x)
        && (min_bounds.y <= xyz.y && xyz.y <= max_bounds.y)
        && (min_bounds.z <= xyz.z && xyz.z <= max_bounds.z)
}

#[aoc(day18, part2)]
pub fn part2(input: &str) -> i64 {
    let cubes: HashSet<_> = parse(input).collect();
    let first: IVec3 = *cubes.iter().next().unwrap();

    let min_bounds =
        cubes.iter().fold(first, |acc, c| acc.min_by_component(*c)) - IVec3::new(1, 1, 1);

    let max_bounds =
        cubes.iter().fold(first, |acc, c| acc.max_by_component(*c)) + IVec3::new(1, 1, 1);

    let mut explored: HashSet<IVec3> = HashSet::new();
    let mut to_explore: VecDeque<IVec3> = VecDeque::new();
    to_explore.push_back(max_bounds);

    let mut faces = 0;

    while let Some(a) = to_explore.pop_front() {
        if cubes.contains(&a) {
            faces += 1;
            continue;
        }

        if explored.contains(&a) {
            continue;
        }

        explored.insert(a);

        for (dx, dy, dz) in [
            (1, 0, 0),
            (0, 1, 0),
            (0, 0, 1),
            (-1, 0, 0),
            (0, -1, 0),
            (0, 0, -1),
        ] {
            let xyz = a + IVec3::new(dx, dy, dz);
            if !explored.contains(&xyz) && in_bounds(xyz, min_bounds, max_bounds) {
                to_explore.push_back(xyz);
            }
        }
    }

    faces
}

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

    #[rstest]
    #[case::given(58, EXAMPLE_INPUT)]
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
