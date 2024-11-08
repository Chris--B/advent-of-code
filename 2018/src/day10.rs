use std::{
    collections::*,
    env, fs,
    io::{self, BufRead},
};

use aoc_runner_derive::{aoc, aoc_generator};
use failure::bail;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
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
            x: parts[1].parse()?,
            y: parts[2].parse()?,
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

fn render_stars(mut points: Vec<Point>) -> u32 {
    let mut t = 0;

    loop {
        let (minx, miny, maxx, maxy) = aabb(&points);
        let last_points = points.clone();
        let last_area = (maxx - minx) * (maxy - miny);
        for p in points.iter_mut() {
            p.advance();
        }
        let (minx, miny, maxx, maxy) = aabb(&points);
        let area = (maxx - minx) * (maxy - miny);

        if area > last_area {
            println!("Star Map @ t={}", t);
            for y in miny..=maxy {
                for x in minx..=maxx {
                    if last_points.iter().find(|p| p.x == x && p.y == y).is_some() {
                        // Unicode "Full Block"
                        // See: https://www.compart.com/en/unicode/block/U+2580
                        print!("\u{2588}");
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

    t
}

// #[aoc(day10, part1)]
// fn run1(input: &str) -> Result<i32, failure::Error> {
//     let points = input.lines()
//         .map(|line| line.parse().unwrap())
//         .collect::<Vec<Point>>();
//     render_stars(points);

//     // Not really a solution that we can get programatically...
//     Ok(0)
// }

#[aoc(day10, part2)]
fn run2(input: &str) -> Result<u32, failure::Error> {
    let points = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<Point>>();

    Ok(render_stars(points))
}
