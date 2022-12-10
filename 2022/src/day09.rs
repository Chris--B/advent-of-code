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

    let mut seen = Framebuffer::new_with_ranges_square(-200..200);

    // Mark our starting point (our origin too) as seen
    let mut count = 1;
    seen[(0, 0)] = true;

    if cfg!(debug_assertions) {
        seen.print_range_with(
            0..6,
            0..5,
            |x, y, _: &bool| {
                if (x, y) == (0, 0) {
                    'H'
                } else {
                    '.'
                }
            },
        );
    }

    for dir in moves {
        // Move head
        head += dir;

        let dist = head - tail;

        // Move tail if it's no longer adjacent
        if dist.abs().component_max() > 1 {
            tail += IVec2::new(sign(dist.x), sign(dist.y))
        }

        if !seen[tail] {
            count += 1;
            seen[tail] = true;
        }

        if cfg!(debug_assertions) {
            let rope = [('H', head), ('T', tail), ('s', IVec2::zero())];
            seen.print_range_with(0..6, 0..5, |x, y, _s| {
                if let Some((c, _)) = rope.iter().find(|(_c, pt)| *pt == (x, y).into()) {
                    *c
                } else if (x, y) == (0, 0) {
                    's'
                } else {
                    '.'
                }
            });
        }
    }

    count
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
    let mut seen = Framebuffer::new_with_ranges_square(-200..200);

    // Mark our starting point (our origin too) as seen
    let mut count = 1;
    seen[(0, 0)] = true;

    if cfg!(debug_assertions) {
        println!("== Initial State ==");
        seen.print_range_with(
            -11..15,
            -5..16,
            |x, y, _: &bool| {
                if (x, y) == (0, 0) {
                    'H'
                } else {
                    '.'
                }
            },
        );
    }

    'moves: for dir in moves {
        // Move head
        rope[0] += dir;

        // Update the rest of the rope movement
        for i in 0..(rope.len() - 1) {
            let prev = rope[i];
            let next = &mut rope[i + 1];
            let dist = prev - *next;

            // Move next if it's no longer adjacent
            if dist.abs().component_max() > 1 {
                *next += IVec2::new(sign(dist.x), sign(dist.y))
            } else {
                // If this knot didn't move, nothing after it will either.
                // Early out, because nothing has changed.
                //
                // Note: THIS STOPS THE BOARD FROM PRINTING!
                continue 'moves;
            }
        }

        // Mark where the final tail node has been
        let tail = rope[ROPE_LEN - 1];
        if !seen[tail] {
            count += 1;
            seen[tail] = true;
        }

        // Pretty print the grid
        if cfg!(debug_assertions) && dir == IVec2::zero() {
            let labels = ['H', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

            seen.print_range_with(-11..15, -5..16, |x, y, _: &bool| {
                if let Some(pos) = rope.iter().position(|pt| *pt == (x, y).into()) {
                    return labels[pos];
                }

                if (x, y) == (0, 0) {
                    's'
                } else {
                    '.'
                }
            });

            continue;
        }
    }

    if cfg!(debug_assertions) {
        let bounds = seen.content_bounds();
        dbg!(bounds);
    }

    count
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
