#![allow(unused)]

use crate::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Brick(IVec3, IVec3);

impl Brick {
    fn max_z(&self) -> i32 {
        self.1.z
    }

    fn min_z(&self) -> i32 {
        self.0.z
    }

    fn cubes(&self) -> CubeIter {
        let mut count = 0;
        let delta = match (self.1 - self.0).as_array() {
            [0, 0, c] => {
                count = 1 + c as u32;
                IVec3::new(0, 0, 1)
            }
            [0, c, 0] => {
                count = 1 + c as u32;
                IVec3::new(0, 1, 0)
            }
            [c, 0, 0] => {
                count = 1 + c as u32;
                IVec3::new(1, 0, 0)
            }
            _ => unreachable!(),
        };

        CubeIter {
            next: self.0,
            delta,
            count,
        }
    }
}

#[derive(Clone, Debug)]

struct CubeIter {
    next: IVec3,
    delta: IVec3,
    count: u32,
}

impl Iterator for CubeIter {
    type Item = IVec3;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count != 0 {
            let next = self.next;

            self.count -= 1;
            self.next += self.delta;

            Some(next)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.count as usize, Some(self.count as usize))
    }
}

impl ExactSizeIterator for CubeIter {}

fn parse(input: &str) -> (Vec<Brick>, Framebuffer<Vec<usize>>) {
    let bricks = input
        .lines()
        .map(|l| {
            let (a, b) = l.split_once('~').unwrap();
            let a: [i32; 3] = parse_list(a, ",");
            let b: [i32; 3] = parse_list(b, ",");

            Brick(a.into(), b.into())
        })
        .collect_vec();

    let max_x = bricks
        .iter()
        .map(|b| b.0.x.max(b.1.x) as u32)
        .max()
        .unwrap();
    let max_y = bricks
        .iter()
        .map(|b| b.0.y.max(b.1.y) as u32)
        .max()
        .unwrap();

    let mut grid: Framebuffer<Vec<usize>> = Framebuffer::new(max_x + 1, max_y + 1);
    for (bid, brick) in bricks.iter().enumerate() {
        for cube in brick.cubes() {
            grid[cube.xy()].push(bid);
            // TODO: binary search insert?
            grid[cube.xy()].sort_by_key(|id| bricks[*id].0.z);
        }
    }

    (bricks, grid)
}

fn simulate_falling(grid: &mut Framebuffer<Vec<usize>>, bricks: &mut [Brick]) {
    let mut i = 0;
    loop {
        i += 1;
        let mut updated = 0;

        for brick_id in 0..bricks.len() {
            let mut cant_fall = 0;

            'cubes: for cube in bricks[brick_id].cubes() {
                if cube.z == 0 {
                    cant_fall += 1;
                    break 'cubes;
                }

                for other_id in &grid[cube.xy()] {
                    if *other_id == brick_id {
                        break;
                    }

                    if bricks[*other_id].max_z() == cube.z - 1 {
                        cant_fall += 1;
                        break 'cubes;
                    }
                }
            }

            // If nothing can't fall then everything can fall and it should!
            if cant_fall == 0 {
                bricks[brick_id].0.z -= 1;
                bricks[brick_id].1.z -= 1;
                updated += 1;
            }
        }

        if updated == 0 {
            break;
        }
    }

    info!("Took {i} iters to stablize");
}

// Part1 ========================================================================
#[aoc(day22, part1)]
pub fn part1(input: &str) -> i64 {
    let (mut bricks, mut grid) = parse(input);
    info!("Found {} bricks", bricks.len());

    simulate_falling(&mut grid, &mut bricks);

    // Now find which bricks are holding something up
    let mut bricks_supported_by = vec![vec![]; bricks.len()];
    let mut bricks_supporting = vec![vec![]; bricks.len()];

    for brick_id in 0..bricks.len() {
        'cubes: for cube in bricks[brick_id].cubes() {
            for other_id in &grid[cube.xy()] {
                if brick_id == *other_id {
                    break;
                }

                if bricks[*other_id].max_z() == cube.z - 1 {
                    if !bricks_supporting[brick_id].contains(other_id) {
                        bricks_supporting[brick_id].push(*other_id);
                    }
                    break;
                }
            }
        }

        for bid in &bricks_supporting[brick_id] {
            bricks_supported_by[*bid].push(brick_id);
        }
    }

    let mut can_dis_count = 0;

    #[allow(clippy::needless_range_loop)]
    'bricks: for brick_id in 0..bricks.len() {
        // info!("{brick_id}");

        for other_id in &bricks_supported_by[brick_id] {
            if brick_id == *other_id {
                continue;
            }

            // info!("    {other_id} -> {:?}", bricks_supporting[*other_id]);
            if bricks_supporting[*other_id].len() <= 1 {
                // brick_id is integral and cannot be disintegrated.
                continue 'bricks;
            }
        }

        can_dis_count += 1;
    }

    can_dis_count
}

// Part2 ========================================================================
#[aoc(day22, part2)]
pub fn part2(input: &str) -> i64 {
    let (mut bricks, mut grid) = parse(input);
    info!("Found {} bricks", bricks.len());

    simulate_falling(&mut grid, &mut bricks);

    let mut count = 0;

    for brick_id in 0..bricks.len() {
        let mut b = bricks.clone();
        let mut g = grid.clone();
        for xy_cell in g.flatten_mut() {
            if let Some(idx) = xy_cell.iter().position(|i| *i == brick_id) {
                xy_cell.remove(idx);
            }
        }

        simulate_falling(&mut g, &mut b);

        for i in 0..bricks.len() {
            if bricks[i] != b[i] {
                count += 1;
            }
        }
    }

    if !cfg!(test) {
        assert!(count > 67473, "count={count}");
        assert!(count < 1447209, "count={count}");
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
";

    #[rstest]
    #[case::given(5, EXAMPLE_INPUT)]
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
    #[case::given(7, EXAMPLE_INPUT)]
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
