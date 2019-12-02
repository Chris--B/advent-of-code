use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
pub fn parse_intcode(input: &str) -> Vec<u32> {
    input
        .split(",")
        .map(|line| line.trim().parse::<u32>().unwrap())
        .collect()
}

#[aoc(day2, part1)]
pub fn p1_simple(input: &[u32]) -> u32 {
    let mut mem = vec![0; input.len()];
    mem.copy_from_slice(input);

    mem[1] = 12;
    mem[2] = 02;

    run_intcode(&mut mem);

    mem[0]
}

fn run_intcode(mem: &mut [u32]) {
    const OP_ADD: u32 = 1;
    const OP_MUL: u32 = 2;
    const OP_HLT: u32 = 99;

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
