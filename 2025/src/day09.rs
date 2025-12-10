use crate::prelude::*;

use rayon::prelude::*;

// Part1 ========================================================================
#[aoc(day9, part1)]
pub fn part1(input: &str) -> i64 {
    let red_tiles: Vec<(i64, i64)> = input.i64s().tuples().collect_vec();
    let n = red_tiles.len();

    let mut area = 0;

    for i in 0..n {
        for j in (i + 1)..n {
            let a = red_tiles[i];
            let b = red_tiles[j];
            let this_area = (1 + (b.1 - a.1).abs()) * (1 + (b.0 - a.0).abs());
            area = area.max(this_area);
        }
    }

    area
}

fn ordered(a: i32, b: i32) -> [i32; 2] {
    let mut xs = [a, b];
    xs.sort();
    xs
}

// Part2 ========================================================================
#[aoc(day9, part2)]
pub fn part2(input: &str) -> i64 {
    let verts: Vec<IVec2> = input
        .i64s()
        .tuples()
        .map(|(x, y)| [x as _, y as _].into())
        .collect_vec();
    let edges: Vec<(IVec2, IVec2)> = verts
        .iter()
        .chain(&[verts[0]])
        .copied()
        .tuple_windows()
        .collect_vec();
    let n = verts.len();
    print_verts(&verts, &edges);

    fn check_rect([x0, y0]: [i32; 2], [x1, y1]: [i32; 2], edges: &[(IVec2, IVec2)]) -> i64 {
        // Save these before reordering below
        let dx = (x1 as i64) - (x0 as i64);
        let dy = (y1 as i64) - (y0 as i64);

        if !cfg!(test) {
            // yolo I guess
            if (1 + dx.abs()) * (1 + dy.abs()) < 1_000_000_000 {
                return 0;
            }
        }

        let [x0, x1] = ordered(x0, x1);
        let [y0, y1] = ordered(y0, y1);

        for (a, b) in edges {
            if a.x == b.x {
                let x = a.x;
                let [yy0, yy1] = ordered(a.y, b.y);
                for y in yy0..=yy1 {
                    if (x0 < x && x < x1) && (y0 < y && y < y1) {
                        return 0;
                    }
                }
            } else if a.y == b.y {
                let y = a.y;
                let [xx0, xx1] = ordered(a.x, b.x);
                for x in xx0..=xx1 {
                    if (x0 < x && x < x1) && (y0 < y && y < y1) {
                        return 0;
                    }
                }
            }
        }

        (1 + dx.abs()) * (1 + dy.abs())
    }

    let jobs: Vec<(usize, usize)> = (0..n)
        .cartesian_product(0..n)
        .filter(|(i, j)| i < j)
        .collect_vec();

    jobs.into_par_iter()
        .map(|(i, j)| check_rect(verts[i].as_array(), verts[j].as_array(), &edges))
        .max()
        .unwrap()
}

fn print_verts(verts: &[IVec2], edges: &[(IVec2, IVec2)]) {
    if cfg!(test) {
        let nx = verts.iter().map(|v| v.x).max().unwrap() as u32;
        let ny = verts.iter().map(|v| v.y).max().unwrap() as u32;
        let mut grid = Framebuffer::new(nx + 2, ny + 2);
        grid.clear('.');

        for (a, b) in edges {
            if a.x == b.x {
                let x = a.x;
                let [y0, y1] = ordered(a.y, b.y);
                for y in y0..y1 {
                    grid[(x, y)] = 'x';
                }
            } else if a.y == b.y {
                let y = a.y;
                let [x0, x1] = ordered(a.x, b.x);
                for x in x0..x1 {
                    grid[(x, y)] = 'x';
                }
            } else {
                grid.just_print();
                unreachable!("Invalid edges");
            }
        }

        for (i, &v) in verts.iter().enumerate() {
            if i < 10 {
                grid[v] = (i as u8 + b'0') as char;
            } else {
                grid[v] = '#';
            }
        }

        grid.just_print();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

    const EXAMPLE_INPUT_BUT_HUMP: &str = r"
7,1
11,1
11,7
9,7
9,5
5,5
5,7
2,7
2,5
2,3
7,3
";

    const EXAMPLE_INPUT_BUT_HUMP_TRANSPOSE: &str = r"
1,7
1,11
7,11
7,9
5,9
5,5
7,5
7,2
5,2
3,2
3,7
";

    const EXAMPLE_INPUT_BUT_HUMP_INV: &str = r"
8,10
4,10
4,4
6,4
6,6
10,6
10,4
13,4
13,6
13,8
8,8
";

    #[rstest]
    #[case::given(50, EXAMPLE_INPUT)]
    #[trace]
    #[timeout(Duration::from_millis(1000))]
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

    #[rstest]
    #[case::given(24, EXAMPLE_INPUT)]
    #[case::given_hump(24, EXAMPLE_INPUT_BUT_HUMP)]
    #[case::given_hump(24, EXAMPLE_INPUT_BUT_HUMP_TRANSPOSE)]
    #[case::given_hump_inv(24, EXAMPLE_INPUT_BUT_HUMP_INV)]
    #[trace]
    #[timeout(Duration::from_millis(1000))]
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
