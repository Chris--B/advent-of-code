use aoc24::framebuffer::Framebuffer;
use image::Rgb;

#[allow(unused)]
const EXAMPLE_JUST_9: &str = r"
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

fn main() {
    let input_path = std::env::args().nth(1);
    let input_path = input_path.as_deref().unwrap_or("input/2024/day10.txt");

    println!("Trying to load heightmap data from {input_path}");
    let input = std::fs::read_to_string(input_path).unwrap();
    // let input = EXAMPLE_JUST_9.trim();
    let heights = Framebuffer::parse_grid(&input, |c| c as u8 - b'0');

    let target = 1024.;
    let actual = heights.width().max(heights.height()) as f32;
    let scale = target / actual;

    let img = heights.make_image(scale.round() as _, |&h| {
        let x: f32 = (h as f32 + 2.) / 12.;
        let x = x + 0.3;
        let r: u8 = (0x26 as f32 * x) as u8;
        let g: u8 = (0x46 as f32 * x) as u8;
        let b: u8 = (0x53 as f32 * x) as u8;

        match h {
            0 => Rgb([0, 0, 0]),
            9 => Rgb([0xff, 0xff, 0x66]),
            _ => Rgb([r, g, b]),
        }
    });

    let out_path = "target/day10.png";
    img.save(out_path).unwrap();
    println!("Saving to {out_path}");
}
