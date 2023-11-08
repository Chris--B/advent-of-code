use std::env;
use std::fs;
use std::io::Read;

use intcode::vm;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = env::args().nth(1).unwrap_or_else(|| "example.intcode".into());
    println!("Reading {filename}");

    let intcode_mem: Vec<vm::Atom> = {
        let mut buffer = Vec::<u8>::new();

        let mut file = fs::File::open(filename)?;
        file.read_to_end(&mut buffer)?;

        let parse_results: Result<Vec<_>, _> = String::from_utf8(buffer)?
            .trim()
            .split(',')
            .map(|atom_str| atom_str.parse())
            .collect();

        parse_results?
    };

    println!("{}", vm::pretty_fmt_memory(&intcode_mem)?);

    Ok(())
}
