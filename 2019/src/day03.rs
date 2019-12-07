use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn xy(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn manhattan(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

use std::collections::HashSet;

pub struct Wire {
    set: HashSet<Point>,
    list: Vec<Point>,
}

impl Wire {
    fn new() -> Wire {
        Wire {
            set: HashSet::new(),
            list: Vec::new(),
        }
    }

    fn insert(&mut self, p: Point) {
        self.set.insert(p);
        self.list.push(p);
    }

    fn intersection<'a>(&'a self, other: &'a Wire) -> impl Iterator<Item = &'a Point> {
        self.set.intersection(&other.set)
    }

    fn steps_for(&self, p: &Point) -> usize {
        assert!(self.set.contains(p));
        for (i, q) in self.list.iter().enumerate() {
            if p == q {
                return i as usize + 1; // 1-indexed result
            }
        }

        panic!("Point not found in list: {:#?}", p);
    }
}

#[cfg(test)]
#[test]
fn check_p1_wire_pair_0() {
    let wire_pair = r#"R8,U5,L5,D3
    U7,R6,D4,L4
    "#;

    assert_eq!(p1_simple(&parse_input(wire_pair)), 6, "Failed sample #0");
}

#[cfg(test)]
#[test]
fn check_p1_wire_pair_1() {
    let wire_pair = r#"R75,D30,R83,U83,L12,D49,R71,U7,L72
    U62,R66,U55,R34,D71,R55,D58,R83"#;

    assert_eq!(p1_simple(&parse_input(wire_pair)), 159, "Failed sample #1");
}

#[cfg(test)]
#[test]
fn check_p1_wire_pair_2() {
    let wire_pair = r#"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
    U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"#;

    assert_eq!(p1_simple(&parse_input(wire_pair)), 135, "Failed sample #2");
}

fn parse_line(line: &str) -> Wire {
    let line = line.trim();
    let mut wire = Wire::new();
    let mut p = Point::xy(0, 0);

    for turn in line.split(",") {
        let dir = turn.chars().nth(0).unwrap();
        let dist: i32 = turn[1..].parse().unwrap();
        assert!(dist != 0);

        let (dx, dy) = match dir {
            'R' => (1, 0),
            'L' => (-1, 0),
            'U' => (0, 1),
            'D' => (0, -1),
            _ => {
                panic!("Unexpected direction: {}", dir);
            }
        };

        for _ in 0..dist {
            p.x += dx;
            p.y += dy;
            wire.insert(p);
        }
    }

    wire
}

#[aoc_generator(day3)]
pub fn parse_input(input: &str) -> (Wire, Wire) {
    let input = input.trim();
    let mut lines = input.lines();

    let wire1 = parse_line(lines.next().unwrap());
    let wire2 = parse_line(lines.next().unwrap());

    // Better not be a third line...
    assert!(lines.next().is_none());

    (wire1, wire2)
}

#[aoc(day3, part1)]
pub fn p1_simple(input: &(Wire, Wire)) -> i32 {
    Wire::intersection(&input.0, &input.1)
        .map(|p| p.manhattan())
        .min()
        .expect("No intersections?")
}

#[aoc(day3, part2)]
pub fn p2_simple(input: &(Wire, Wire)) -> usize {
    Wire::intersection(&input.0, &input.1)
        .map(|p| input.0.steps_for(p) + input.1.steps_for(p))
        .min()
        .expect("No intersections?")
}

#[cfg(test)]
#[test]
fn check_p2_wire_pair_0() {
    let wire_pair = r#"R8,U5,L5,D3
    U7,R6,D4,L4
    "#;

    assert_eq!(p2_simple(&parse_input(wire_pair)), 30, "Failed sample #0");
}

#[cfg(test)]
#[test]
fn check_p2_wire_pair_1() {
    let wire_pair = r#"R75,D30,R83,U83,L12,D49,R71,U7,L72
    U62,R66,U55,R34,D71,R55,D58,R83"#;

    assert_eq!(p2_simple(&parse_input(wire_pair)), 610, "Failed sample #1");
}

#[cfg(test)]
#[test]
fn check_p2_wire_pair_2() {
    let wire_pair = r#"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
    U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"#;

    assert_eq!(p2_simple(&parse_input(wire_pair)), 410, "Failed sample #2");
}
