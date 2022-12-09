use crate::prelude::*;

fn print_ex_grid(points: &[(char, IVec2)]) {
    for y in (0..5).rev() {
        for x in 0..6 {
            if let Some((c, _)) = points.iter().find(|(_c, pt)| *pt == (x, y).into()) {
                print!("{c}");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

// Part1 ========================================================================
#[aoc(day9, part1)]
#[inline(never)]
pub fn part1(input: &str) -> i64 {
    let moves = input
        .as_bytes()
        .split(|b| *b == b'\n') // really need ".ascii_lines()"...
        .flat_map(|line| {
            let dir = match line[0] {
                b'R' => IVec2::new(1, 0),
                b'L' => IVec2::new(-1, 0),
                b'U' => IVec2::new(0, 1),
                b'D' => IVec2::new(0, -1),
                c => unreachable!("Unrecognized move direction '{c}'"),
            };
            let steps = fast_parse_u8(&line[2..]) as i32;

            (0..steps).map(move |_| dir)
        });

    let mut head = IVec2::zero();
    let mut tail = IVec2::zero();

    let mut seen = HashSet::new();
    seen.insert(tail);

    for dir in moves {
        // Move head
        head += dir;

        let dist = head - tail;

        // Move tail if it's no longer adjacent
        if dist.abs().component_max() > 1 {
            match dist.as_array() {
                [0, 0] => unreachable!("Shouldn't have gotten here"),
                [x, 0] => tail += (sign(x), 0).into(),
                [0, y] => tail += (0, sign(y)).into(),
                [x, y] => tail += (sign(x), sign(y)).into(),
            }
        }

        seen.insert(tail);

        if cfg!(debug_assertions) {
            print_ex_grid(&[('H', head), ('T', tail), ('s', IVec2::zero())]);
        }
    }

    seen.len() as i64
}

// Part2 ========================================================================
#[aoc(day9, part2)]
#[inline(never)]
pub fn part2(input: &str) -> i64 {
    unimplemented!();
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

    #[rstest]
    #[case::given(13, EXAMPLE_INPUT)]
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
