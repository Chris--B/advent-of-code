use aoc_runner_derive::{aoc, aoc_generator};

use intcode::vm::Vm;

pub const OP_ADD: i64 = 1;
pub const OP_MUL: i64 = 2;

pub const OP_IN: i64 = 3;
pub const OP_OUT: i64 = 4;

pub const OP_JN: i64 = 5;
pub const OP_JZ: i64 = 6;
pub const OP_LT: i64 = 7;
pub const OP_EQ: i64 = 8;

pub const OP_HLT: i64 = 99;

pub const PM_POS: i64 = 0;
pub const PM_IMM: i64 = 1;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Op {
    code: i64,
    pm_arg0: i64,
    pm_arg1: i64,
    pm_arg2: i64,
}

impl Op {
    pub fn decode(num: i64) -> Op {
        let mut num = num;

        let code = num % 100;
        num /= 100;

        let pm_arg0 = num % 10;
        num /= 10;

        let pm_arg1 = num % 10;
        num /= 10;

        let pm_arg2 = num % 10;

        Op {
            code,
            pm_arg0,
            pm_arg1,
            pm_arg2,
        }
    }
}

#[cfg(test)]
#[test]
fn check_op_decode() {
    assert_eq!(
        Op::decode(1002),
        Op {
            code: OP_MUL,
            pm_arg0: PM_POS,
            pm_arg1: PM_IMM,
            pm_arg2: PM_POS,
        }
    );
}

pub fn run_intcode(mem: &mut [i64], input: &[i64], output: &mut Vec<i64>) {
    let mut ip: usize = 0;

    fn load_param(mem: &[i64], pm: i64, param: i64) -> i64 {
        match pm {
            PM_POS => {
                assert!(param >= 0);
                mem[param as usize]
            }
            PM_IMM => param,
            _ => panic!("Invalid ParamMode: {}", pm),
        }
    }

    fn write_param(mem: &mut [i64], pm: i64, param: i64, value: i64) {
        match pm {
            PM_POS => {
                assert!(param >= 0);
                mem[param as usize] = value;
            }
            PM_IMM => panic!("Cannot write to IMM ParamMode!"),
            _ => panic!("Invalid ParamMode: {}", pm),
        }
    }

    let mut input_index = 0;

    loop {
        let op = Op::decode(mem[ip]);

        match op.code {
            OP_ADD => {
                let ra = load_param(&mem, op.pm_arg0, mem[ip + 1]);
                let rb = load_param(&mem, op.pm_arg1, mem[ip + 2]);

                write_param(mem, PM_POS, mem[ip + 3], ra + rb);

                ip += 4;
            }
            OP_MUL => {
                let ra = load_param(&mem, op.pm_arg0, mem[ip + 1]);
                let rb = load_param(&mem, op.pm_arg1, mem[ip + 2]);

                write_param(mem, PM_POS, mem[ip + 3], ra * rb);

                ip += 4;
            }
            OP_IN => {
                // Read from input
                let value = input[input_index];
                input_index += 1;

                // Save to memory
                write_param(mem, op.pm_arg0, mem[ip + 1], value);

                ip += 2;
            }
            OP_OUT => {
                // Read from memoery
                let value = load_param(&mem, op.pm_arg0, mem[ip + 1]);

                // Write to output
                output.push(value);

                ip += 2;
            }
            OP_JN => {
                let pred = load_param(&mem, op.pm_arg0, mem[ip + 1]);
                let addr = load_param(&mem, op.pm_arg1, mem[ip + 2]);

                if pred != 0 {
                    assert!(addr > 0);
                    ip = addr as usize;
                } else {
                    ip += 3;
                }
            }
            OP_JZ => {
                let pred = load_param(&mem, op.pm_arg0, mem[ip + 1]);
                let addr = load_param(&mem, op.pm_arg1, mem[ip + 2]);

                if pred == 0 {
                    assert!(addr > 0);
                    ip = addr as usize;
                } else {
                    ip += 3;
                }
            }
            OP_LT => {
                let ra = load_param(&mem, op.pm_arg0, mem[ip + 1]);
                let rb = load_param(&mem, op.pm_arg1, mem[ip + 2]);

                let value = if ra < rb { 1 } else { 0 };
                write_param(mem, PM_POS, mem[ip + 3], value);

                ip += 4;
            }
            OP_EQ => {
                let ra = load_param(&mem, op.pm_arg0, mem[ip + 1]);
                let rb = load_param(&mem, op.pm_arg1, mem[ip + 2]);

                let value = if ra == rb { 1 } else { 0 };
                write_param(mem, PM_POS, mem[ip + 3], value);

                ip += 4;
            }
            OP_HLT => break,
            _ => panic!("Invalid opcode at position {}: {:?}", ip, op),
        }
    }
}

