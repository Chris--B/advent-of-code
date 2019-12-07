
use std::{
    collections::*,
    env,
    fs,
    io::{
        self,
        BufRead,
    },
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
    let input = io::BufReader::new(file);

    let all_coords: Vec<(u32, u32)> = input
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let mut it = line.split(',');
            let a = it.next().unwrap().trim().parse().unwrap();
            let b = it.next().unwrap().trim().parse().unwrap();
            (a, b)
        })
        .collect();

    let (min_x, min_y) = all_coords.iter().cloned().fold((0, 0), |(ax, ay), (x, y)| {
        (ax.min(x), ay.min(y))
    });
    println!("Min Bounds: ({}, {})", min_x, min_y);
    let (max_x, max_y) = all_coords.iter().cloned().fold((0, 0), |(ax, ay), (x, y)| {
        (ax.max(x), ay.max(y))
    });
    println!("Max Bounds: ({}, {})", max_x, max_y);

    // If a coordinate is closest to point X and is along a boundry axis,
    // its area is infinite. Ignore those.

    // Map point in grid -> point in coord list
    let mut closest = HashMap::<(u32, u32), (u32, u32)>::new();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if let Some(coord) = closest_point(&all_coords, (x, y)) {
                closest.insert((x, y), coord);
            } else {
            }
        }
    }

    // Restricted set of original points in coord list.
    let mut coords: HashSet<(u32, u32)> = all_coords.iter().cloned().collect();
    for y in min_y..=max_y {
        for x in [min_x, max_x].iter().cloned() {
            if let Some(p) = closest.get(&(x, y)) {
                coords.remove(&p);
            }
        }
    }
    for x in min_x..=max_x {
        for y in [min_y, max_y].iter().cloned() {
            if let Some(p) = closest.get(&(x, y)) {
                coords.remove(&p);
            }
        }
    }

    // Map point in `coords` to its area.
    let mut areas = HashMap::<(u32, u32), usize>::new();

    for (_point, coord) in closest.iter() {
        if !coords.contains(coord) {
            continue;
        }
        let entry = areas.entry(*coord).or_insert(0);
        *entry += 1;
    }

    let answer = areas.iter().map(|(_coord, area)| area).max().unwrap_or(&0);
    println!("Answer {}", answer);

    Ok(())
}

fn man_dist(a: (u32, u32), b: (u32, u32)) -> usize {
    ((a.0 as i64 - b.0 as i64).abs() +
     (a.1 as i64 - b.1 as i64).abs()) as usize
}

fn closest_point(points: &[(u32, u32)], other: (u32, u32)) -> Option<(u32, u32)> {
    let mut distances: Vec<_> = points
        .iter()
        .map(|p| (man_dist(other, *p) as isize, p))
        .collect();
    distances.sort_by_key(|(d, _p)| *d);

    let a = distances[0].0;
    let b = distances[1].0;
    if a == b {
        None
    } else {
        Some(*distances[0].1)
    }
}

fn run2() -> Result<(), failure::Error> {
    let file = fs::File::open("input.txt")?;
    let input = io::BufReader::new(file);

    let coords: Vec<(u32, u32)> = input
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let mut it = line.split(',');
            let a = it.next().unwrap().trim().parse().unwrap();
            let b = it.next().unwrap().trim().parse().unwrap();
            (a, b)
        })
        .collect();

    let (min_x, min_y) = coords.iter().cloned().fold((0, 0), |(ax, ay), (x, y)| {
        (ax.min(x), ay.min(y))
    });
    let (max_x, max_y) = coords.iter().cloned().fold((0, 0), |(ax, ay), (x, y)| {
        (ax.max(x), ay.max(y))
    });
    println!("Bounds: ({}, {}) ~ ({}, {})", min_x, min_y, max_x, max_y);

    let mut safeties = HashMap::<(u32, u32), usize>::new();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let safety = safeties.entry((x, y)).or_insert(0);
            for coord in coords.iter() {
                *safety += man_dist(*coord, (x, y));
            }
        }
    }

    let answer = safeties.values().filter(|c| **c < 10_000).count();
    println!("Answer {}", answer);

    Ok(())
}
