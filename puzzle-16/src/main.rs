#![allow(dead_code)]

use std::{
    // collections::*,
    env,
    fs,
    io::{
        self,
        BufRead,
    },
    str,
};

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Copy, Clone, Debug)]
struct Instr {
    opcode: u8, // [0-15]
    a:      u8, // Input  - opcode determines whether this is a reg id or imm.
    b:      u8, // Input  - opcode determines whether this is a reg id or imm.
    c:      u8, // Output - opcode determines whether this is a reg id or imm.
}

impl str::FromStr for Instr {
    type Err = failure::Error;
    fn from_str(s: &str) -> Result<Instr, failure::Error> {
        let mut iter = s.split(' ');
        let opcode: u8 = iter.next().unwrap().parse().expect("Bad opcode");
        let a:      u8 = iter.next().unwrap().parse().expect("Bad 'a' input");
        let b:      u8 = iter.next().unwrap().parse().expect("Bad 'b' input");
        let c:      u8 = iter.next().unwrap().parse().expect("Bad 'c' output");

        Ok(Instr {
            opcode,
            a,
            b,
            c
        })
    }
}

struct UnknownOpcode {
    before: [u8; 4],
    after:  [u8; 4],
    instr:  Instr
}

impl UnknownOpcode {
    fn parse_from_input(
            before_line: &str,
            instr_line:  &str,
            after_line:  &str)
    -> UnknownOpcode {
        assert!(before_line.starts_with("Before: ["));
        let before_line = before_line.trim_left_matches("Before: ");

        assert!(after_line.starts_with("After:  ["));
        let after_line = after_line.trim_left_matches("After:  ");

        UnknownOpcode {
            before: parse_u8x4(before_line),
            after:  parse_u8x4(after_line),
            instr:  str::parse(instr_line).expect("Bad instr"),
        }
    }
}

fn parse_u8x4(s: &str) -> [u8; 4] {
    lazy_static! {
        static ref EXPR: Regex = Regex::new(r"\[(\d+), (\d+), (\d+), (\d+)\]").unwrap();
    };
    let captures = EXPR.captures(s).unwrap();

    [
        captures.get(1).expect("No capture 1").as_str().parse().expect("Bad parse 1"),
        captures.get(2).expect("No capture 2").as_str().parse().expect("Bad parse 2"),
        captures.get(3).expect("No capture 3").as_str().parse().expect("Bad parse 3"),
        captures.get(4).expect("No capture 4").as_str().parse().expect("Bad parse 4"),
    ]
}

#[derive(Copy, Clone, Debug)]
enum Opcode {
    Addr, // regs[instr.c] = regs[instr.a] + regs[instr.b];
    Addi, // regs[instr.c] = regs[instr.a] + instr.b;
    Mulr, // regs[instr.c] = regs[instr.a] * regs[instr.b];
    Muli, // regs[instr.c] = regs[instr.a] * instr.b;
    Banr, // regs[instr.c] = regs[instr.a] & regs[instr.b];
    Bani, // regs[instr.c] = regs[instr.a] & instr.b;
    Borr, // regs[instr.c] = regs[instr.a] | regs[instr.b];
    Bori, // regs[instr.c] = regs[instr.a] | instr.b;

    Setr, // regs[instr.c] = regs[instr.a]
    Seti, // regs[instr.c] = instr.a

    Gtir, // regs[instr.c] = (instr.a       > regs[instr.b] ) as u8,
    Gtri, // regs[instr.c] = (regs[instr.a] > instr.b       ) as u8,
    Gtrr, // regs[instr.c] = (regs[instr.a] > regs[instr.b] ) as u8,

    Eqir, // regs[instr.c] = (instr.a       == regs[instr.b]) as u8,
    Eqri, // regs[instr.c] = (regs[instr.a] == instr.b      ) as u8,
    Eqrr, // regs[instr.c] = (regs[instr.a] == regs[instr.b]) as u8,
}