const INPUT_CODE_AC: i64 = 1;
const INPUT_CODE_TRC: i64 = 5;

#[aoc_generator(day5)]
pub fn parse_intcode(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}

#[aoc(day5, part1)]
#[allow(clippy::ptr_arg)]
pub fn p1_simple(intcode: &Vec<i64>) -> i64 {
    let mut intcode = intcode.clone();
    let input = [INPUT_CODE_AC];
    let mut output: Vec<i64> = vec![];

    run_intcode(&mut intcode, &input, &mut output);

    *output.last().expect("No output?")
}

#[aoc(day5, part1, new_vm)]
#[allow(clippy::ptr_arg)]
pub fn p1_newvm(intcode: &Vec<i64>) -> i64 {
    let mut vm = Vm::with_memory_from_slice(&intcode);
    vm.add_input(INPUT_CODE_AC);

    let why = vm.run();
    why.unwrap();

    *vm.get_output().last().expect("No output?")
}

#[aoc(day5, part1, new_vm2)]
#[allow(clippy::ptr_arg)]
pub fn part1_newvm(intcode: &Vec<i64>) -> Result<i64, intcode::cpu::VmError> {
    use intcode::cpu::*;

    let mut vm = Vm::from_code(&intcode);

    vm.input(INPUT_CODE_AC);

    match vm.run()? {
        intcode::cpu::NameMe::Output(out) => Ok(out),
        reason => panic!("{:#?}", reason),
    }
}

#[cfg(test)]
#[test]
fn check_branch_inst() {
    {
        let mut intcode = [3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut output = vec![];
        run_intcode(&mut intcode, &[9], &mut output);
        assert_eq!(output.len(), 1, "Wrong number of  output values");
        assert_eq!(output[0], 0);
    }

    {
        let mut intcode = [3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut output = vec![];
        run_intcode(&mut intcode, &[8], &mut output);
        assert_eq!(output.len(), 1, "Wrong number of  output values");
        assert_eq!(output[0], 1);
    }
}

#[aoc(day5, part2)]
#[allow(clippy::ptr_arg)]
pub fn p2_simple(intcode: &Vec<i64>) -> i64 {
    let mut intcode = intcode.clone();
    let input = [INPUT_CODE_TRC];
    let mut output: Vec<i64> = vec![];

    run_intcode(&mut intcode, &input, &mut output);

    *output.last().expect("No output?")
}

#[aoc(day5, part2, new_vm)]
#[allow(clippy::ptr_arg)]
pub fn p2_newvm(intcode: &Vec<i64>) -> i64 {
    let mut vm = Vm::with_memory_from_slice(&intcode);
    vm.add_input(INPUT_CODE_TRC);

    let why = vm.run();
    why.unwrap();

    *vm.get_output().last().expect("No output?")
}

#[aoc(day5, part2, new_vm2)]
#[allow(clippy::ptr_arg)]
pub fn part2_newvm(intcode: &Vec<i64>) -> Result<i64, intcode::cpu::VmError> {
    use intcode::cpu::*;

    let mut vm = Vm::from_code(&intcode);

    vm.input(INPUT_CODE_TRC);

    match vm.run()? {
        intcode::cpu::NameMe::Output(out) => Ok(out),
        reason => panic!("{:#?}", reason),
    }
}
