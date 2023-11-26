use aoc22::day12::{find_path, parse};
use aoc22::framebuffer::Framebuffer;

// use clap::builder::TypedValueParser as _;
use clap::Parser;
use image::Rgb;

#[derive(Parser, Debug)]
// #[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "input/2022/day12.txt")]
    input: String,

    #[arg(short, long)]
    output: Option<String>,

    #[arg(short, long, default_value = "10", value_name = "pixel scale factor")]
    scale: u32,
}

fn norm<N: Into<i64>>(x: N, max: N, o: u8) -> u8 {
    let x = x.into() as f32;
    let max = max.into() as f32;
    let p = (u8::MAX - o) as f32;

    (p * x / max) as u8 + o
}

fn main() {
    let args = Args::parse();

    let input = std::fs::read_to_string(&args.input).unwrap();

    let day = parse(&input);
    let total_steps_map = find_path(&day, day.start);
    let max_steps = total_steps_map
        .iter_coords()
        .map(|pt| total_steps_map[pt])
        .filter(|s| *s != i64::MAX)
        .max()
        .unwrap();

    let colored: Framebuffer<Rgb<u8>> = Framebuffer::new_with_ranges_and(
        day.heightmap.range_x(),
        day.heightmap.range_y(),
        |x, y| {
            if total_steps_map[(x, y)] == i64::MAX {
                // Unreachable
                return Rgb([0, 0, 0]);
            }

            let h = norm(day.heightmap[(x, y)], 25, 96);
            let s = norm(total_steps_map[(x, y)], max_steps, 0);

            let a = h / 2 + 32;
            let b = s / 2 + 16;
            Rgb([0, b, a])
            // Rgb([0, a, b])
            // Rgb([a, b, 0])
            // Rgb([s, s, s])
            // Rgb([h, h, h])
            // let x = h / 2 + s / 2;
            // Rgb([x, x, x])
        },
    );

    let image = colored.make_image(args.scale, |rgb| *rgb);

    let output = args
        .output
        .unwrap_or_else(|| format!("day12-{}x{}.png", colored.width(), colored.height()));
    image.save(&output).unwrap();
}
