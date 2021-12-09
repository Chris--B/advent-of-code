use aoc_runner_derive::aoc;

use image::{GenericImage, GenericImageView, ImageBuffer};

pub fn parse_input(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '0' => 0,
                    '1' => 1,
                    '2' => 2,
                    '3' => 3,
                    '4' => 4,
                    '5' => 5,
                    '6' => 6,
                    '7' => 7,
                    '8' => 8,
                    '9' => 9,
                    _ => todo!(),
                })
                .collect()
        })
        .collect()
}

// Part1 ======================================================================
#[aoc(day9, part1)]
#[inline(never)]
pub fn part1(input: &str) -> u64 {
    let input = parse_input(input);

    let width = input[0].len() as isize;
    let height = input.len() as isize;

    let get = |x: isize, y: isize| -> u64 {
        if x >= 0 && y >= 0 {
            if let Some(row) = input.get(y as usize) {
                if let Some(depth) = row.get(x as usize) {
                    return *depth;
                }
            }
        }

        u64::MAX
    };

    let mut risk = 0;

    for y in 0..height {
        for x in 0..width {
            let pts = [get(x, y - 1), get(x, y + 1), get(x - 1, y), get(x + 1, y)];
            let min = pts.into_iter().min().unwrap();
            if min > get(x, y) {
                // found one
                risk += 1 + get(x, y);
            }
        }
    }

    risk
}

struct Grid(Vec<Vec<u64>>);

impl Grid {
    fn dims(&self) -> (isize, isize) {
        let width = self.0[0].len() as isize;
        let height = self.0.len() as isize;

        (width, height)
    }
    fn get(&self, x: isize, y: isize) -> Option<u64> {
        self.0.get(y as usize)?.get(x as usize).copied()
    }

    fn get_mut(&mut self, x: isize, y: isize) -> Option<&mut u64> {
        self.0.get_mut(y as usize)?.get_mut(x as usize)
    }

    fn fill(&mut self, old: u64, new: u64) {
        let (w, h) = self.dims();

        for y in 0..h {
            for x in 0..w {
                let x = self.get_mut(x, y).unwrap();
                if *x == old {
                    *x = new;
                }
            }
        }
    }
}

// Part2 ======================================================================
#[aoc(day9, part2)]
#[inline(never)]
pub fn part2(input: &str) -> usize {
    let mut grid = Grid(parse_input(input));

    let (w, h) = grid.dims();
    dbg!(w, h);

    let mut img = ImageBuffer::from_fn(10 * w as u32, 10 * h as u32, |x, y| {
        let x = x / 10;
        let y = y / 10;
        if grid.get(x as isize, y as isize).unwrap() == 9 {
            image::Rgb([0_u8, 0, 0])
        } else {
            image::Rgb([255, 255, 255])
        }
    });
    img.save("_day9.png").unwrap();

    // manually color image

    let img =
        match image::open("/Users/chris/code/me/advent-of-code/2021/_day9-colored.png").unwrap() {
            image::DynamicImage::ImageRgb8(img) => img,
            _ => panic!(),
        };

    let mut counts = std::collections::HashMap::new();
    for pixel in img.pixels() {
        if *pixel == image::Rgb([0, 0, 0]) || *pixel == image::Rgb([255, 255, 255]) {
            continue;
        }
        *counts.entry(*pixel).or_insert(0) += 1;
    }

    let mut nums: Vec<usize> = counts.iter().map(|(k, v)| *v).collect();
    nums.sort_unstable();
    nums.reverse();

    dbg!(&nums[..3]);

    // for y in 0..h {
    //     for x in 0..w {
    //         print!("{:>3} ", grid.get(x, y).unwrap());
    //     }
    //     println!();
    // }
    // println!();

    nums[..3].iter().copied().product::<usize>() / ((10 * 10) * (10 * 10) * (10 * 10))
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

// #[test]
// fn check_example_2() {
//     const INPUT: &str = "2199943210
// 3987894921
// 9856789892
// 8767896789
// 9899965678";
//     assert_eq!(part2(INPUT), 1134);
// }
