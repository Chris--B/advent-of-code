#![allow(unused)]

use image::{Rgb, RgbImage};
use indicatif::ProgressBar;

use crate::prelude::*;

use std::ops::Add;

type Grid = Framebuffer<char>;

fn parse(input: &str) -> (IVec2, Grid) {
    let grid = Grid::parse_grid(input, |b| b);
    let start = grid
        .iter_coords()
        .find(|(x, y)| grid[(*x, *y)] == 'S')
        .unwrap()
        .into();

    (start, grid)
}

// Part1 ========================================================================
fn do_part1(steps: i64, input: &str) -> i64 {
    let (start, mut grid) = parse(input);
    grid.set_border_color(Some('#'));

    let mut ps: Vec<IVec2> = vec![start];
    let mut seen_this_step: Vec<IVec2> = vec![];

    for step in 1..=steps {
        for p in ps.drain(..) {
            for dir in Cardinal::ALL_NO_DIAG {
                let next = p + dir.into();

                if grid[next] == '#' {
                    continue;
                }

                if !seen_this_step.contains(&next) {
                    seen_this_step.push(next);
                }
            }
        }

        ps.append(&mut seen_this_step);

        if cfg!(test) {
            println!("=== step={step}, plots={}", ps.len());
            grid.print(|x, y, c| if ps.contains(&(x, y).into()) { 'O' } else { *c });
            println!();
        } else {
            info!("[step {step:2>}] plots={}", ps.len());
        }
    }

    ps.len() as i64
}

#[aoc(day21, part1)]
pub fn part1(input: &str) -> i64 {
    do_part1(64, input)
}

// Part2 ========================================================================
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct MetaPoint {
    in_tile: IVec2,
    tile: IVec2,
}

impl MetaPoint {
    fn full_coord(mut self, nx: i32, ny: i32) -> IVec2 {
        self.in_tile + self.tile * IVec2::new(nx, ny)
    }

    fn refit(mut self, nx: i32, ny: i32) -> Self {
        // Adjust x if it's outside of its tile
        let x = self.in_tile.x;
        if !(0..nx).contains(&x) {
            self.in_tile.x = wrap(x, nx);

            // and adjust the tile
            if x >= nx {
                self.tile.x += 1;
            } else if x < 0 {
                self.tile.x -= 1;
            } else {
                unreachable!();
            }
        };

        // Adjust y if it's outside of its tile
        let y = self.in_tile.y;
        if !(0..ny).contains(&y) {
            self.in_tile.y = wrap(y, ny);

            // and adjust the tile
            if y >= ny {
                self.tile.y += 1;
            } else if y < 0 {
                self.tile.y -= 1;
            } else {
                unreachable!();
            }
        };

        self
    }
}

impl Add<Cardinal> for MetaPoint {
    type Output = Self;
    fn add(mut self, rhs: Cardinal) -> Self {
        self.in_tile += rhs.into();
        self
    }
}

fn wrap(a: i32, n: i32) -> i32 {
    // Keep the indices positive
    ((a % n) + n) % n
}

fn do_part2_sim(steps: i64, input: &str) -> i64 {
    let (start, grid) = parse(input);
    let nx = grid.width() as i32;
    let ny = grid.height() as i32;

    let start = MetaPoint {
        in_tile: start,
        tile: IVec2::zero(),
    };

    let mut seen: HashSet<MetaPoint> = HashSet::new();
    let mut frontier = vec![start];
    let mut new_frontier = vec![];

    let spacing = false;

    const IMAGE_DIM: u32 = if cfg!(test) { 512 } else { 4096 };
    let mut frame = RgbImage::new(IMAGE_DIM, IMAGE_DIM);
    let pb = ProgressBar::new(steps as u64);

    for step in (1..=steps) {
        for p in frontier.drain(..) {
            for dir in Cardinal::ALL_NO_DIAG {
                let next: MetaPoint = (p + dir);
                let next = next.refit(nx, ny);

                if grid[next.in_tile] == '#' {
                    continue;
                }

                if !seen.contains(&next) {
                    seen.insert(next);
                    new_frontier.push(next);
                }
            }
        }

        std::mem::swap(&mut frontier, &mut new_frontier);
        pb.inc(1);

        // Debug logging
        if log_enabled!(Info) {
            const S: i64 = if cfg!(test) { 5 } else { 500 };
            if (step == steps || step % S == 0) {
                let max_x = seen.iter().map(|p| p.full_coord(nx, ny).x).max().unwrap();
                let max_y = seen.iter().map(|p| p.full_coord(nx, ny).y).max().unwrap();
                info!(
                    "[step {step:5>}] tiles={}, frontier={}, seen={}, max_x={max_x}, max_y={max_y}",
                    seen.iter().unique_by(|MetaPoint { tile, .. }| tile).count(),
                    frontier.len(),
                    seen.len(),
                );
            }

            // Save PNGs
            {
                let dir = format!(
                    "/Users/chris/code/me/advent-of-code/2023/target/day21{test}/",
                    test = if cfg!(test) { "_test" } else { "" }
                );
                let filename = format!("{dir}/f{step:>05}.png");
                std::fs::create_dir_all(dir).unwrap();

                for p in &frontier {
                    let (x, y) = p.full_coord(nx, ny).into();
                    let x = x + (IMAGE_DIM / 2) as i32;
                    let y = y + (IMAGE_DIM / 2) as i32;

                    if let Some(px) = frame.get_pixel_mut_checked(x as u32, y as u32) {
                        *px = Rgb([0xff, 0x00, 0xff]);
                    }
                }

                frame.save(filename).unwrap();
            }

            if cfg!(test) && step == steps {
                for my in -1..2 {
                    for y in grid.range_y().rev() {
                        for mx in -1..2 {
                            let tile = IVec2::new(mx, my);
                            for x in grid.range_x() {
                                if seen.contains(&MetaPoint {
                                    in_tile: IVec2::new(x, y),
                                    tile,
                                }) {
                                    print!("O");
                                } else {
                                    print!("{}", grid[(x, y)]);
                                }
                            }
                            if spacing {
                                print!(" ");
                            }
                        }
                        println!();
                    }
                    if spacing {
                        println!();
                    }
                }
                println!();
            }
        }
    }

    todo!()
}

fn do_part2(steps: i64, input: &str) -> i64 {
    do_part2_sim(steps, input)
}

#[aoc(day21, part2)]
pub fn part2(input: &str) -> i64 {
    let r = do_part2(26_501_365, input);

    assert!(r > 91510463199137);
    assert!(r > 255114563307125);

    r
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;
    use std::time::Duration;

    const EXAMPLE_INPUT: &str = r"
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
";

    #[rstest]
    #[case::given(6, 16, EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_1(
        #[notrace]
        #[case]
        steps: i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(do_part1(steps, input), expected);
    }

    #[rstest]
    #[case::given_10(10, 50, EXAMPLE_INPUT)]
    #[case::given_50(50, 1594, EXAMPLE_INPUT)]
    #[case::given_100(100, 6536, EXAMPLE_INPUT)]
    #[case::given_500(500, 167004, EXAMPLE_INPUT)]
    #[case::given_1000(1000, 668697, EXAMPLE_INPUT)]
    #[case::given_5000(5000, 16733044, EXAMPLE_INPUT)]
    #[timeout(ms(1_000))]
    #[trace]
    fn check_ex_part_2(
        #[notrace]
        #[case]
        steps: i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        let input = input.trim();
        let res = do_part2(steps, input);
        assert_eq!(res, expected, "Off by {}", res - expected);
    }
}
