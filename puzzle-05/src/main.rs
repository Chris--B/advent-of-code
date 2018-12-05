#![allow(unused_imports, unused_variables)]

use std::{
    env,
    fs,
    collections,
    io::{
        self,
        Read,
        BufRead,
    },
    str::FromStr,
};

use failure::bail;

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
    let mut input = io::BufReader::new(file);
    let mut units = String::new();
    input.read_to_string(&mut units);

    println!("Units: {}", collapse_polymer(units.as_str().trim()).len());

    Ok(())
}

fn run2() -> Result<(), failure::Error> {
    let file = fs::File::open("input.txt")?;
    let input = io::BufReader::new(file);

    Ok(())
}

fn is_upper(c: char) -> bool {
    c == c.to_ascii_uppercase()
}

fn is_lower(c: char) -> bool {
    c == c.to_ascii_lowercase()
}

fn collapse_polymer(polymer: &str) -> String {
    let mut res = String::new();
    let mut cur = polymer.chars();
    res.push(cur.next().unwrap());
    loop {
        let mut dirty = false;
        loop {
            if let Some(next) = cur.next() {
                let a: char = res.chars().last().unwrap();
                let b: char = next;
                if (is_lower(a) && is_upper(b) && a.to_ascii_uppercase() == b) ||
                   (is_upper(a) && is_lower(b) && a.to_ascii_lowercase() == b)
                {
                    res.pop();
                } else {
                    dirty = true;
                    res.push(next)
                }
            } else {
                break;
            }
        }
        if !dirty {
            break;
        }
    }

    res
}

#[test]
fn check() {
    assert_eq!(collapse_polymer("aA"),     "");
    assert_eq!(collapse_polymer("abBA"),   "");
    assert_eq!(collapse_polymer("abAB"),   "abAB");
    assert_eq!(collapse_polymer("aabAAB"), "aabAAB");
    assert_eq!(collapse_polymer("dabAcCaCBAcCcaDA"), "dabCBAcaDA");
}
