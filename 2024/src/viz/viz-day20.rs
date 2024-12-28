use clap::Parser;
use image::{imageops, Rgb};

use aoc24::prelude::*;

#[derive(Debug, Parser)]
struct Opts {
    #[arg(short, long, default_value = "input/2024/day20.txt")]
    input: String,

    #[arg(short, long)]
    output: Option<String>,

    #[arg(long)]
    example: bool,

    #[arg(long, default_value = "512")]
    width: u32,

    /// When set, the generated image will have a shortest path colored.
    #[arg(long)]
    solve: bool,
}

impl Opts {
    fn output(&self) -> String {
        if let Some(ref output) = self.output {
            output.clone()
        } else if self.example {
            "day20-example.png".to_string()
        } else {
            "day20.png".to_string()
        }
    }
}

const EXAMPLE_INPUT: &str = r"
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

fn main() {
    let opts = Opts::parse();

    let input: String = if opts.example {
        println!("Using built-in example:");
        println!("{}", opts.example);
        EXAMPLE_INPUT.trim().to_string()
    } else {
        println!("Reading input from {:?}", opts.input);
        std::fs::read_to_string(&opts.input).expect("Failed to open input")
    };

    let mut start = IVec2::zero();
    let mut end = IVec2::zero();
    let mut map = Framebuffer::parse_grid2(&input, |ParsingInfo { c, x, y }| {
        match c {
            'S' => start = IVec2::new(x, y),
            'E' => end = IVec2::new(x, y),
            '.' => {}
            '#' => {}
            _ => unreachable!(),
        }
        c
    });

    if opts.solve {
        println!("Attempting to solve...");

        let mut path = vec![start];
        let mut i = 0;
        'search: loop {
            let curr = path[i];
            i += 1;

            for next in curr.neighbors() {
                if i > 2 && path[i - 2] == next {
                    continue;
                }

                match map[next] {
                    '#' => continue,
                    '.' => {
                        path.push(next);
                        break;
                    }
                    'S' => unreachable!(),
                    'E' => {
                        path.push(next);
                        break 'search;
                    }
                    c => unreachable!("Unrecognized map character {c}"),
                }
            }
        }

        println!("Found path of length {}", path.len());
        for &p in &path {
            if p == start {
                continue;
            }
            if p == end {
                continue;
            }
            map[p] = 'O';
        }
    }

    // Write out the image
    let mut palette = [Rgb([0, 0, 0_u8]); 128];
    palette[b'#' as usize] = AOC_DARK_GRAY;
    palette[b'.' as usize] = AOC_BLUE;
    palette[b'S' as usize] = START_GREEN;
    palette[b'E' as usize] = FINAL_RED;
    palette[b'O' as usize] = AOC_GOLD;

    let scale = opts.width.div_ceil(map.width() as u32);
    let mut img = map.make_image(scale, |&c| palette[c as usize]);
    imageops::flip_vertical_in_place(&mut img);

    println!("Saving to {}", opts.output());
    img.save(opts.output())
        .unwrap_or_else(|e| panic!("Failed to save image to {}: {e:?}", opts.output()));
}
