use aoc_runner_derive::{aoc, aoc_generator};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct Passport {
    /// Birth Year
    byr: String,

    /// Issue Year
    iyr: String,

    /// Expiration Year
    eyr: String,

    /// Height
    hgt: String,

    /// Hair Color
    hcl: String,

    /// Eye Color
    ecl: String,

    /// Passport ID
    pid: String,

    /// Country ID
    cid: Option<String>,
}

impl Passport {
    fn from_text<'a>(lines: impl Iterator<Item = &'a str>) -> Option<Self> {
        let mut byr = None;
        let mut iyr = None;
        let mut eyr = None;
        let mut hgt = None;
        let mut hcl = None;
        let mut ecl = None;
        let mut pid = None;
        let mut cid = None;

        // Match k/v pairs
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?P<key>[a-z]{3}):(?P<val>\S+)").unwrap();
        }

        for line in lines {
            for entry in line.split_whitespace() {
                let caps = RE.captures(entry).unwrap();

                match &caps["key"] {
                    "byr" => {
                        assert_eq!(byr, None);
                        byr = Some(caps["val"].to_string());
                    }
                    "iyr" => {
                        assert_eq!(iyr, None);
                        iyr = Some(caps["val"].to_string());
                    }
                    "eyr" => {
                        assert_eq!(eyr, None);
                        eyr = Some(caps["val"].to_string());
                    }
                    "hgt" => {
                        assert_eq!(hgt, None);
                        hgt = Some(caps["val"].to_string());
                    }
                    "hcl" => {
                        assert_eq!(hcl, None);
                        hcl = Some(caps["val"].to_string());
                    }
                    "ecl" => {
                        assert_eq!(ecl, None);
                        ecl = Some(caps["val"].to_string());
                    }
                    "pid" => {
                        assert_eq!(pid, None);
                        pid = Some(caps["val"].to_string());
                    }
                    "cid" => {
                        assert_eq!(cid, None);
                        cid = Some(caps["val"].to_string());
                    }
                    _ => unreachable!(),
                }
            }
        }

        Some(Passport {
            byr: byr?,
            iyr: iyr?,
            eyr: eyr?,
            hgt: hgt?,
            hcl: hcl?,
            ecl: ecl?,
            pid: pid?,
            cid,
        })
    }

    fn is_valid(&self) -> bool {
        // Valid byr
        (
            (1920..=2002).contains(&self.byr.parse().unwrap_or_default())
        )
        &&

        // Valid iyr
        (
            (2010..=2020).contains(&self.iyr.parse().unwrap_or_default())
        )
        &&

        // Valid eyr
        (
            (2020..=2030).contains(&self.eyr.parse().unwrap_or_default())
        )
        &&

        // Valid hgt
        ({
            // TODO: This may be wasted work, look into `get_unchecked` to skip it.
            let ascii: &[u8] = self.hgt.as_bytes();
            let (num_bytes, unit) = ascii.split_at(ascii.len() - 2);
            let num_str = std::str::from_utf8(num_bytes).unwrap();

            match unit {
                b"cm" => {
                    if let Ok(num) = num_str.parse::<u8>() {
                        (150..=193).contains(&num)
                    } else { false }
                }
                b"in" => {
                    if let Ok(num) = num_str.parse::<u8>() {
                        (59..=76).contains(&num)
                    } else { false }
                }
                _ => {
                    false
                }
            }
        })
        &&

        // Valid hcl
        ({
            lazy_static! {
                static ref RE: Regex = Regex::new("#[0-9a-f]{6}").unwrap();
            }
            RE.is_match(&self.hcl)
        })
        &&

        // Valid ecl
        (matches!(
            self.ecl.as_str(),
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
        ))
        &&

        // Valid pid
        ({
            lazy_static! {
                static ref RE: Regex = Regex::new("^[0-9]{9}$").unwrap();
            }
            RE.is_match(&self.pid)
        })
        &&

        // Valid cid
        ({
            // Ignored, so always valid
            true
        })
    }
}

#[aoc_generator(day4)]
pub fn parse_input(input: &str) -> Vec<Passport> {
    input
        .trim()
        .lines()
        .chunk_by(|l| l.trim().is_empty())
        .into_iter()
        .filter_map(|(is_empty, record)| if is_empty { None } else { Some(record) })
        .filter_map(Passport::from_text)
        .collect()
}

#[aoc(day4, part1)]
pub fn part1(input: &[Passport]) -> usize {
    input.len()
}

#[test]
fn check_part1_ex() {
    let input: &str = r#"
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
"#;
    assert_eq!(2, part1(&parse_input(input)));
}

#[aoc(day4, part2)]
pub fn part2(input: &[Passport]) -> usize {
    input
        .iter()
        .filter(|passport: &&Passport| passport.is_valid())
        .count()
}

#[test]
fn check_part2_ex_invalid() {
    let input: &str = r#"
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
"#;
    assert_eq!(0, part2(&parse_input(input)));
}

#[test]
fn check_part2_ex_valid() {
    let input: &str = r#"
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
"#;
    assert_eq!(4, part2(&parse_input(input)));
}
