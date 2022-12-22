use clap::Parser;
use image::*;
use noise::{NoiseFn, Perlin};
use rand::prelude::*;
use ultraviolet::UVec2;

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

    #[arg(short, long, default_value = "2", value_name = "pixel scale factor")]
    scale: u32,
}

fn pick_terrain(perlin: &Perlin, x: u32, y: u32) -> f64 {
    let x = x as f64 / (2. * 23.);
    let y = y as f64 / (2. * 23.);
    let n = perlin.get([x, y]);

    0.5 * (n + 1.)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let input = std::fs::read_to_string(&args.input).unwrap();

    let mut ground: Vec<UVec2> = vec![];
    let mut walls: Vec<UVec2> = vec![];

    // Start offset from the origin so we have a cute border
    const BORDER: u32 = 0;

    let mut coord = UVec2::new(BORDER, BORDER);
    let mut dims = UVec2::zero();

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
    dims.y -= 2;

    let tile_size = 8;
    dims *= 2 * tile_size;

    let tileset: RgbaImage = image::open(&args.tileset).unwrap().to_rgba8();
    let ground_tile: RgbaImage = tileset.view(132, 176, tile_size, tile_size).to_image();
    let grass_tile: RgbaImage = tileset.view(148, 216, tile_size, tile_size).to_image();
    let wall_tile: RgbaImage = tileset
        .view(339, 176, 2 * tile_size, 2 * tile_size)
        .to_image();

    let mut map = RgbaImage::new(dims.x, dims.y);

    let perlin = Perlin::new(thread_rng().gen());

    for xy in ground {
        let (x, y) = (2 * tile_size * xy).into();

        for (x, y) in [
            (x, y),
            (x + tile_size, y),
            (x, y + tile_size),
            (x + tile_size, y + tile_size),
        ] {
            let n = pick_terrain(&perlin, x, y);

            let tile = if n < 0.5 { &grass_tile } else { &ground_tile };
            map.copy_from(tile, x, y).unwrap();
        }
    }

    for xy in walls {
        let (x, y) = (2 * tile_size * xy).into();
        map.copy_from(&wall_tile, x, y).unwrap();
    }

    let output = args
        .output
        .unwrap_or_else(|| format!("day22-{}x{}.png", map.width(), map.height()));

    if args.scale != 0 {
        let w = map.width() * args.scale;
        let h = map.height() * args.scale;
        map = imageops::resize(&map, w, h, imageops::FilterType::Nearest);
    }
    map.save(&output).unwrap();

    Ok(())
}
