use std::{
    collections::HashSet,
    fs,
    io,
    io::BufRead,
};

use failure;

fn main() {
    match run2() {
        Ok(()) => {},
        Err(ref err) => eprintln!("{:?}", err),
    }
}

fn run1() -> Result<(), failure::Error> {
    let file = fs::File::open("input-1.txt")?;
    let input = io::BufReader::new(file);

    let mut freqs: Vec<i32> = vec![];
    for line in input.lines() {
        let line = line?;
        freqs.push(line.parse()?);
    }
    let freq_change: i32 = freqs.iter().sum();
    println!("{}", freq_change);

    Ok(())
}

fn run2() -> Result<(), failure::Error> {
    let file = fs::File::open("input-2.txt")?;
    let input = io::BufReader::new(file);

    let mut changes: Vec<i32> = vec![];
    for line in input.lines() {
        let line = line?;
        changes.push(line.parse()?);
    }

    let mut seen = HashSet::new();
    let mut freq = 0;
    seen.insert(freq);

    for change in changes.iter().cycle() {
        freq += change;

        if seen.contains(&freq) {
            break;
        }
        seen.insert(freq);
    }
    println!("{}", freq);

    Ok(())
}
