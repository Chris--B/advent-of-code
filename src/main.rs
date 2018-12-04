#![allow(unused_imports, unused_variables)]

use std::{
    env,
    fs,
    collections,
    io::{
        self,
        BufRead,
    },
    str::FromStr,
};

fn main() {
    let run = env::args().nth(1).unwrap_or("1".to_string());
    if run == "1" {
        match run1() {
            Ok(()) => {},
            Err(ref err) => eprintln!("{:?}", err),
        }
    } else if run == "2" {
        match run2() {
            Ok(()) => {},
            Err(ref err) => eprintln!("{:?}", err),
        }
    }
}

fn run1() -> Result<(), failure::Error> {
    let file = fs::File::open("input-1.txt")?;
    let input = io::BufReader::new(file);

    Ok(())
}

fn run2() -> Result<(), failure::Error> {
    let file = fs::File::open("input-2.txt")?;
    let input = io::BufReader::new(file);

    Ok(())
}
