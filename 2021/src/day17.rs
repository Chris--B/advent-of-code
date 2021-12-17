use aoc_runner_derive::{aoc, aoc_generator};

use std::cmp::Ordering;

#[derive(Copy, Clone, Debug)]
pub struct Area {
    x: (i64, i64),
    y: (i64, i64),
}

impl Area {
    fn contains(&self, p: (i64, i64)) -> bool {
        (self.x.0 <= p.0 && p.0 <= self.x.1) && (self.y.0 <= p.1 && p.1 <= self.y.1)
    }
}

#[aoc_generator(day17)]
pub fn parse_input(input: &str) -> Area {
    // target area: x=192..251, y=-89..-59
    let (x0, x1, y0, y1) =
        scan_fmt::scan_fmt!(input, "target area: x={}..{}, y={}..{}", i64, i64, i64, i64).unwrap();

    Area {
        x: (x0, x1),
        y: (y0, y1),
    }
}

fn sign(x: i64) -> i64 {
    match x.cmp(&0) {
        Ordering::Greater => -1,
        Ordering::Less => 1,
        Ordering::Equal => 0,
    }
}

fn step(pos: &mut (i64, i64), vel: &mut (i64, i64)) {
    // The probe's x position increases by its x velocity.
    pos.0 += vel.0;

    // The probe's y position increases by its y velocity.
    pos.1 += vel.1;

    // Due to drag, the probe's x velocity changes by 1 toward the value 0; that is,
    //      it decreases by 1 if it is greater than 0,
    //      increases by 1 if it is less than 0, or
    //      does not change if it is already 0.
    match vel.0.cmp(&0) {
        Ordering::Greater => vel.0 -= 1,
        Ordering::Less => vel.0 += 1,
        Ordering::Equal => {}
    }

    // Due to gravity, the probe's y velocity decreases by 1.
    vel.1 -= 1;
}

fn check_vel(mut vel: (i64, i64), area: &Area) -> Option<(i64, i64)> {
    let mut pos = (0, 0);
    let left_side = (sign(pos.0 - area.x.0), sign(pos.1 - area.y.0));
    let right_side = (sign(pos.0 - area.x.1), sign(pos.1 - area.y.1));

    let mut max = pos;
    let mut ever_in_area = false;

    loop {
        // println!("({}, {})", pos.0, pos.1);

        if pos.1 > max.1 {
            max = pos;
        }

        if area.contains(pos) {
            ever_in_area = true;
        }

        let new_left_side = (sign(pos.0 - area.x.0), sign(pos.1 - area.y.0));
        let new_right_side = (sign(pos.0 - area.x.1), sign(pos.1 - area.y.1));

        if new_left_side != left_side && new_right_side != right_side {
            break;
        }

        step(&mut pos, &mut vel);
    }

    if ever_in_area {
        Some(max)
    } else {
        None
    }
}

// Part1 ======================================================================
#[aoc(day17, part1)]
#[inline(never)]
pub fn part1(area: &Area) -> i64 {
    let mut peaks = vec![];

    for y in 1..=100 {
        for x in -100..=100 {
            if let Some(peak) = check_vel((x, y), area) {
                peaks.push(peak);
            }
        }
    }

    *peaks.iter().map(|(_x, y)| y).max().unwrap()
}

// Part2 ======================================================================
#[aoc(day17, part2)]
#[inline(never)]
pub fn part2(area: &Area) -> usize {
    let mut vels = vec![];

    for y in -260..=260 {
        for x in -260..=260 {
            if check_vel((x, y), area).is_some() {
                vels.push((x, y));
            }
        }
    }

    // println!("min_x={}", vels.iter().map(|(x, _y)| x).min().unwrap());
    // println!("max_x={}", vels.iter().map(|(x, _y)| x).max().unwrap());
    // println!("min_y={}", vels.iter().map(|(_x, y)| y).min().unwrap());
    // println!("max_y={}", vels.iter().map(|(_x, y)| y).max().unwrap());

    vels.len()
}

#[test]
fn check_example_1() {
    let input = "target area: x=20..30, y=-10..-5";
    assert_eq!(part1(&parse_input(input)), 45);
}

#[test]
fn check_example_2() {
    let input = "target area: x=20..30, y=-10..-5";
    assert_eq!(part2(&parse_input(input)), 112);
}
