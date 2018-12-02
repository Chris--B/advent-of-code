
use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::BufRead;

fn main() {
    match run1() {
        Ok(()) => {},
        Err(ref err) => eprintln!("{:?}", err),
    }
}

fn run1() -> Result<(), failure::Error> {
    let file = fs::File::open("input-01.txt")?;
    let input = io::BufReader::new(file);

    let mut total_twos = 0;
    let mut total_threes = 0;
    for line in input.lines() {
        let box_id = line?;
        let (twos, threes) = checksum_id(&box_id);
        if twos { total_twos += 1; }
        if threes { total_threes += 1; }
    }

    println!("{}", total_twos * total_threes);

    Ok(())
}

fn checksum_id(box_id: &str) -> (bool, bool) {
    let mut counts = HashMap::new();
    for c in box_id.chars() {
        let count = counts.entry(c).or_insert(0);
        *count += 1;
    }

    let mut twos   = counts.values().find(|count| **count == 2).is_some();
    let mut threes = counts.values().find(|count| **count == 3).is_some();
    (twos, threes)
}

#[test]
fn check_checksum_id() {
    assert_eq!(checksum_id("abcdef"), (false, false));
    assert_eq!(checksum_id("bababc"), (true, true));
    assert_eq!(checksum_id("abbcde"), (true, false));
    assert_eq!(checksum_id("abcccd"), (false, true));
    assert_eq!(checksum_id("aabcdd"), (true, false));
    assert_eq!(checksum_id("abcdee"), (true, false));
    assert_eq!(checksum_id("ababab"), (false, true));
}
