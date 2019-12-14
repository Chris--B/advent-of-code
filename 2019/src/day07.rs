use aoc_runner_derive::{aoc, aoc_generator};
use intcode::vm::Vm;
use itertools::Itertools;

fn run_amp_loop(intcode: &[i64], phases: &[i64]) -> i64 {
    let mut vms = vec![Vm::with_memory_from_slice(intcode); 5];
    let mut signal = 0;

    for i in 0..5 {
        signal = {
            vms[i].add_input(phases[i]);
            vms[i].add_input(signal);
            vms[i].run().expect("amp crashed");

            *vms[i].get_output()
                .last()
                .expect("No output from amp?")
        };
    }

    signal
}

#[cfg(test)]
#[test]
fn check_example_43210() {
    let intcode = vec![
        3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
    ];
    let phases = [4, 3, 2, 1, 0];

    assert_eq!(run_amp_loop(&intcode, &phases), 43210);
}

#[cfg(test)]
#[test]
fn check_example_54321() {
    let intcode = vec![
        3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99,
        0,
    ];
    let phases = [0, 1, 2, 3, 4];

    assert_eq!(run_amp_loop(&intcode, &phases), 54321);
}

#[cfg(test)]
#[test]
fn check_example_65210() {
    let intcode = vec![
        3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1, 33,
        31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
    ];
    let phases = [1, 0, 4, 3, 2];

    assert_eq!(run_amp_loop(&intcode, &phases), 65210);
}

#[aoc_generator(day7)]
pub fn parse_intcode(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}

#[aoc(day7, part1)]
#[allow(clippy::ptr_arg)]
pub fn p1_simple(intcode: &[i64]) -> i64 {
    (0..=4)
        .permutations(5)
        .map(|p| run_amp_loop(intcode, &p))
        .max()
        .expect("No permutations?")
}
