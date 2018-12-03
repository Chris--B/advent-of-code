
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

#[derive(Debug, Copy, Clone, PartialEq)]
struct Claim {
    id:          u32,
    offset_left: u32,
    offset_top:  u32,
    width:       u32,
    height:      u32,
}

impl FromStr for Claim {
    type Err = failure::Error;

    fn from_str(s: &str) -> Result<Claim, failure::Error> {
        let mut parts = s.split(" ");

        let id: u32 = parts.next().unwrap()[1..].parse()?;

        assert_eq!(parts.next(), Some("@"), "Expected an '@': {}", s);

        let mut offsets = parts.next().unwrap().split(",").map(|t| {
            if t.ends_with(":") {
                &t[..t.len()-1]
            } else {
                t
            }
        });
        let offset_left: u32 = offsets.next().unwrap().parse()?;
        let offset_top:  u32 = offsets.next().unwrap().parse()?;

        let mut dims = parts.next().unwrap().split("x");
        let width:  u32 = dims.next().unwrap().parse()?;
        let height: u32 = dims.next().unwrap().parse()?;

        Ok(Claim {
            id,
            offset_left,
            offset_top,
            width,
            height,
        })
    }
}

#[test]
fn check_claim() {
    assert_eq!(
        "#123 @ 3,2: 5x4".parse::<Claim>().unwrap(),
        Claim {
            id: 123,
            offset_left: 3,
            offset_top:  2,
            width:       5,
            height:      4,
        });
}

impl Claim {
    fn sq_inches(&self) -> impl Iterator<Item=(u32, u32)> {
        struct Iter {
            claim: Claim,
            index: u32,
        }

        impl Iterator for Iter {
            type Item = (u32, u32);
            fn next(&mut self) -> Option<Self::Item> {
                let mut x = self.index % self.claim.width;
                let mut y = self.index / self.claim.width;

                if y >= self.claim.height {
                    return None;
                }
                self.index += 1;

                x += self.claim.offset_left;
                y += self.claim.offset_top;

                Some((x, y))
            }
        }

        Iter {
            claim: *self,
            index: 0,
        }
    }
}

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
    let file = fs::File::open("input-1.txt")?;
    let input = io::BufReader::new(file);

    let mut fabric = collections::HashMap::new();
    input.lines()
        .map(|l| l.unwrap())
        .map(|l| l.parse::<Claim>().unwrap())
        .for_each(|claim| {
            for point in claim.sq_inches() {
                let entry = fabric.entry(point).or_insert(0);
                *entry += 1;
            }
        });

    let mut overlaps = 0;
    for (point, count) in fabric.iter() {
        if *count > 1 {
            // println!("{:?}", point);
            overlaps += 1;
        }
    }
    println!("Overlaps: {}", overlaps);

    Ok(())
}

fn run2() -> Result<(), failure::Error> {
    let file = fs::File::open("input-2.txt")?;
    let input = io::BufReader::new(file);

    let mut fabric = collections::HashMap::new();
    let claims: Vec<Claim> = input.lines()
        .map(|l| l.unwrap())
        .map(|l| l.parse::<Claim>().unwrap())
        .collect();

    for claim in claims.iter() {
        for point in claim.sq_inches() {
            let entry = fabric.entry(point).or_insert(0);
            *entry += 1;
        }
    }

    'claim:
    for claim in claims.iter() {
        for point in claim.sq_inches() {
            if fabric.get(&point).unwrap() != &1 {
                continue 'claim;
            }
        }
        println!("{:?}", claim);
    }

    Ok(())
}
