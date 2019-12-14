use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
pub fn parse_intcode(input: &str) -> Vec<i64> {
    input
        .split(',')
        .map(|line| line.trim().parse::<i64>().unwrap())
        .collect()
}

#[aoc(day2, part1)]
pub fn p1_simple(input: &[i64]) -> i64 {
    let mut mem = vec![0; input.len()];
    mem.copy_from_slice(input);

    mem[1] = 12;
    mem[2] = 2;

    run_intcode(&mut mem);

    mem[0]
}

fn run_intcode(mem: &mut [i64]) {
    const OP_ADD: i64 = 1;
    const OP_MUL: i64 = 2;
    const OP_HLT: i64 = 99;

    let mut ip: usize = 0;

    loop {
        let op = mem[ip];

        // Exit early if we don't need to read more data (it may not be there)
        if op == OP_HLT {
            break;
        }

        // Load indices of data
        let ia = mem[ip + 1] as usize;
        let ib = mem[ip + 2] as usize;
        let ic = mem[ip + 3] as usize;

        // Load data
        let ra = mem[ia];
        let rb = mem[ib];
        let rc = &mut mem[ic];

        match op {
            OP_ADD => {
                *rc = ra + rb;
            }
            OP_MUL => {
                *rc = ra * rb;
            }
            _ => panic!("Invalid opcode at position {}: {}", ip, op),
        }

        ip += 4;
    }
}

#[cfg(test)]
#[test]
fn check_intcode_runner() {
    let mut prog1 = [1, 0, 0, 0, 99];
    run_intcode(&mut prog1);
    assert_eq!(prog1, [2, 0, 0, 0, 99]);
}

#[allow(clippy::inconsistent_digit_grouping)]
const MOON_LANDING: i64 = 1969_07_20;

#[aoc(day2, part2, bruteforce)]
pub fn p2_simple(input: &[i64]) -> i64 {
    let mut mem = vec![0; input.len()];

    for noun in 0..=99 {
        for verb in 0..=99 {
            mem.copy_from_slice(input);

            mem[1] = noun;
            mem[2] = verb;

            run_intcode(&mut mem);

            if mem[0] == MOON_LANDING {
                return 100 * noun + verb;
            }
        }
    }

    panic!("No noun/verb pair found")
}

#[aoc(day2, part2, analytic)]
pub fn p2_analytic(input: &[i64]) -> i64 {
    // It can be observed that this program acts as a linear function of
    // its `noun` and `verb` inputs and an unknown constant:
    //
    //   X * n + Y * v + b == VALUE
    //

    fn run_sim(noun: i64, verb: i64, input: &[i64]) -> i64 {
        let mut mem = vec![0; input.len()];
        mem.copy_from_slice(input);

        mem[1] = noun;
        mem[2] = verb;

        run_intcode(&mut mem);

        mem[0]
    }

    let b = run_sim(0, 0, input);
    let x = run_sim(1, 0, input) - b;

    // uh... math?
    let n = MOON_LANDING / x;
    let v = MOON_LANDING % x - b;

    100 * n + v
}

#[aoc(day2, part1, new_vm)]
pub fn p1_new_vm(input: &[i64]) -> i64 {
    let mut vm = intcode::vm::Vm::with_memory_from_slice(input);

    vm.mem_mut()[1] = 12;
    vm.mem_mut()[2] = 2;

    vm.run().unwrap();

    vm.mem()[0]
}

#[aoc(day2, part2, new_vm)]
pub fn p2_new_vm(input: &[i64]) -> i64 {
    let mut vm = intcode::vm::Vm::empty();

    for noun in 0..=99 {
        for verb in 0..=99 {
            vm.reset(input);

            vm.mem_mut()[1] = noun;
            vm.mem_mut()[2] = verb;

            vm.run().unwrap();
            if vm.mem()[0] == MOON_LANDING {
                return 100 * noun + verb;
            }
        }
    }

    panic!("No noun/verb pair found")
}
