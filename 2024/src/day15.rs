use crate::{framebuffer::ParsingInfo, prelude::*};

fn c_to_dir(c: char) -> Cardinal {
    match c {
        '^' => Norð,
        'v' | 'V' => Souð,
        '>' => East,
        '<' => West,
        _ => unreachable!("Invalid direction character {c:?}"),
    }
}

// Part1 ========================================================================
#[aoc(day15, part1)]
pub fn part1(input: &str) -> i64 {
    let (map, moves) = input.split_once("\n\n").unwrap();

    let mut robot = IVec2::zero();
    let mut map = Framebuffer::parse_grid2(map, |info| {
        if info.c == '@' {
            robot = IVec2::new(info.x, info.y);
            '@'
        } else {
            info.c
        }
    });

    if cfg!(test) {
        println!("{} moves={moves:?}", moves.len());
        println!();
        println!("Initial State:");
        map.just_print();
    }

    for c in moves.lines().flat_map(str::chars) {
        let dir: IVec2 = c_to_dir(c).into();
        let next = robot + dir;

        match map[next] {
            '.' => {
                map[robot] = '.';
                map[next] = '@';
                robot = next;
            }
            '#' => {
                // Nothing happens, do nothing.
                if cfg!(test) {
                    println!("  + BONK! Wall.");
                    println!();
                }
            }
            'O' => {
                // We found a box! Let's see if we can move it or not.
                let mut count = 0;
                let mut n = next;
                while map[n] == 'O' {
                    n += dir;
                    count += 1;
                }

                if map[n] == '.' {
                    // We can move the boxes! (Just update the start and end, since the whole chain is identical)
                    map[next] = '.';
                    map[n] = 'O';

                    // And move the robot
                    map[robot] = '.';
                    map[next] = '@';
                    robot = next;
                } else {
                    if cfg!(test) {
                        println!(
                            "  + BONK! Tried to move {count} boxes, but pushing against {:?}",
                            map[n]
                        );
                        println!();
                    }
                }
            }
            cc => {
                println!("Bad State");
                map.just_print();
                unreachable!("Unexpected map character: {cc}");
            }
        }

        if cfg!(test) {
            println!("Move {c}:");
            map.just_print();
            println!();
        }
    }

    // Compute GPS scores
    let maxy = map.height() as i32;
    map.iter_coords()
        .filter_map(|(x, y)| {
            if map[(x, y)] == 'O' {
                // Our y axis is inverted from AOC's
                let y = maxy - y - 1;
                Some((x + 100 * y) as i64)
            } else {
                None
            }
        })
        .sum()
}

// Part2 ========================================================================
fn has_a_box(thing: IVec2, boxes: &[IVec2]) -> Option<usize> {
    boxes
        .iter()
        .position(|&b| thing == b || thing == (b + East.into()))
}

fn sanity_check_boxes(boxes: &[IVec2]) {
    if cfg!(debug_assertions) {
        // Make sure no boxes overlap
        let l = boxes.len();
        for id in 0..l {
            let matches = boxes
                .iter()
                .filter(|&&b| boxes[id] == b || boxes[id] == (b + East.into()))
                .collect_vec();
            assert_eq!(
                matches,
                &[&boxes[id]],
                "Tried to find box id={id}, but found 2 boxes instead!"
            );
        }
    }
}

/// Try to move boxes given a box and direction
fn try_to_move_boxes(
    start: usize,
    dir: IVec2,
    boxes: &[IVec2],
    map: &Framebuffer<char>,
) -> HashSet<usize> {
    let mut seen: HashSet<usize> = HashSet::new();

    let mut queue: Vec<usize> = vec![start];
    while let Some(id) = queue.pop() {
        // Don't process boxes twice
        if seen.contains(&id) {
            continue;
        }
        seen.insert(id);

        for next in [boxes[id] + dir, boxes[id] + dir + East.into()] {
            // If anything is pushing against a wall, we are immediately done. Full stop.
            if map[next] == '#' {
                return HashSet::new();
            }

            // But if it's a box, we got things to do!
            if let Some(next_id) = has_a_box(next, boxes) {
                // println!("  + id={id} pushes against next_id={next_id}");
                queue.push(next_id);
            }
        }
    }

    seen
}

