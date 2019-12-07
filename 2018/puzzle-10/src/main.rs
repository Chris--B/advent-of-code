
use std::{
    collections::*,
    env,
    fs,
    io::{
        self,
        BufRead,
    },
};

use lazy_static::lazy_static;
use regex::Regex;

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

#[derive(Copy, Clone)]
struct Point {
    x:  i64,
    y:  i64,
    dx: i64,
    dy: i64,
}

impl Point {
    fn advance(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
    }
}

impl std::str::FromStr for Point {
    type Err = failure::Error;
    fn from_str(s: &str) -> Result<Point, failure::Error> {
        let ss = s
            .replace("=", " ")
            .replace("<", " ")
            .replace(">", " ")
            .replace(",", " ");
        let parts: Vec<_> = ss.split(" ").filter(|s| !s.is_empty()).collect();
        Ok(Point {
            x:  parts[1].parse()?,
            y:  parts[2].parse()?,
            dx: parts[4].parse()?,
            dy: parts[5].parse()?,
        })
    }
}

fn aabb(points: &[Point]) -> (i64, i64, i64, i64) {
    let mut minx = points[0].x;
    let mut miny = points[0].y;
    let mut maxx = points[0].x;
    let mut maxy = points[0].y;

    for p in points.iter() {
        minx = minx.min(p.x);
        miny = miny.min(p.y);
        maxx = maxx.max(p.x);
        maxy = maxy.max(p.y);
    }

    (minx, miny, maxx, maxy)
}

fn run1() -> Result<(), failure::Error> {
    let file = fs::File::open("input.txt")?;
    let input = io::BufReader::new(file);

    let mut points = input.lines()
        .map(|line| line.unwrap())
        .map(|line| line.parse().unwrap())
        .collect::<Vec<Point>>();

    let mut t = 0;

    loop {
        let (minx, miny, maxx, maxy) = aabb(&points);
        let last_points = points.clone();
        let last_area   = (maxx-minx) * (maxy-miny);
        for p in points.iter_mut() {
            p.advance();
        }
        let (minx, miny, maxx, maxy) = aabb(&points);
        let area = (maxx-minx) * (maxy-miny);

        if area > last_area {
            println!("t = {}", t);
            for y in miny..=maxy {
                for x in minx..=maxx {
                    if last_points.iter()
                        .find(|p| p.x == x && p.y == y)
                        .is_some()
                    {
                        print!(".");
                    } else {
                        print!(" ");
                    }
                }
                println!("");
            }
            break;
        }
        t += 1;
    }

    Ok(())
}

fn run2() -> Result<(), failure::Error> {
    let file = fs::File::open("input.txt")?;
    let _input = io::BufReader::new(file);

    Ok(())
}
