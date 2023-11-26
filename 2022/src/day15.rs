use crate::prelude::*;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
struct SensorDesc {
    at: IVec2,
    closest: IVec2,
    dist: i32,
}

impl SensorDesc {
    /// An iterator that yields coordinates just outside of this sensor's area
    fn iter_area_edge(&self) -> impl Iterator<Item = IVec2> + '_ {
        let at: IVec2 = self.at;
        let dist = self.dist + 1;
        (0..=dist)
            .map(move |dx| IVec2::new(dx, dist - dx))
            .flat_map(|dv| {
                [
                    IVec2::new(dv.x, dv.y),
                    IVec2::new(-dv.x, dv.y),
                    IVec2::new(dv.x, -dv.y),
                    IVec2::new(-dv.x, -dv.y),
                ]
                .into_iter()
            })
            .map(move |dv| dv + at)
    }

    fn in_range(&self, pt: IVec2) -> bool {
        manhattan(self.at, pt) <= self.dist
    }
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
pub fn part1(input: &str) -> i32 {
    let sensors: SmallVec<[SensorDesc; 32]> = input.lines().map(parse_sensor_desc).collect();
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
            if sensor.in_range(here) && here != sensor.closest {
                // if manhattan(here, sensor.at) <= sensor.dist && here != sensor.closest {
                counter += 1;
                continue 'xs;
            }
        }
    }

    counter
}

#[aoc(day15, part1, edge_walker)]
pub fn part1_edge_walker(input: &str) -> i32 {
    const Y: i32 = if cfg!(test) { 10 } else { 2_000_000 };

    let sensors: SmallVec<[SensorDesc; 32]> = input.lines().map(parse_sensor_desc).collect();

    // Find points on the periferie of sensor ranges - this will keep our search area small
    let points: HashSet<IVec2> = sensors
        .iter()
        .flat_map(|s| s.iter_area_edge().filter(|here| here.y == Y))
        .collect();

    // Find the range of points we care about
    let (min_x, max_x) = points
        .iter()
        .map(|here| here.x)
        .minmax()
        .into_option()
        .unwrap();

    // This the worst-case count of places to find a beacon
    let maybe_count = max_x - min_x;

    // This will actually cull them out
    let viable = points
        .iter()
        .copied()
        .filter(|here| {
            for sensor in &sensors {
                if *here == sensor.closest {
                    // skip all beacons
                    return false;
                }

                if sensor.in_range(*here) {
                    // if manhattan(*here, sensor.at) <= sensor.dist {
                    // skip any points in range of a sensor
                    return false;
                }
            }

            true
        })
        .count() as i32;

    // The difference is where we can NOT find a beacon
    maybe_count - viable
}

// Part2 ========================================================================
fn in_bounds(pt: &IVec2) -> bool {
    for n in pt.as_array() {
        if n < 0 {
            return false;
        }

        #[cfg(test)]
        if n > 20 {
            return false;
        }

        if n > 4_000_000 {
            return false;
        }
    }

    true
}

#[aoc(day15, part2)]
pub fn part2(input: &str) -> i64 {
    let sensors: SmallVec<[SensorDesc; 32]> = input.lines().map(parse_sensor_desc).collect();

    let mut located = IVec2::zero();

    for pt in sensors
        .iter()
        .flat_map(SensorDesc::iter_area_edge)
        .filter(in_bounds)
    {
        let mut valid = true;

        for sensor in &sensors {
            if sensor.in_range(pt) {
                valid = false;
                break;
            }
        }

        if valid {
            located = pt;
            break;
        }
    }

    located.x as i64 * 4_000_000 + located.y as i64
}

#[cfg(feature = "broken")]
#[derive(Copy, Clone, Debug)]
struct Line {
    o: IVec2,
    r: IVec2,
    dist: i32,
}

#[cfg(feature = "broken")]
impl Line {
    fn intersects_with(&self, other: &Line) -> Option<IVec2> {
        let p: Vec3 = self.o.xyz().into();
        let q: Vec3 = other.o.xyz().into();

        let r: Vec3 = self.r.xyz().into();
        let s: Vec3 = other.r.xyz().into();

        let rxs = r.cross(s);
        let q_p = q - p;

        // Lines are not parallel, so they intersect *somewhere*
        if rxs.z != 0. {
            let t = q_p.cross(s).z / rxs.z;
            let u = q_p.cross(r).z / rxs.z;

            if (0. <= t && t <= self.dist as f32) && // .
               (0. <= u && u <= other.dist as f32)
            {
                // In range!
                // dbg!(self, other, t, u);
                let pp = p + t * r;
                let qq = q + u * s;
                debug_assert_eq!(pp, qq);

                Some([pp.x as i32, pp.y as i32].into())
            } else {
                // Not in range, but would intersect if the line were longer
                None
            }
        } else {
            None
        }
    }
}

#[cfg(feature = "broken")]
// Uncomment when it works
#[aoc(day15, part2, math)]
pub fn part2_math(input: &str) -> i64 {
    let sensors: SmallVec<[SensorDesc; 32]> = input.lines().map(parse_sensor_desc).collect();

    let mut lines = vec![];

    for s in &sensors {
        let dist = s.dist + 1;
        let ends = [
            s.at + (-1, 0).into(),
            s.at + (0, -1).into(),
            s.at + (1, 0).into(),
            s.at + (0, 1).into(),
        ];

        for (mut a, mut b) in [
            (ends[0], ends[1]),
            (ends[1], ends[2]),
            (ends[2], ends[3]),
            (ends[3], ends[0]),
        ]
        .into_iter()
        {
            // Encode as point closet to origin + length vector
            if manhattan(a, IVec2::zero()) > manhattan(b, IVec2::zero()) {
                std::mem::swap(&mut a, &mut b);
            }

            lines.push(Line {
                o: a,
                r: b - a,
                dist,
            });
        }
    }

    let mut shared = vec![];

    let len = lines.len();
    for i in 0..len {
        for j in (i + 1)..len {
            if let Some(pt) = lines[i].intersects_with(&lines[j]) {
                shared.push(pt);
            }
        }
    }

    dbg!(shared.len());

    if cfg!(test) {
        debug_assert!(shared.contains(&(14, 11).into()));
    }

    let elf = shared
        .into_iter()
        .filter(in_bounds)
        .find(|here| !sensors.iter().any(|s| s.in_range(*here)))
        .unwrap();

    elf.x as i64 * 4_000_000 + elf.y as i64
}

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
        #[values(part1, part1_edge_walker)]
        p: impl FnOnce(&str) -> i32,
        #[case] expected: i32,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(input), expected);
    }

    #[cfg(feature = "broken")]
    #[rstest]
    #[case::given(56000011, EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2, part2_math)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(input), expected);
    }
}
