use aoc_runner_derive::aoc;

use image::{Luma, Rgb};

use std::sync::atomic::{AtomicBool, Ordering::SeqCst};

use crate::framebuffer::Framebuffer;

// Turn off by default, because it dumps a lot
static SAVE_IMG: AtomicBool = AtomicBool::new(true);

fn saving_images() -> bool {
    SAVE_IMG.load(SeqCst)
}

/// Wall values separate basins and we'll never mess with them.
const WALL_HEIGHT: u8 = 9;

// Grayscale value markers
const WALL_GRAY: u8 = u8::MAX;
const SLOPE_GRAY: u8 = u8::MAX / 2;

fn parse_input(input: &str) -> Framebuffer<u8> {
    let mut pixels = vec![];
    let mut width = 0;

    for (i, b) in input.as_bytes().iter().enumerate() {
        if *b == b'\n' {
            if width == 0 {
                width = i;
            }

            continue;
        }

        let p = b - b'0';
        assert!(p <= 9);
        pixels.push(p);
    }

    assert_eq!(pixels.len() % width, 0);
    let height = pixels.len() / width;

    let mut fb = Framebuffer::from_func(width, height, |x, y| {
        let i = x + y * width;
        pixels[i]
    });

    // Pretend everything out of bounds is just more wall
    fb.set_border_color(Some(WALL_HEIGHT));

    fb
}

/// Produce a framebuffer where each point is one of:
///     - a wall, 9
///     - a min point in a basin, its depth [0, 8]
///     - not a min point, 10
fn make_min_points(fb: &Framebuffer<u8>) -> Framebuffer<u8> {
    let mut min_points = Framebuffer::with_dims(fb.width(), fb.height());
    fb.kernel_3x3(|x, y, taps| {
        #[rustfmt::skip]
        let [
            [_, a, _],
            [b, o, c],
            [_, d, _],
        ] = *taps;

        if *o == WALL_HEIGHT {
            min_points[(x, y)] = WALL_GRAY;
            return;
        }

        let m = o.min(a).min(b).min(c).min(d);
        min_points[(x, y)] = if m == o {
            // Found a min point, leave it as its depth
            *o
        } else {
            // Not a min point
            SLOPE_GRAY
        };
    });

    min_points
}

fn random_color(x: usize, y: usize) -> Rgb<u8> {
    let i = x + 100 * y;
    let (r, g, b) = (128, i / 256, i % 256);

    Rgb([r, g as u8, b as u8])
}

// Part1 ======================================================================
#[aoc(day9, part1)]
#[inline(never)]
pub fn part1(input: &str) -> usize {
    let fb = parse_input(input);
    let min_points = make_min_points(&fb);

    min_points
        .save_to("_day9_mins.png", |b| Luma([*b]))
        .unwrap();

    min_points
        .into_inner()
        .into_iter()
        // Only count things that still look like heights
        .filter(|p| *p < WALL_HEIGHT)
        .map(|p| 1 + p as usize)
        .sum()
}

// Part2 ======================================================================
#[aoc(day9, part2)]
#[inline(never)]
pub fn part2(input: &str) -> usize {
    let fb = parse_input(input);
    let min_points = make_min_points(&fb);

    // Walls are black
    const BLACK: Rgb<u8> = Rgb([0_u8; 3]);
    const WHITE: Rgb<u8> = Rgb([0xff_u8; 3]);

    let mut fb_a = Framebuffer::from_func(fb.width(), fb.height(), |x, y| {
        // Translate the pseduo-grayscale colors to Rgb
        match min_points[(x, y)] {
            WALL_GRAY => BLACK,
            SLOPE_GRAY => WHITE,
            o => {
                assert!(o < WALL_HEIGHT);
                // Finally a point! Give it a unique color
                random_color(x, y)
            }
        }
    });
    let mut fb_b = fb_a.clone();

    fb_a.set_border_color(Some(BLACK));
    fb_b.set_border_color(Some(BLACK));

    let mut prev = &mut fb_a;
    let mut next = &mut fb_b;

    let mut imgs = vec![];

    loop {
        if saving_images() {
            imgs.push(prev.clone());
        }

        let mut points_changed = 0;

        prev.kernel_3x3(|x, y, taps| {
            #[rustfmt::skip]
            let [
                [_, a, _],
                [b, o, c],
                [_, d, _],
            ] = *taps;

            // Only consider white points for basins
            if *o != WHITE {
                next[(x, y)] = *o;
                return;
            }

            // NO DIAGONALS.
            let taps = [a, b, c, d];

            // Pick the first color, there should be no conflicts!
            if let Some(color) = taps.iter().find(|p| ***p != BLACK && ***p != WHITE) {
                // We're adjacent to known basin, so expand
                points_changed += 1;
                next[(x, y)] = **color;
            } else {
                // We're not, so do nothing yet
            }
        });

        if points_changed == 0 {
            break;
        }

        std::mem::swap(&mut prev, &mut next);
    }

    if saving_images() {
        imgs.push(prev.clone());
    }

    let pixel_counts = next.counts();
    let mut sorted: Vec<(Rgb<_>, usize)> = pixel_counts
        .into_iter()
        .map(|(color, count)| (*color, count))
        .filter(|(color, _count)| *color != BLACK && *color != WHITE)
        .collect();
    // sort backwards, we want largest first
    sorted.sort_by_key(|(_color, count)| usize::MAX - *count);

    let top_3 = &sorted[..3];
    let res = top_3.iter().map(|(_color, count)| *count).product();

    // Pretty pictures
    if saving_images() {
        for x in 0..prev.width() {
            'inner: for y in 0..prev.width() {
                // skip walls
                if prev[(x, y)] == BLACK {
                    continue;
                }

                // if it's a top-3 basin, skip it too
                for (color, _count) in top_3 {
                    if prev[(x, y)] == *color {
                        continue 'inner;
                    }
                }

                // otherwise, clear it
                prev[(x, y)] = WHITE;
            }
        }

        imgs.push(prev.clone());
    }

    if saving_images() {
        for (t, frame) in imgs.into_iter().enumerate() {
            let name = format!("_9-2_step-{:02}.png", t);
            println!("Saving step {} to \'{}\'", t, name);
            frame.save_to(&name, |p| *p).unwrap();
        }
    } else {
        drop(imgs);
    }

    res
}

#[test]
fn check_example_1() {
    const INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";
    assert_eq!(part1(INPUT), 15);
}

#[test]
fn check_example_2() {
    const INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";
    assert_eq!(part2(INPUT), 1134);
}
