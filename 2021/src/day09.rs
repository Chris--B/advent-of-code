use aoc_runner_derive::aoc;

use image::Rgb;

use std::fs::File;
use std::sync::atomic::{AtomicBool, Ordering::SeqCst};

use crate::framebuffer::Framebuffer;

// Turn off by default, because it dumps a lot
static SAVE_IMG: AtomicBool = AtomicBool::new(false);
const SCALE: u32 = 20;

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
fn find_min_points(fb: &mut Framebuffer<u8>) {
    fb.kernel_3x3(|_x, _y, taps| {
        #[rustfmt::skip]
        let [
            [_, a, _],
            [b, o, c],
            [_, d, _],
        ] = *taps;

        if *o == WALL_HEIGHT {
            return WALL_GRAY;
        }

        let m = o.min(a).min(b).min(c).min(d);
        if m == o {
            // Found a min point, leave it as its depth
            *o
        } else {
            // Not a min point
            SLOPE_GRAY
        }
    });
}

fn random_color(_x: usize, _y: usize) -> Rgb<u8> {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    let r = rng.gen();
    let g = rng.gen();
    let b = rng.gen();

    Rgb([r, g, b])
}

const BLACK: Rgb<u8> = Rgb([0_u8; 3]);
const WHITE: Rgb<u8> = Rgb([0xff_u8; 3]);

// Mataches AOC colors
const GOLD: Rgb<u8> = Rgb([0xff, 0xff, 0x66]);
const BLUE: Rgb<u8> = Rgb([15, 15, 35]);
const GREEN: Rgb<u8> = Rgb([0x0, 0x99, 0x0]);

// Part1 ======================================================================
#[aoc(day9, part1)]
#[inline(never)]
pub fn part1(input: &str) -> usize {
    let mut fb = parse_input(input);
    find_min_points(&mut fb);

    if saving_images() {
        fb.save_to("_day9_mins.png", SCALE, |b| match *b {
            WALL_GRAY => GREEN,
            SLOPE_GRAY => BLUE,
            _ => GOLD,
        })
        .unwrap();
    }

    fb.into_inner()
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
    let mut min_points = parse_input(input);
    find_min_points(&mut min_points);

    // TODO: Be nice to translate pixel types with a kernel, instead of this
    let mut fb = Framebuffer::from_func(min_points.width(), min_points.height(), |x, y| {
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
    fb.set_border_color(Some(BLACK));

    let mut fbs = vec![];

    loop {
        if saving_images() {
            fbs.push(fb.clone());
        }

        let mut points_changed = 0;

        fb.kernel_3x3(|_x, _y, taps| {
            #[rustfmt::skip]
            let [
                [_, a, _],
                [b, o, c],
                [_, d, _],
            ] = *taps;

            // Only consider white points for basins
            if *o != WHITE {
                return *o;
            }

            // NO DIAGONALS.
            let taps = [a, b, c, d];

            // Pick the first color, there should be no conflicts!
            if let Some(color) = taps.iter().find(|p| ***p != BLACK && ***p != WHITE) {
                // We're adjacent to known basin, so expand
                points_changed += 1;
                **color
            } else {
                // We're not, so just return the same color
                *o
            }
        });

        if points_changed == 0 {
            break;
        }
    }

    if saving_images() {
        fbs.push(fb.clone());
    }

    let pixel_counts = fb.counts();
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
        for x in 0..fb.width() {
            'inner: for y in 0..fb.width() {
                // skip walls
                if fb[(x, y)] == BLACK {
                    continue;
                }

                // if it's a top-3 basin, skip it too
                for (color, _count) in top_3 {
                    if fb[(x, y)] == *color {
                        continue 'inner;
                    }
                }

                // otherwise, clear it
                fb[(x, y)] = WHITE;
            }
        }

        fbs.push(fb.clone());
    }

    if saving_images() {
        use image::{
            gif::{GifEncoder, Repeat},
            Delay, Frame,
        };

        let gif_output = File::create("_9-2_steps.gif").unwrap();
        let mut gif_encoder = GifEncoder::new(gif_output);
        gif_encoder.set_repeat(Repeat::Infinite).unwrap();

        let delay = Delay::from_numer_denom_ms(300, 1);

        let last_frame_idx = fbs.len() - 1;
        for (t, fb) in fbs.into_iter().enumerate() {
            let name = format!("_9-2_step-{:02}.png", t);
            println!("Saving step {} to \'{}\'", t, name);

            let img = fb.make_image(SCALE, |p| image::Rgba([p[0], p[1], p[2], 0xff]));
            img.save(&name).unwrap();

            let frame = if t < last_frame_idx {
                Frame::from_parts(img, 0, 0, delay)
            } else {
                Frame::from_parts(img, 0, 0, Delay::from_numer_denom_ms(2000, 1))
            };
            gif_encoder.encode_frame(frame).unwrap();
        }

        println!("Saving _9-2_steps.gif");
    } else {
        drop(fbs);
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
