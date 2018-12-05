
use std::{
    env,
    fs,
    collections,
    io::{
        self,
        Read,
    },
    time,
};

fn main() {
    let run = env::args().nth(1).unwrap_or("2".to_string());
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
    input.read_to_string(&mut units)?;

    println!("Units: {}", collapse(units.as_str().trim().chars()).len());

    Ok(())
}

fn run2() -> Result<(), failure::Error> {
    let file = fs::File::open("input.txt")?;
    let mut input = io::BufReader::new(file);
    let mut units = String::new();
    input.read_to_string(&mut units)?;

    let before = time::Instant::now();
    println!("Units: {}", optimize(units.as_str().trim()).len());
    let after = time::Instant::now();

    println!("{}", after.duration_since(before).subsec_millis());

    Ok(())
}

fn is_upper(c: char) -> bool {
    c == c.to_ascii_uppercase()
}

fn is_lower(c: char) -> bool {
    c == c.to_ascii_lowercase()
}

fn collapse(mut cur: impl Iterator<Item=char>) -> String {
    let mut res = String::new();
    res.push(cur.next().unwrap());
    loop {
        let mut dirty = false;
        loop {
            if let Some(next) = cur.next() {
                if res.is_empty() {
                    res.push(next);
                } else {
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

fn optimize(polymer: &str) -> String {
    let mut units = collections::HashSet::new();
    for unit in polymer.chars() {
        units.insert(unit.to_ascii_lowercase());
    }
    let units = units;
    let trial_polymers: Vec<String> = units
    .iter()
    .map(|unit| {
        let trial_polymer = polymer
            .chars()
            .filter(|c| *c != *unit && *c != unit.to_ascii_uppercase());
        collapse(trial_polymer)
    })
    .collect();

    trial_polymers.iter().min_by_key(|polymer| polymer.len()).unwrap().clone()
}

#[test]
fn check() {
    assert_eq!(collapse("aA".chars()),     "");
    assert_eq!(collapse("abBA".chars()),   "");
    assert_eq!(collapse("abAB".chars()),   "abAB");
    assert_eq!(collapse("aabAAB".chars()), "aabAAB");
    assert_eq!(collapse("dabAcCaCBAcCcaDA".chars()), "dabCBAcaDA");
}
