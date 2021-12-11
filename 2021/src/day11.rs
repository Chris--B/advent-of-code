use aoc_runner_derive::aoc;

fn flatten_mut<T>(a: &mut [[T; 10]; 10]) -> &mut [T; 100] {
    unsafe { std::mem::transmute(a) }
}

fn parse_input(input: &str) -> [[u8; 10]; 10] {
    let bytes = input.as_bytes();

    let mut octs = [0_u8; 100];
    for (i, oct) in bytes
        .iter()
        .filter_map(|b| if *b == b'\n' { None } else { Some(*b - b'0') })
        .enumerate()
    {
        octs[i] = oct;
    }

    unsafe { std::mem::transmute(octs) }
}

fn _print(step: usize, octs: &[u8; 100]) {
    println!("After step {}:", step);

    for (i, o) in octs.iter().enumerate() {
        if i > 0 && i % 10 == 0 {
            println!();
        }

        if *o == 0 {
            print!(".");
        } else {
            print!("{:x}", o);
        }
    }

    println!();
    println!();
}

fn sim_step(octs: &mut [[u8; 10]; 10]) -> usize {
    // First, the energy level of each octopus increases by one
    for o in flatten_mut(octs) {
        *o += 1;
    }

    let mut flashed = [[false; 10]; 10];
    let mut num_flashes = 0;

    loop {
        let mut substep_flashes = 0;

        // Then, any octopus with an energy level greater than 9 flashes
        for y in 0..10 {
            for x in 0..10 {
                // Not active enough
                if octs[y][x] < 10 {
                    continue;
                }

                // Already went
                if flashed[y][x] {
                    continue;
                }

                // We're flashing, so flash all 8 neighbors
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }

                        let xx = (x as isize + dx) as usize;
                        let yy = (y as isize + dy) as usize;

                        if xx < 10 && yy < 10 {
                            octs[yy][xx] += 1;
                        }
                    }
                }

                // This octopus has now flashed, mark it as such
                flashed[y][x] = true;
                octs[y][x] = 0;
                substep_flashes += 1;
            }
        }

        num_flashes += substep_flashes;

        if substep_flashes == 0 {
            break;
        }
    }

    // Finally, any octopus that flashed during this step has its energy level set to 0
    for (i, o) in flatten_mut(octs).iter_mut().enumerate() {
        if flatten_mut(&mut flashed)[i] {
            *o = 0;
        }
    }

    num_flashes
}

// Part1 ======================================================================
#[aoc(day11, part1)]
#[inline(never)]
pub fn part1(input: &str) -> usize {
    let mut octs = parse_input(input);

    let mut flashes = 0;

    for _step in 0..100 {
        flashes += sim_step(&mut octs);
    }

    flashes
}

// Part2 ======================================================================
#[aoc(day11, part2)]
#[inline(never)]
pub fn part2(input: &str) -> i64 {
    let mut octs = parse_input(input);

    for step in 1.. {
        if sim_step(&mut octs) == 100 {
            return step;
        }
    }

    unreachable!()
}

#[test]
fn check_example_1() {
    let input = r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"#;
    assert_eq!(part1(input), 1656);
}

#[test]
fn check_example_2() {
    let input = r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"#;
    assert_eq!(part2(input), 195);
}
