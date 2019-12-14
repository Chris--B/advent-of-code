use aoc_runner_derive::{aoc, aoc_generator};

use itertools::Itertools;

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

pub fn run_intcode(mem: &mut [i64], mut input: impl Iterator<Item = i64>, output: &mut Vec<i64>) {
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

    let mut input_count = 0;

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
                let value = input
                    .next()
                    .unwrap_or_else(|| panic!("Failed to get next input: {}", input_count));
                input_count += 1;

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

#[allow(clippy::ptr_arg)]
fn exec_amp(intcode: &Vec<i64>, phase: i64, signal: i64) -> i64 {
    let mut intcode = intcode.clone();
    let mut output: Vec<i64> = vec![];

    let input = [phase, signal];

    run_intcode(&mut intcode, input.iter().cloned(), &mut output);

    output[0]
}

#[allow(clippy::ptr_arg)]
fn exec_phase_seq(intcode: &Vec<i64>, phases: &[i64]) -> i64 {
    let mut signal = 0;

    // A
    signal = exec_amp(intcode, phases[0], signal);

    // B
    signal = exec_amp(intcode, phases[1], signal);

    // C
    signal = exec_amp(intcode, phases[2], signal);

    // D
    signal = exec_amp(intcode, phases[3], signal);

    // E
    signal = exec_amp(intcode, phases[4], signal);

    signal
}

#[cfg(test)]
#[test]
fn check_example_1_1() {
    let intcode = vec![
        3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
    ];
    let phases = [4, 3, 2, 1, 0];

    assert_eq!(exec_phase_seq(&intcode, &phases), 43210);
}

#[allow(clippy::ptr_arg)]
fn _exec_phase_seq_2(intcode: &Vec<i64>, phases: &[i64]) -> i64 {
    let mut signal = 0;

    let mut intcodes = [
        intcode.clone(),
        intcode.clone(),
        intcode.clone(),
        intcode.clone(),
        intcode.clone(),
    ];

    // A
    println!("A");
    signal = {
        let mut output: Vec<i64> = vec![];
        let input = [phases[0], signal];
        run_intcode(&mut intcodes[0], input.iter().cloned(), &mut output);
        output[0]
    };

    // B
    println!("B");
    signal = {
        let mut output: Vec<i64> = vec![];
        let input = [phases[1], signal];
        run_intcode(&mut intcodes[1], input.iter().cloned(), &mut output);
        output[0]
    };

    // C
    println!("C");
    signal = {
        let mut output: Vec<i64> = vec![];
        let input = [phases[2], signal];
        run_intcode(&mut intcodes[2], input.iter().cloned(), &mut output);
        output[0]
    };

    // D
    println!("D");
    signal = {
        let mut output: Vec<i64> = vec![];
        let input = [phases[3], signal];
        run_intcode(&mut intcodes[3], input.iter().cloned(), &mut output);
        output[0]
    };

    // E
    println!("E");
    signal = {
        let mut output: Vec<i64> = vec![];
        let input = [phases[4], signal];
        run_intcode(&mut intcodes[4], input.iter().cloned(), &mut output);
        output[0]
    };

    signal
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
pub fn p1_simple(intcode: &Vec<i64>) -> i64 {
    (0..=4)
        .permutations(5)
        .map(|p| exec_phase_seq(intcode, &p))
        .max()
        .expect("No permutations?")
}
