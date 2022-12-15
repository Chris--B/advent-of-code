use crate::prelude::*;

#[derive(Copy, Clone, Debug, Default)]
struct SensorDesc {
    at: IVec2,
    closest: IVec2,
    dist: i32,
}

fn parse_sensor_desc(input: &str) -> SensorDesc {
    let mut bytes: &[u8] = input.as_bytes();
    let mut desc = SensorDesc::default();

    /*
        "Sensor at x=2, y=18: closest beacon is at x=-2, y=15"
    */

    // skip "Sensor at x="
    bytes = &bytes[12..];

    // parse x
    let cut = bytes.iter().position(|b| *b == b',').unwrap();
    desc.at.x = fast_parse_i32(&bytes[..cut]);
    bytes = &bytes[cut..];

    // skip ", y="
    bytes = &bytes[4..];

    // parse y
    let cut = bytes.iter().position(|b| *b == b':').unwrap();
    desc.at.y = fast_parse_i32(&bytes[..cut]);
    bytes = &bytes[cut..];

    // skip ": closest beacon is at x="
    bytes = &bytes[25..];

    // parse x
    let cut = bytes.iter().position(|b| *b == b',').unwrap();
    desc.closest.x = fast_parse_i32(&bytes[..cut]);
    bytes = &bytes[cut..];

    // skip ", y="
    bytes = &bytes[4..];

    // parse y
    desc.closest.y = fast_parse_i32(bytes);

    // cache it cuz why not
    desc.dist = manhattan(desc.closest, desc.at);

    desc
}

fn manhattan(a: IVec2, b: IVec2) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

// Part1 ========================================================================
#[aoc(day15, part1)]
pub fn part1(input: &str) -> i64 {
    let sensors: SmallVec<[SensorDesc; 16]> = input.lines().map(parse_sensor_desc).collect();
    let (min_x, max_x) = sensors
        .iter()
        .flat_map(|desc| [desc.at.x + desc.dist, desc.at.x - desc.dist].into_iter())
        .minmax()
        .into_option()
        .unwrap();
    let y = if cfg!(test) { 10 } else { 2_000_000 };

    let mut counter = 0;

    'xs: for x in min_x..=max_x {
        let here = IVec2::new(x, y);

        for sensor in sensors.iter() {
            // If we're closer (or tied) in distance to its closest beacon,
            // no other beacon can be here
            if manhattan(here, sensor.at) <= sensor.dist && here != sensor.closest {
                counter += 1;
                continue 'xs;
            }
        }
    }

    counter
}

// Part2 ========================================================================
// #[aoc(day15, part2)]
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
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";

    #[rstest]
    #[case::given(26, EXAMPLE_INPUT)]
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