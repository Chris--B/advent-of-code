use aoc24::day12::*;
use aoc24::prelude::*;

use image::Rgb;

fn main() {
    let input_path = std::env::args().nth(1);
    let input_path = input_path.as_deref().unwrap_or("input/2024/day12.txt");

    println!("Trying to load data from {input_path}");
    let input = std::fs::read_to_string(input_path).unwrap();
    let mut plots = Framebuffer::parse_grid_char(&input);
    plots.set_border_color(Some('@'));

    let target = 1024.;
    let actual = plots.width().max(plots.height()) as f32;
    let scale = target / actual;

    {
        let img = plots.make_image(scale.round() as _, |&plot| Rgb(make_color(plot as u8, 26)));
        let out_path = "target/day12.png";
        img.save(out_path).unwrap();
        println!("Saving to {out_path}");
    }

    {
        let mut edges = plots.clone();
        for p in edges.iter_coords() {
            if !is_edge(&plots, p.into()) {
                edges[p] = '.';
            }
        }
        let img = edges.make_image(scale.round() as _, |&plot| {
            if plot == '.' {
                Rgb([0_u8, 0, 0])
            } else {
                Rgb(make_color(plot as u8, 26))
            }
        });
        let out_path = "target/day12_edges.png";
        img.save(out_path).unwrap();
        println!("Saving to {out_path}");
    }
}

fn make_color(i: u8, n: usize) -> [u8; 3] {
    let hue = (i as f32 * 360.0 / n as f32) % 360.0;
    let c = 0.63;
    let x = c * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs());
    let m = 0.27;

    let (r, g, b) = match hue as u32 {
        0..=59 => (c, x, 0.0),
        60..=119 => (x, c, 0.0),
        120..=179 => (0.0, c, x),
        180..=239 => (0.0, x, c),
        240..=299 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    [
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    ]
}