fn exec(mut regs: [u8; 4], op: Opcode, instr: Instr) -> [u8; 4] {
    let instr_a = instr.a as usize;
    let instr_b = instr.b as usize;
    let instr_c = instr.c as usize;

    match op {
        Opcode::Setr => regs[instr_c] = regs[instr_a],
        Opcode::Seti => regs[instr_c] = instr.a,
        Opcode::Addr => regs[instr_c] = regs[instr_a]  +  regs[instr_b as usize],
        Opcode::Addi => regs[instr_c] = regs[instr_a]  +  instr.b,
        Opcode::Mulr => regs[instr_c] = regs[instr_a]  *  regs[instr_b as usize],
        Opcode::Muli => regs[instr_c] = regs[instr_a]  *  instr.b,
        Opcode::Banr => regs[instr_c] = regs[instr_a]  &  regs[instr_b as usize],
        Opcode::Bani => regs[instr_c] = regs[instr_a]  &  instr.b,
        Opcode::Borr => regs[instr_c] = regs[instr_a]  |  regs[instr_b as usize],
        Opcode::Bori => regs[instr_c] = regs[instr_a]  |  instr.b,
        Opcode::Gtir => regs[instr_c] = (instr.a       >  regs[instr_b] ) as u8,
        Opcode::Gtri => regs[instr_c] = (regs[instr_a] >  instr.b       ) as u8,
        Opcode::Gtrr => regs[instr_c] = (regs[instr_a] >  regs[instr_b] ) as u8,
        Opcode::Eqir => regs[instr_c] = (instr.a       == regs[instr_b]) as u8,
        Opcode::Eqri => regs[instr_c] = (regs[instr_a] == instr.b      ) as u8,
        Opcode::Eqrr => regs[instr_c] = (regs[instr_a] == regs[instr_b]) as u8,
    }
    regs
}


fn guess_opcode(unknown: UnknownOpcode) -> Vec<Opcode> {
    let before = unknown.before;
    let after  = unknown.after;
    let instr  = unknown.instr;

    let ops = [
        Opcode::Addr,
        Opcode::Addi,
        Opcode::Mulr,
        Opcode::Muli,
        Opcode::Banr,
        Opcode::Bani,
        Opcode::Borr,
        Opcode::Bori,
        Opcode::Setr,
        Opcode::Seti,
    ];

    let mut possible = vec![];
    for op in &ops {
        let op = *op;
        if exec(before, op, instr) == after {
            possible.push(op);
        }
    }
    possible
}

fn main() {
    let run = env::args().nth(1).unwrap_or("1".to_string());
    if run == "1" {
        match run1() {
            Ok(()) => {},
            Err(ref err) => eprintln!("{}", err),
        }
    } else if run == "2" {
        match run2() {
            Ok(()) => {},
            Err(ref err) => eprintln!("{}", err),
        }
    }
}

fn run1() -> Result<(), failure::Error> {
    let file = fs::File::open("input.txt")?;
    let input = io::BufReader::new(file);

    let lines: Vec<String> = input
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let unknowns: Vec<UnknownOpcode> = lines
        .chunks_exact(4)
        .take_while(|thing| !thing[0].is_empty() || !thing[1].is_empty())
        .map(|thing| {
            assert_eq!(thing[3], "");
            assert!(thing[0].starts_with("Before: ["));

            assert!(thing[2].starts_with("After:  ["));

            UnknownOpcode::parse_from_input( &thing[0], &thing[1], &thing[2])
        })
        .collect();
    println!("Found {} unknowns to test", unknowns.len());

    let behaves_like = guess_opcode(UnknownOpcode {
        before: [3, 2, 1, 1],
        instr: Instr { opcode: 9, a: 2, b: 1, c: 2},
        after:  [3, 2, 2, 1]
    });
    println!("Behaves like: {} opcodes:", behaves_like.len());
    for op in behaves_like {
        println!("  {:?}", op);
    }

    Ok(())
}

fn run2() -> Result<(), failure::Error> {
    let file = fs::File::open("input.txt")?;
    let _input = io::BufReader::new(file);

    Ok(())
}
