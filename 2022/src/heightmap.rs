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

    #[arg(short, long, default_value = "1", value_name = "pixel scale factor")]
    scale: u32,
}

fn main() {
    let args = Args::parse();

    let input = std::fs::read_to_string(&args.input).unwrap();

    let day = parse(&input);
    let total_steps_map = find_path(&day, day.start);
    let max_steps = total_steps_map
        .iter_coords()
        .map(|pt| total_steps_map[pt])
        .max()
        .unwrap();

    let colored: Framebuffer<Rgb<u8>> = Framebuffer::new_with_ranges_and(
        day.heightmap.range_x(),
        day.heightmap.range_y(),
        |x, y| {
            let h = 255. * day.heightmap[(x, y)] as f32 / 25.0;
            let s = 255. * total_steps_map[(x, y)] as f32 / max_steps as f32;
            Rgb([s as u8, h as u8, 0_u8])
        },
    );

    let image = colored.make_image(args.scale, |rgb| *rgb);
    image.save("out.png").unwrap();
}
