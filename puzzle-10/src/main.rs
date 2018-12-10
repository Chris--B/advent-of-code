
use std::{
    collections::*,
    iter::FromIterator,
    env,
    fs,
    io::{
        self,
        BufRead,
    },
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
    let file = fs::File::open("input.txt")?;
    let input = io::BufReader::new(file);

    Ok(())
}

fn run2() -> Result<(), failure::Error> {
    let file = fs::File::open("input.txt")?;
    let _input = io::BufReader::new(file);

    Ok(())
}
