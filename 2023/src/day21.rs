#![allow(unused)]

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

    let mut ps: HashSet<MetaPoint> = [start].into();
    let mut seen_this_step: HashSet<MetaPoint> = HashSet::new();

    let spacing = false;

    for step in 1..=steps {
        for p in ps.drain() {
            for dir in Cardinal::ALL_NO_DIAG {
                let next = (p + dir).refit(nx, ny);

                if grid[next.in_tile] == '#' {
                    continue;
                }

                if !seen_this_step.contains(&next) {
                    seen_this_step.insert(next);
                }
            }
        }

        for p in seen_this_step.drain() {
            ps.insert(p);
        }

        if log_enabled!(Info) {
            {
                info!(
                    "[step {step:2>}] plots={}, tiles={}",
                    ps.len(),
                    ps.iter().unique_by(|MetaPoint { tile, .. }| tile).count()
                );
            }

            if [1, steps].contains(&step) {
                // Render a 3x3 grid of the maps
                for my in -1..2 {
                    for y in grid.range_y().rev() {
                        for mx in -1..2 {
                            let tile = IVec2::new(mx, my);
                            for x in grid.range_x() {
                                if ps.contains(&MetaPoint {
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

    ps.len() as i64
}

fn do_part2(steps: i64, input: &str) -> i64 {
    use num::traits::Pow;

    if steps < 20 {
        do_part2_sim(steps, input)
    } else {
        let x0 = 10;
        let x1 = 100;
        let x0log = (x0 as f64).ln();
        let x1log = (x1 as f64).ln();

        let y0 = do_part2_sim(x0, input) as f64;
        let y1 = do_part2_sim(x1, input) as f64;
        let y0log = y0.ln();
        let y1log = y1.ln();

        let m = (y1log - y0log) / (x1log - x0log);
        let b = y0log - (m * x0log);

        assert_eq!(y0, (f64::exp(b) * (x0 as f64).pow(m)).round());
        assert_eq!(y1, (f64::exp(b) * (x1 as f64).pow(m)).round());

        info!("y_log = {m} * x_log + {b}");
        info!("y = e^{b} * x^({m})");

        let r = f64::exp(b) * (steps as f64 - 1.).pow(m);
        info!("r={r}");

        r as i64
    }
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

    fn ms(ms: u32) -> Duration {
        Duration::from_millis(ms.into())
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
        assert_eq!(do_part2(steps, input), expected);
    }

    #[rstest]
    #[case::given_p1(6, 16, EXAMPLE_INPUT)]
    #[case::given_10(10, 50, EXAMPLE_INPUT)]
    #[case::given_50(50, 1594, EXAMPLE_INPUT)]
    #[case::given_100(100, 6536, EXAMPLE_INPUT)]
    #[timeout(ms(1_000))]
    #[trace]
    fn check_ex_part_2_sim(
        #[notrace]
        #[case]
        steps: i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(do_part2_sim(steps, input), expected);
    }
}
