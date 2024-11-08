use aoc_runner_derive::{aoc, aoc_generator};
use intcode::vm::{Vm, VmStopReason};
use itertools::Itertools;

fn run_simple_amp_loop(intcode: &[i64], phases: &[i64]) -> i64 {
    let mut vms = vec![Vm::with_memory_from_slice(intcode); 5];
    let mut signal = 0;

    for i in 0..5 {
        signal = {
            vms[i].add_input(phases[i]);
            vms[i].add_input(signal);
            vms[i].run().expect("amp crashed");

            *vms[i].get_output().last().expect("No output from amp?")
        };
    }

    signal
}

#[cfg(test)]
#[test]
fn check_example_part1_43210() {
    let intcode = vec![
        3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
    ];
    let phases = [4, 3, 2, 1, 0];

    assert_eq!(run_simple_amp_loop(&intcode, &phases), 43210);
}

#[cfg(test)]
#[test]
fn check_example_part1_54321() {
    let intcode = vec![
        3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99,
        0,
    ];
    let phases = [0, 1, 2, 3, 4];

    assert_eq!(run_simple_amp_loop(&intcode, &phases), 54321);
}

#[cfg(test)]
#[test]
fn check_example_part1_65210() {
    let intcode = vec![
        3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1, 33,
        31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
    ];
    let phases = [1, 0, 4, 3, 2];

    assert_eq!(run_simple_amp_loop(&intcode, &phases), 65210);
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
pub fn part1_simple(intcode: &[i64]) -> i64 {
    (0..=4)
        .permutations(5)
        .map(|p| run_simple_amp_loop(intcode, &p))
        .max()
        .expect("No permutations?")
}

fn run_wild_amp_loop(intcode: &[i64], phases: &[i64]) -> i64 {
    let mut amps = vec![Vm::with_memory_from_slice(intcode); 5];
    let mut signals = vec![0]; // Send a single 0 for A's first signal
    let mut halted = [false; 5];

    // Send the phase as the first input for each amp
    for i in 0..5 {
        amps[i].add_input(phases[i]);
    }

    loop {
        for i in 0..5 {
            // println!();
            // dbg!(i);

            // Send all pending signals to the next amp
            for signal in &signals {
                amps[i].add_input(*signal);
            }
            signals.clear();

            match amps[i].run() {
                Err(VmStopReason::BlockedOnInput { .. }) => {
                    // Pass all output onto the next amp
                    let output = amps[i].pop_output();
                    // dbg!("not halted", &output);
                    signals.extend_from_slice(&output);
                }
                Ok(_) => {
                    // Pass all output onto the next amp
                    let output = amps[i].pop_output();
                    // dbg!("halted", &output);
                    signals.extend_from_slice(&output);

                    // And don't run again
                    halted[i] = true;
                }
                Err(reason) => panic!("amp crashed from {:#?}", reason),
            };

            assert_eq!(
                amps[i].get_unused_input(),
                &[],
                "Amp didn't use all of its input"
            );
        }

        if halted.iter().all(|h| *h) {
            break;
        }
    }

    *signals.last().expect("No final signal from amp?")
}

#[cfg(test)]
#[test]
fn check_example_part2_139629729() {
    let intcode = vec![
        3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1, 28,
        1005, 28, 6, 99, 0, 0, 5,
    ];
    let phases = [9, 8, 7, 6, 5];

    assert_eq!(run_wild_amp_loop(&intcode, &phases), 139629729);
}

#[cfg(test)]
#[test]
fn check_example_part2_18216() {
    let intcode = vec![
        3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54, -5,
        54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4, 53,
        1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
    ];
    let phases = [9, 7, 8, 5, 6];

    assert_eq!(run_wild_amp_loop(&intcode, &phases), 18216);
}

#[aoc(day7, part2)]
pub fn part2_simple(intcode: &[i64]) -> i64 {
    (5..=9)
        .permutations(5)
        .map(|p| run_wild_amp_loop(intcode, &p))
        .max()
        .expect("No permutations?")
}
