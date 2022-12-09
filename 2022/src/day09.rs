use crate::prelude::*;

// Part1 ========================================================================
#[aoc(day9, part1)]
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

    let mut seen = HashSet::with_capacity(3_000);
    seen.insert(tail);

    for dir in moves {
        // Move head
        head += dir;

        let dist = head - tail;

        // Move tail if it's no longer adjacent
        if dist.abs().component_max() > 1 {
            tail += IVec2::new(sign(dist.x), sign(dist.y))
        }

        seen.insert(tail);

        if cfg!(debug_assertions) {
            print_ex_grid(&[('H', head), ('T', tail), ('s', IVec2::zero())]);

            fn print_ex_grid(points: &[(char, IVec2)]) {
                for y in (0..5).rev() {
                    for x in 0..6 {
                        if let Some((c, _)) = points.iter().find(|(_c, pt)| *pt == (x, y).into()) {
                            print!("{c}");
                        } else if (x, y) == (0, 0) {
                            print!("s");
                        } else {
                            print!(".");
                        }
                    }
                    println!();
                }
                println!();
            }
        }
    }

    seen.len() as i64
}

// Part2 ========================================================================
#[aoc(day9, part2)]
pub fn part2(input: &str) -> i64 {
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
            // Uncomment to print the board after each move command (not individual steps)
            // .chain((0..1).map(|_| IVec2::zero()))
        });

    const ROPE_LEN: usize = 10;
    let mut rope = [IVec2::zero(); ROPE_LEN];
    let mut seen = HashSet::with_capacity(3_000);

    seen.insert(rope[ROPE_LEN - 1]);

    for dir in moves {
        // Pretty print the grid
        if cfg!(debug_assertions) && dir == IVec2::zero() {
            let chain = [
                ('H', rope[0]),
                ('1', rope[1]),
                ('2', rope[2]),
                ('3', rope[3]),
                ('4', rope[4]),
                ('5', rope[5]),
                ('6', rope[6]),
                ('7', rope[7]),
                ('8', rope[8]),
                ('9', rope[9]),
            ];
            print_ex_grid(&chain);

            continue;

            fn print_ex_grid(points: &[(char, IVec2)]) {
                for y in (-7..15).rev() {
                    for x in -11..15 {
                        if let Some((c, _)) = points.iter().find(|(_c, pt)| *pt == (x, y).into()) {
                            print!("{c}");
                        } else if (x, y) == (0, 0) {
                            print!("s");
                        } else {
                            print!(".");
                        }
                    }
                    println!();
                }
                println!();
            }
        }

        // Move head
        rope[0] += dir;

        // Update the rest of the rope movement
        for i in 0..(rope.len() - 1) {
            let lead = rope[i];
            let tail = &mut rope[i + 1];
            let dist = lead - *tail;

            // Move tail if it's no longer adjacent
            if dist.abs().component_max() > 1 {
                *tail += IVec2::new(sign(dist.x), sign(dist.y))
            }
        }

        // Mark where the final tail node has been
        seen.insert(rope[ROPE_LEN - 1]);
    }
    seen.len() as i64
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

    const EXAMPLE_INPUT_2: &str = r"
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
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

    #[rstest]
    #[case::given(1, EXAMPLE_INPUT)]
    #[case::given_2(36, EXAMPLE_INPUT_2)]
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
