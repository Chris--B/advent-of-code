use aoc_runner_derive::{aoc, aoc_generator};

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

    let (layer, counts) = input
        .chunks_exact(IMAGE_HEIGHT * IMAGE_WIDTH)
        .map(|layer| (layer, count_digits(layer)))
        .min_by_key(|(_layer, counts)| counts[0])
        .unwrap();

    dbg!(counts[1]) * dbg!(counts[2])
}
