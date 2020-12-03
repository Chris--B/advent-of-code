use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use smallvec::SmallVec;

#[derive(Clone, Debug, Default)]
pub struct Line {
    min: u8,
    max: u8,
    c: u8,
    password: SmallVec<[u8; 32]>,
}

#[aoc_generator(day2)]
pub fn parse_input(input: &str) -> Vec<Line> {
    // Lines look like this:
    //      9-10 b: bbktbbbxhfbpb
    let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();

    input
        .lines()
        .map(|line| line.trim())
        .map(|line| {
            let caps = re.captures(line).unwrap();

            Line {
                min: caps[1].parse().unwrap(),
                max: caps[2].parse().unwrap(),
                c: caps[3].as_bytes()[0],
                password: caps[4].as_bytes().into(),
            }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[Line]) -> usize {
    input
        .iter()
        .filter(|l: &&Line| password_is_valid(l.c, l.min, l.max, &l.password))
        .count()
}

fn password_is_valid(c: u8, min: u8, max: u8, password: &[u8]) -> bool {
    let mut count = 0;

    for x in password {
        if *x == c {
            count += 1;
            if count > max {
                return false;
            }
        }
    }

    count >= min
}

#[test]
fn part1_check_ex() {
    assert_eq!(true, password_is_valid(b'a', 1, 3, b"abcde"));
    assert_eq!(false, password_is_valid(b'b', 1, 3, b"cdefg"));
    assert_eq!(true, password_is_valid(b'c', 2, 9, b"ccccccccc"));
}

#[aoc(day2, part2)]
pub fn part2(input: &[Line]) -> usize {
    input
        .iter()
        .filter(|l: &&Line| password_is_valid_p2(l.c, l.min, l.max, &l.password))
        .count()
}

fn password_is_valid_p2(c: u8, min: u8, max: u8, password: &[u8]) -> bool {
    // just kidding, they're indices now
    let a = min as usize - 1;
    let b = max as usize - 1;

    (password[a] == c) ^ (password[b] == c)
}

#[test]
fn part2_check_ex() {
    assert_eq!(true, password_is_valid_p2(b'a', 1, 3, b"abcde"));
    assert_eq!(false, password_is_valid_p2(b'b', 1, 3, b"cdefg"));
    assert_eq!(false, password_is_valid_p2(b'c', 2, 9, b"ccccccccc"));
}
