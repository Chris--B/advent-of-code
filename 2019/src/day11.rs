use aoc_runner_derive::{aoc, aoc_generator};
use intcode::vm::{Atom, Vm};

#[aoc_generator(day11)]
pub fn parse_input(input: &str) -> Vec<Atom> {
    input
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect()
}

#[aoc(day11, part1)]
pub fn part1(boost_code: &[Atom]) -> Atom {
    let mut vm = Vm::with_memory_from_slice(boost_code);

    vm.add_input(1);

    let _ip = vm.run().expect("Error running BOOST");
    assert!(vm.get_output().len() == 1);

    vm.get_output()[0]
}
