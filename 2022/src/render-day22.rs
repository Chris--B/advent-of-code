use aoc22::framebuffer::Framebuffer;

use clap::Parser;
use image::Rgb;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "input/2022/day22.txt")]
    input: String,

    #[arg(short, long)]
    output: Option<String>,

    #[arg(long, default_value = "tileset.png")]
    tileset: String,

    #[arg(short, long, default_value = "2", value_name = "pixel scale factor")]
    scale: u32,
}

fn main() {
    let args = Args::parse();
    let input = std::fs::read_to_string(&args.input).unwrap();

    let colored: Framebuffer<Rgb<u8>> =
        Framebuffer::new_with_ranges_and(0..256, 0..256, |x, y| Rgb([x as u8, y as u8, 255 / 2]));

    let image = colored.make_image(args.scale, |rgb| *rgb);
    let output = args
        .output
        .unwrap_or_else(|| format!("day22-{}x{}.png", colored.width(), colored.height()));

    image.save(&output).unwrap();
}
