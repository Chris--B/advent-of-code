use aoc_runner_derive::{aoc, aoc_generator};
use intcode::vm::Vm;

#[aoc_generator(day9)]
pub fn parse_input(input: &str) -> Vec<i64> {
    input
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect()
}

#[aoc(day9, part1)]
pub fn part1(boost_code: &[i64]) -> i64 {
    let mut vm = Vm::with_memory_from_slice(boost_code);

    vm.add_input(1);

    let _ip = vm.run().expect("Error running BOOST");
    assert!(vm.get_output().len() == 1);

    vm.get_output()[0]
}

#[aoc(day9, part2)]
pub fn part2(boost_code: &[i64]) -> i64 {
    let mut vm = Vm::with_memory_from_slice(boost_code);

    vm.add_input(2);

    let _ip = vm.run().expect("Error running BOOST");
    assert!(vm.get_output().len() == 1);

    vm.get_output()[0]
}

#[aoc(day9, part1, new_vm2)]
pub fn part1_newvm2(boost_code: &[i64]) -> Result<i64, intcode::cpu::VmError> {
    use intcode::cpu::*;

    let mut vm = Vm::from_code(boost_code);

    vm.input(1);

    match vm.run()? {
        NameMe::Output(out) => Ok(out),
        reason => panic!("{:#?}", reason),
    }
}

#[aoc(day9, part2, new_vm2)]
pub fn part2_newvm2(boost_code: &[i64]) -> Result<i64, intcode::cpu::VmError> {
    use intcode::cpu::*;

    let mut vm = Vm::from_code(boost_code);

    vm.input(2);

    match vm.run()? {
        NameMe::Output(out) => Ok(out),
        reason => panic!("{:#?}", reason),
    }
}
