use aoc_runner_derive::{aoc, aoc_generator};

pub const OP_ADD: i32 = 1;
pub const OP_MUL: i32 = 2;
pub const OP_IN: i32 = 3;
pub const OP_OUT: i32 = 4;
pub const OP_HLT: i32 = 99;

pub const PM_POS: i32 = 0;
pub const PM_IMM: i32 = 1;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Op {
    code: i32,
    pm_arg0: i32,
    pm_arg1: i32,
    pm_arg2: i32,
}

impl Op {
    pub fn decode(num: i32) -> Op {
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

pub fn run_intcode(mem: &mut [i32], input: &[i32], output: &mut Vec<i32>) {
    let mut ip: usize = 0;

    fn load_param(mem: &[i32], pm: i32, param: i32) -> i32 {
        match pm {
            PM_POS => {
                assert!(param >= 0);
                mem[param as usize]
            }
            PM_IMM => param,
            _ => panic!("Invalid ParamMode: {}", pm),
        }
    }

    fn write_param(mem: &mut [i32], pm: i32, param: i32, value: i32) {
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
            OP_HLT => {
                break;
            }
            _ => panic!("Invalid opcode at position {}: {:?}", ip, op),
        }
    }
}

#[aoc_generator(day5)]
pub fn parse_intcode(input: &str) -> Vec<i32> {
    input
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect()
}

#[aoc(day5, part1)]
pub fn p1_simple(intcode: &Vec<i32>) -> i32 {
    let mut intcode = intcode.clone();
    let input: Vec<i32> = vec![1];
    let mut output: Vec<i32> = vec![];

    run_intcode(&mut intcode, &input, &mut output);

    *output.last().unwrap()
}
