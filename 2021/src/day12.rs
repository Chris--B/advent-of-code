use aoc_runner_derive::{aoc, aoc_generator};

use std::collections::HashMap;

#[aoc_generator(day12)]
pub fn parse_input(input: &str) -> HashMap<String, Vec<String>> {
    let mut graph = HashMap::new();

    for line in input.lines() {
        let mut iter = line.split('-');
        let from = iter.next().unwrap();
        let to = iter.next().unwrap();

        // bidirectional
        graph
            .entry(from.into())
            .or_insert_with(Vec::new)
            .push(to.into());
        graph
            .entry(to.into())
            .or_insert_with(Vec::new)
            .push(from.into());
    }

    graph
}

// Part1 ======================================================================
#[aoc(day12, part1)]
#[inline(never)]
pub fn part1(graph: &HashMap<String, Vec<String>>) -> usize {
    // for (k, v) in graph.iter() {
    //     println!("'{}' -> {:?}", k, v);
    // }
    // println!();

    let mut unfinished_paths: Vec<Vec<&str>> = vec![vec!["start"]];
    let mut finished_paths: Vec<Vec<&str>> = vec![];

    while !unfinished_paths.is_empty() {
        let mut new_paths = vec![];

        for p in unfinished_paths.drain(..) {
            let curr: &str = p.last().unwrap();

            for next in graph[curr].iter() {
                let next: &str = next;

                // Only visit this cave if
                // (1) it's big (and can be visited whenever we want)
                let is_big = (b'A'..=b'Z').contains(&next.as_bytes()[0]);
                // OR

                // (2) it's small and has not appeared yet
                if is_big || !p.contains(&next) {
                    let mut new_path: Vec<&str> = p.clone();
                    new_path.push(next);

                    if next == "end" {
                        finished_paths.push(new_path);
                    } else {
                        new_paths.push(new_path);
                    }
                }
            }
        }

        unfinished_paths.extend(new_paths);
    }

    finished_paths.len()
}

// Part2 ======================================================================
#[aoc(day12, part2)]
#[inline(never)]
pub fn part2(graph: &HashMap<String, Vec<String>>) -> usize {
    fn can_visit_cave(path_so_far: &[&str], next: &str) -> bool {
        // Big caves can be revisited any number of times
        if (b'A'..=b'Z').contains(&next.as_bytes()[0]) {
            return true;
        }

        // We cannot revisit "start"
        if next == "start" {
            assert_eq!(path_so_far[0], "start");
            return false;
        }

        // We can visit "end", but should never see it twice
        if next == "end" {
            assert_eq!(path_so_far.iter().find(|p| *p == &"end"), None);
            return true;
        }

        // A single small cave can be visited twice now
        // All others must only be visited once
        let mut just_smalls: Vec<_> = path_so_far
            .iter()
            .filter(|p| !(b'A'..=b'Z').contains(&p.as_bytes()[0]))
            .collect();
        just_smalls.sort();
        let mut has_doubled_small = false;
        for (a, b) in just_smalls.iter().zip(just_smalls.iter().skip(1)) {
            if a == b {
                has_doubled_small = true;
                break;
            }
        }

        let count = path_so_far.iter().filter(|p| *p == &next).count();
        if has_doubled_small {
            count < 1
        } else {
            count < 2
        }
    }

    let mut unfinished_paths: Vec<Vec<&str>> = vec![vec!["start"]];
    let mut finished_paths: Vec<Vec<&str>> = vec![];

    while !unfinished_paths.is_empty() {
        let mut new_paths = vec![];

        for p in unfinished_paths.drain(..) {
            let curr: &str = p.last().unwrap();

            for next in graph[curr].iter() {
                let next: &str = next;

                if can_visit_cave(&p, next) {
                    let mut new_path: Vec<&str> = p.clone();
                    new_path.push(next);

                    if next == "end" {
                        finished_paths.push(new_path);
                    } else {
                        new_paths.push(new_path);
                    }
                }
            }
        }

        unfinished_paths.extend(new_paths);
    }

    // for p in finished_paths.iter() {
    //     for cave in p {
    //         print!("{}", cave);
    //         if cave != &"end" {
    //             print!(",")
    //         }
    //     }
    //     println!();
    // }

    finished_paths.len()
}

#[test]
fn check_example_1() {
    let input = r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end"#;
    assert_eq!(part1(&parse_input(input)), 10);
}

#[test]
fn check_example_2() {
    let input = r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end"#;
    assert_eq!(part2(&parse_input(input)), 36);
}
