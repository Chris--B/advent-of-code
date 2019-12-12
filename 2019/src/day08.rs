use aoc_runner_derive::{aoc, aoc_generator};

use std::fmt::Write;

#[aoc_generator(day8)]
pub fn parse_input(input: &str) -> Vec<u32> {
    input
        .trim()
        .chars()
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
            _ => panic!("Unexpected digit: '{}'", c),
        })
        .collect()
}

fn count_digits(layer: &[u32]) -> [u32; 10] {
    let mut table = [0u32; 10];

    for pixel in layer {
        let pixel = *pixel as usize;
        assert!(pixel < 10);
        table[pixel] += 1;
    }

    table
}

#[aoc(day8, part1)]
pub fn part1(input: &[u32]) -> u32 {
    const IMAGE_WIDTH: usize = 25;
    const IMAGE_HEIGHT: usize = 6;

    let counts = input
        .chunks_exact(IMAGE_HEIGHT * IMAGE_WIDTH)
        .map(|layer| count_digits(layer))
        .min_by_key(|counts| counts[0])
        .unwrap();

    dbg!(counts[1]) * dbg!(counts[2])
}

#[aoc(day8, part2)]
pub fn part2(input: &[u32]) -> Result<String, std::fmt::Error> {
    const IMAGE_WIDTH: usize = 25;
    const IMAGE_HEIGHT: usize = 6;
    const IMAGE_SIZE: usize = IMAGE_HEIGHT * IMAGE_WIDTH;

    let layers: Vec<&[u32]> = input.chunks(IMAGE_SIZE).collect();

    const COLOR_BLACK: u32 = 0;
    const COLOR_WHITE: u32 = 1;
    const COLOR_TRANSPARENT: u32 = 2;

    let mut image: Vec<u32> = vec![0; IMAGE_SIZE];
    for (i, pixel) in image.iter_mut().enumerate() {
        for layer in &layers {
            let layer_pixel = layer[i];
            if layer_pixel != COLOR_TRANSPARENT {
                *pixel = layer_pixel;
                break;
            }
        }
    }

    let mut output = String::new();
    writeln!(output)?;

    for row in image.chunks(IMAGE_WIDTH) {
        for p in row {
            // See: https://en.wikipedia.org/wiki/Block_Elements
            let display = match *p {
                COLOR_WHITE => "\u{2591}", // Light shade
                COLOR_BLACK => "\u{2588}", // Full block
                COLOR_TRANSPARENT => " ",
                _ => panic!("Invalid pixel: {}", p),
            };
            write!(output, "{}", display)?;
        }
        writeln!(output,)?;
    }

    Ok(output)
}
