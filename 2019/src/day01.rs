use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn parse_masses(input: &str) -> Vec<u32> {
    input
        .split_whitespace()
        .map(|line| line.trim().parse::<u32>().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn p1_simple(input: &[u32]) -> u32 {
    input.iter().cloned().map(fuel_cost_linear).sum()
}

fn fuel_cost_linear(mass: u32) -> u32 {
    (mass / 3).saturating_sub(2)
}

#[cfg(test)]
#[test]
fn check_fuel_cost_linear() {
    assert_eq!(fuel_cost_linear(0), 0);
    assert_eq!(fuel_cost_linear(2), 0);
    assert_eq!(fuel_cost_linear(12), 2);
    assert_eq!(fuel_cost_linear(100_756), 33_583);
}

fn fuel_cost(mass: u32) -> u32 {
    let mut total = 0;
    let mut next = fuel_cost_linear(mass);
    while next > 0 {
        total += next;
        next = fuel_cost_linear(next);
    }

    total
}

#[cfg(test)]
#[test]
fn check_fuel_cost() {
    assert_eq!(fuel_cost(0), 0);
    assert_eq!(fuel_cost(14), 2);
    assert_eq!(fuel_cost(1_969), 966);
    assert_eq!(fuel_cost(100_756), 50_346);
}

#[aoc(day1, part2, loop)]
pub fn p2_loop(input: &[u32]) -> u32 {
    input.iter().cloned().map(fuel_cost).sum()
}
