use aoc22::framebuffer::Framebuffer;

use clap::Parser;
use image::Rgba;
use ultraviolet::IVec2;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // #[arg(short, long, default_value = "input/2022/day22.txt")]
    #[arg(short, long, default_value = "input/2022/day22-example.txt")]
    input: String,

    #[arg(short, long)]
    output: Option<String>,

    #[arg(long, default_value = "tileset.png")]
    tileset: String,

    #[arg(short, long, default_value = "10", value_name = "pixel scale factor")]
    scale: u32,
}

fn main() {
    let args = Args::parse();
    let input = std::fs::read_to_string(&args.input).unwrap();

    let mut ground: Vec<IVec2> = vec![];
    let mut walls: Vec<IVec2> = vec![];

    // Start offset from the origin so we have a cute border
    const BORDER: i32 = 2;

    let mut coord = IVec2::new(BORDER, BORDER);
    let mut dims = IVec2::zero();

    for line in input.lines() {
        dims.x = dims.x.max(coord.x + BORDER);
        coord.x = BORDER;

        for b in line.as_bytes().iter().copied() {
            match b {
                b'.' => ground.push(coord),
                b'#' => walls.push(coord),
                _ => { /* Not Rendered, ignore */ }
            }

            coord.x += 1;
        }
        coord.y += 1;

        // We have two lines at the end of the input with no map data, so we get a +2 for free here
        dims.y = dims.y.max(coord.y);
    }

    dbg!(dims);

    let mut colored: Framebuffer<Rgba<u8>> =
        Framebuffer::new_with_ranges_and(0..dims.x, 0..dims.y, |x, y| Rgba([0; 4]));

    for xy in ground {
        colored[xy] = Rgba(0xA3_85_60_FF_u32.to_be_bytes());
    }

    for xy in walls {
        colored[xy] = Rgba(0xF2_E8_6D_FF_u32.to_be_bytes());
    }

    let image = colored.make_image(args.scale, |rgba| *rgba);
    let output = args
        .output
        .unwrap_or_else(|| format!("day22-{}x{}.png", colored.width(), colored.height()));

    image.save(&output).unwrap();
}
