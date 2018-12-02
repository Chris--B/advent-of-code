
use std::{
    env,
    fs,
    collections::HashMap,
    io::{
        self,
        BufRead,
    },
};

use itertools::Itertools;

use strsim;

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
    let file = fs::File::open("input-01.txt")?;
    let input = io::BufReader::new(file);

    let (twos, threes) = input
        .lines()
        .map(|line| line.unwrap())
        .map(|id| checksum_id(&id))
        .fold((0, 0), |accum, p| {
            (accum.0 + p.0 as u32,
             accum.1 + p.1 as u32)
        });
    println!("{}", twos * threes);

    Ok(())
}

fn run2() -> Result<(), failure::Error> {
    let file = fs::File::open("input-01.txt")?;
    let input = io::BufReader::new(file);

    // Read each line as a String.
    // e.g. "oeylbtcxjqnzhgkyylfapviusr"
    let box_ids: Vec<String> = input
        .lines()
        .map(|line| line.map_err(|e| e.into()))
        .collect::<Result<_, failure::Error>>()?;

    // Create all pairs of all lines, and then filter them
    let ids = box_ids.iter().map(String::as_str);
    let pairs: Vec<_> = ids.clone()
        .cartesian_product(ids)
        // The "<" here removes duplicate pairs
        // since (a, b) == (b, a), for our problem
        .filter(|p| p.0 != p.1 && p.0 < p.1)
        .filter(|p| strsim::hamming(p.0, p.1).unwrap() == 1)
        .collect();
    // We should now only have 1 pair.
    let pair = pairs.first().unwrap();
    assert_eq!(pairs.len(), 1);

    // Combine characters that appear in both ids.
    let result = String::from_utf8(
        pair.0.chars().zip(pair.1.chars())
        .filter_map(|p| {
            if p.0 == p.1 { Some(p.0 as u8)} else { None }
        })
        .collect::<Vec<u8>>()
    ).unwrap();
    assert_eq!(pair.0.len(), result.len()+1);
    println!("{}", result);

    Ok(())
}


fn checksum_id(box_id: &str) -> (bool, bool) {
    let mut counts = HashMap::new();
    for c in box_id.chars() {
        let count = counts.entry(c).or_insert(0);
        *count += 1;
    }

    let twos   = counts.values().find(|count| **count == 2).is_some();
    let threes = counts.values().find(|count| **count == 3).is_some();
    (twos, threes)
}

#[test]
fn check_checksum_id() {
    assert_eq!(checksum_id("abcdef"), (false, false));
    assert_eq!(checksum_id("bababc"), (true,  true));
    assert_eq!(checksum_id("abbcde"), (true,  false));
    assert_eq!(checksum_id("abcccd"), (false, true));
    assert_eq!(checksum_id("aabcdd"), (true,  false));
    assert_eq!(checksum_id("abcdee"), (true,  false));
    assert_eq!(checksum_id("ababab"), (false, true));
}
