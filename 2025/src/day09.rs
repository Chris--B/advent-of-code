#![allow(unused)]

use indicatif::ProgressBar;
use rayon::prelude::*;
use ultraviolet::Vec2;

use crate::prelude::*;

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

fn intersects(a: [IVec2; 2], b: [IVec2; 2]) -> Option<IVec2> {
    debug_assert_ne!(a, b, "Don't check if a line intersects itself");

    None
}

fn ordered(a: i32, b: i32) -> [i32; 2] {
    let mut xs = [a, b];
    xs.sort();
    xs
}

// Part2 ========================================================================
fn is_inside(pt: IVec2, verts: &[IVec2], edges: &[(IVec2, IVec2)]) -> bool {
    debug_assert!(
        verts.iter().all(|v| v.x > 0 && v.y > 0),
        "Vertices must be positive"
    );

    fn is_on_line(pt: IVec2, (a, b): (IVec2, IVec2)) -> bool {
        if a.x == b.x && a.x == pt.x {
            // Vertical Edge, check that y is in bounds
            let [y0, y1] = ordered(a.y, b.y);
            y0 <= pt.y && pt.y <= y1
        } else if a.y == b.y && a.y == pt.y {
            // Horiztonal Edge, check that x is in bounds
            let [x0, x1] = ordered(a.x, b.x);
            x0 <= pt.x && pt.x <= x1
        } else {
            false
        }
    }

    // Walk intersections until we find our point
    let mut inside = false;
    for &edge in edges {
        if is_on_line(pt, edge) {
            return true;
        }
    }

    // We're not ON an edge, but we might be inside of the shape still.
    // Ray trace in from the side and count the edges we hit?

    let mut here = IVec2::new(0, pt.y);
    'search: while here.x < pt.x {
        for &edge in edges {
            if is_on_line(here, edge) {
                // println!("[pt={pt:?}] Hitting edge={edge:?}");
                // println!("  + was x={}", here.x);
                here.x = i32::max(edge.0.x, edge.1.x) + 1;
                // println!("  + now x={}", here.x);

                // TODO: Handle saddles
                if edge.0.x == edge.1.x {
                    inside = !inside;
                }
                continue 'search;
            }
        }
        // println!();

        // Find the shortest distance we could possibly move without crossing an edge
        let mut dist = i32::MAX;
        for &v in verts {
            dist = dist.min((here - v).abs().component_max() / 10).max(1);
        }
        here.x += dist;
    }

    inside
}

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

    if cfg!(test) {
        let nx = verts.iter().map(|v| v.x).max().unwrap() as u32;
        let ny = verts.iter().map(|v| v.y).max().unwrap() as u32;
        let mut grid = Framebuffer::new(nx + 2, ny + 2);
        grid.clear('.');

        for (a, b) in &edges {
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

    println!("Have  {} verts", verts.len());
    println!("Found {} edges", edges.len());
    let min_x = verts.iter().map(|&v| v.x).min().unwrap();
    let min_y = verts.iter().map(|&v| v.y).min().unwrap();
    println!("Min axes: x={min_x}, y={min_y}");

    assert!(!is_inside([0, 0].into(), &verts, &edges));

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(64)
        .build()
        .unwrap();

    let mut jobs = vec![];
    for i in 0..n {
        for j in (i + 1)..n {
            jobs.push((i, j));
        }
    }
    let pb = ProgressBar::new(jobs.len() as _);

    let area: i64 = jobs
        .into_par_iter()
        .map(|(i, j)| -> i64 {
            let mut a = if cfg!(test) { 10 } else { 1_000_000_000 };

            pb.inc(1);
            let [x0, y0] = verts[i].as_array();
            let [x1, y1] = verts[j].as_array();

            if x0 == x1 || y0 == y1 {
                return 0;
            }

            let dx = (x1 as i64) - (x0 as i64);
            let dy = (y1 as i64) - (y0 as i64);
            let this_area = (1 + dx.abs()) * (1 + dy.abs());
            if this_area < a {
                // Not possible to be the best, ignore it.
                return 0;
            }

            if is_inside(IVec2::new(x0, y0), &verts, &edges)
                && is_inside(IVec2::new(x0, y1), &verts, &edges)
                && is_inside(IVec2::new(x1, y0), &verts, &edges)
                && is_inside(IVec2::new(x1, y1), &verts, &edges)
            {
                let [xx0, xx1] = ordered(x0, x1);
                let [yy0, yy1] = ordered(y0, y1);

                for x in xx0..=xx1 {
                    if !is_inside(IVec2::new(x, y0), &verts, &edges) {
                        return 0;
                    }
                    if !is_inside(IVec2::new(x, y1), &verts, &edges) {
                        return 0;
                    }
                }

                for y in yy0..=yy1 {
                    if !is_inside(IVec2::new(x0, y), &verts, &edges) {
                        return 0;
                    }
                    if !is_inside(IVec2::new(x1, y), &verts, &edges) {
                        return 0;
                    }
                }

                if cfg!(test) {
                    println!(
                        "Found rect: [{x0:>2},{y0:>2}]x[{x1:>2},{y1:>2}] == {this_area:>3} {}x{}",
                        (1 + (y1 - y0).abs()),
                        (1 + (x1 - x0).abs())
                    );
                }
                a = a.max(this_area);
            }

            a
        })
        .max()
        .unwrap();

    pb.finish();

    if !cfg!(test) {
        assert!(area > 163561216, "area > 163561216; area = {area}");
        assert!(area < 4000000000, "area < 4000000000; area = {area}");
    }

    area
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
    #[case::some_edge_1_3([1,3], false)]
    #[case::some_edge_2_3([2,3], true)]
    #[case::some_edge_3_3([3,3], true)]
    #[case::some_edge_7_3([7,3], true)]
    #[case::inside_9_3([9,3], true)]
    #[timeout(Duration::from_millis(100))]
    fn check_is_inside(#[case] pt: [i32; 2], #[case] inside: bool) {
        let input = EXAMPLE_INPUT;
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

        println!("Have  {} verts", verts.len());
        println!("Found {} edges", edges.len());

        assert_eq!(is_inside(pt.into(), &verts, &edges), inside);
    }

    #[rstest]
    #[case::given(50, EXAMPLE_INPUT)]
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

    #[rstest]
    #[case::given(24, EXAMPLE_INPUT)]
    #[case::given_hump(24, EXAMPLE_INPUT_BUT_HUMP)]
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
