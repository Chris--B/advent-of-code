
use std::{
    collections::*,
    env,
    fs,
    io::{
        self,
        Read,
        BufRead,
    },
    time,
};

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
    let mut xxx = 0;
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if let Some(coord) = closest_point(&all_coords, (x, y)) {
                closest.insert((x, y), coord);
            } else {
                // println!("{}, {}", x, y);
                xxx += 1;
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

    // println!("Finite Area Points:");
    for (x, y) in coords.iter() {
        // println!("({}, {})", x, y);
    }
    // println!("closest.len() == {}", closest.len());
    // println!("xxx           == {}", xxx);
    // Map point in `coords` to its area.
    let mut areas = HashMap::<(u32, u32), usize>::new();

    for (_point, coord) in closest.iter() {
        if !coords.contains(coord) {
            continue;
        }
        let entry = areas.entry(*coord).or_insert(0);
        *entry += 1;
    }
    // println!("");

    // println!("Areas:");
    for (coord, area) in areas.iter() {
        // println!("  {:>3}, {:>3} -> {:>3}", coord.0, coord.1, area);
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
    let mut closest_point = points[0];
    let mut closest_dist  = man_dist(closest_point, other);


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
    let _input = io::BufReader::new(file);

    Ok(())
}
