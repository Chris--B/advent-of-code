use std::{fs::File, io::BufWriter};

use aoc22::day16;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // #[arg(short, long, default_value = "input/2022/day16.txt")]
    #[arg(short, long, default_value = "input/2022/day16-example.txt")]
    input: String,

    #[arg(short, long)]
    output: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let input = std::fs::read_to_string(&args.input)
        .unwrap_or_else(|_| panic!("Failed to find {}", args.input));

    let full_day = day16::parse(&input);
    full_day.print_dot(&mut BufWriter::new(
        File::create("target/day16-full.dot")
            .unwrap_or_else(|_| panic!("Failed to find {}", "target/day16-full.dot")),
    ))?;

    let simple_day = day16::simplify_tunnels(full_day.clone());
    simple_day.print_dot(&mut BufWriter::new(
        File::create("target/day16-simple.dot")
            .unwrap_or_else(|_| panic!("Failed to find {}", "target/day16-simple.dot")),
    ))?;

    Ok(())
}