fn print_board(robot: Option<IVec2>, map: &Framebuffer<char>, boxes: &[IVec2]) {
    if cfg!(test) {
        assert!(
            boxes.len() < (10 + 26),
            "print_board() uses digits + lower ascii to name boxes, but you have too many! {}",
            boxes.len()
        );
        map.print(|x, y, &c| {
            let here = IVec2::new(x, y);
            if Some(here) == robot {
                '@'
            } else if let Some(id) = boxes
                .iter()
                .position(|&b| here == b || here == (b + East.into()))
            {
                assert_eq!(c, '.');
                b"0123456789abcdefghijklmnopqrstuvwxyz"[id] as char
            } else {
                c
            }
        });
        println!();
    }
}

#[aoc(day15, part2)]
pub fn part2(input: &str) -> i64 {
    let (map, moves) = input.split_once("\n\n").unwrap();

    let mut robot = IVec2::zero();
    let mut boxes: Vec<IVec2> = vec![];
    let map = Framebuffer::parse_grid2(map, |ParsingInfo { c, x, y }| match c {
        '@' => {
            robot = IVec2::new(2 * x, y);
            '.'
        }
        'O' => {
            boxes.push(IVec2::new(2 * x, y));
            '.'
        }
        '#' => '#',
        '.' => '.',
        _ => unreachable!("Unexpected map character {c:?}"),
    });
    let map = Framebuffer::new_with_ranges_and(
        0..(2 * map.width() as i32),
        0..(map.height() as i32),
        |x, y| {
            let x = x / 2;
            map[(x, y)]
        },
    );

    if cfg!(test) {
        println!("Initial State:");
        print_board(Some(robot), &map, &boxes);
    }

    for c in moves.lines().flat_map(str::chars) {
        let dir: IVec2 = c_to_dir(c).into();
        let next = robot + dir;

        if map[next] == '#' {
            if cfg!(test) {
                println!("  + BONK! Wall.");
                println!();
            }
        } else if let Some(id) = has_a_box(next, &boxes) {
            let to_move = try_to_move_boxes(id, dir, &boxes, &map);
            if !to_move.is_empty() {
                if cfg!(test) {
                    println!("  + PUSH! box id={id} pushes {} boxes", to_move.len());
                }

                for id in to_move {
                    boxes[id] += dir;
                }
                robot = next;
            } else {
                if cfg!(test) {
                    println!("  + BONK! Tried to move some boxes, but pushing against a wall",);
                    println!();
                }
            }
        } else if map[next] == '.' {
            // (Note: map doesn't know where boxes are, so we this ordering is important!)
            // Robot isn't pushing against anything, it can move freely
            robot = next;
        }

        if cfg!(test) {
            println!("Moving {c}:");
            print_board(Some(robot), &map, &boxes);
            sanity_check_boxes(&boxes);
        }
    }

    // Compute GPS scores
    let maxy = map.height() as i32;
    boxes
        .into_iter()
        .map(|pos| {
            let [x, y] = pos.as_array();
            // Our y axis is inverted from AOC's
            let y = maxy - y - 1;
            (x + 100 * y) as i64
        })
        .sum()
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT_SMALL: &str = r"
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";

    const EXAMPLE_INPUT_BIG: &str = r"
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

    #[rstest]
    #[case::given(2028, EXAMPLE_INPUT_SMALL)]
    #[case::given(10092, EXAMPLE_INPUT_BIG)]
    #[trace]
    #[timeout(Duration::from_millis(100))]
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

    /*
         1 3 5 7 9 11
        ##############
        ##...[].##..## GPS = 5,1
        ##...@.[]...## GPS = 7,2
        ##....[]....## GPS = 6,3
        0 2 4 6 8 10 12
    */
    const EXAMPLE_INPUT_P2_SMALL: &str = r"
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^
";

    const NOTHING_HAPPENS: &str = r"
######
#O..##
#..#@#
#...##
#....#
######

<vv<<^^<<^^
";

    #[rstest]
    #[case::given_small((5 + 100*1) + (7 + 100*2) + (6 + 100*3), EXAMPLE_INPUT_P2_SMALL)]
    #[case::nothing(2 + 100*1, NOTHING_HAPPENS)]
    #[case::given_big(9021, EXAMPLE_INPUT_BIG)]
    #[trace]
    #[timeout(Duration::from_millis(200))]
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
