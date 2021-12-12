use image::ImageBuffer;
use image::Rgba;
use image::{
    gif::{GifEncoder, Repeat},
    Delay, Frame,
};
use std::fs::File;

fn _grayscale_energy(energy: u8) -> Rgba<u8> {
    // Grauscale - maximum energy (9) should be bright, but 0 should be not-black
    let r = (energy as f64) / 9.;
    let rgb = (r + 0.5) * 256.;
    let rgb = rgb as u8;
    Rgba([rgb, rgb, rgb, 0xff])
}

fn palette_energy(energy: u8) -> Rgba<u8> {
    // Matches AOC colors
    const GOLD: Rgba<u8> = Rgba([0xff, 0xff, 0x66, 0xff]);
    const BLUE: Rgba<u8> = Rgba([0x0f, 0x0f, 0x23, 0xff]);
    // const GREEN: Rgba<u8> = Rgba([0x0, 0x99, 0x0, 0xff]);

    const PALETTE: [Rgba<u8>; 5] = [
        BLUE,
        Rgba([0x1f, 0x19, 0x5f, 0xff]),
        Rgba([0x73, 0x53, 0xba, 0xff]),
        Rgba([0xfa, 0xa6, 0xff, 0xff]),
        GOLD,
    ];

    let r = energy as f64 / 9.;
    let i_ish = r * (PALETTE.len() - 1) as f64;
    let i = i_ish.round() as usize;

    PALETTE[i]
}

fn image_of(octs: &[[u8; 10]; 10]) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let scale = 500 / 10;
    let width = octs[0].len() as u32;
    let height = octs.len() as u32;

    let img = ImageBuffer::from_fn(width, height, |x, y| {
        // grayscale_energy(octs[y as usize][x as usize])
        palette_energy(octs[y as usize][x as usize])
    });

    image::imageops::resize(
        &img,
        width * scale,
        height * scale,
        image::imageops::FilterType::Nearest,
    )
}

/// Creates a directory, but returns Ok(()) if it already exists
fn create_dir<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<()> {
    use std::io::ErrorKind;

    match std::fs::create_dir(path) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == ErrorKind::AlreadyExists => Ok(()),
        Err(e) => Err(e),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use aoc21::day11::{parse_input, sim_step};

    // TODO: Clap
    let input_filename = std::env::args().nth(1).expect("Please provide a filename");
    let input = std::fs::read_to_string(&input_filename)?;
    let id = input.lines().next().unwrap_or("err");

    let mut snapshots = vec![];
    let mut octs = parse_input(&input);

    for step in 1.. {
        if step % 100 == 0 {
            eprintln!("simulating step {}...", step);
        }

        snapshots.push(octs);

        if sim_step(&mut octs) == 100 {
            break;
        }
    }

    eprintln!("simulation took {} steps", snapshots.len());

    create_dir(format!("day11-{id:}", id = id))?;
    let images: Vec<ImageBuffer<_, _>> = snapshots.iter().map(image_of).collect();

    let gif_output = File::create(format!("day11-{id:}/day11-{id:}.gif", id = id,))?;

    let mut gif_encoder = GifEncoder::new(gif_output);
    gif_encoder.set_repeat(Repeat::Infinite).unwrap();

    let delay = Delay::from_numer_denom_ms(120, 1);

    let last_frame_idx = images.len() - 1;
    for (t, img) in images.into_iter().enumerate() {
        let name = format!("day11-{id:}/day11-{id:}-{step:03}.png", id = id, step = t);

        println!("Saving step {} to \'{}\'", t, name);

        // img.save(&name)?;

        let frame = if t < last_frame_idx {
            Frame::from_parts(img, 0, 0, delay)
        } else {
            Frame::from_parts(img, 0, 0, Delay::from_numer_denom_ms(10_000, 1))
        };
        gif_encoder.encode_frame(frame).unwrap();
    }

    Ok(())
}
