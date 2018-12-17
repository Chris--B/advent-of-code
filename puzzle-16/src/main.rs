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

#[derive(Copy, Clone, Debug)]
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
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

fn exec(mut r: [u8; 4], op: Opcode, instr: Instr) -> [u8; 4] {
    let a = instr.a as usize;
    let b = instr.b as usize;
    let c = instr.c as usize;

    match op {
        Opcode::Addi => r[c] = r[a]  +  b as u8,
        Opcode::Muli => r[c] = r[a]  *  b as u8,
        Opcode::Bani => r[c] = r[a]  &  b as u8,
        Opcode::Bori => r[c] = r[a]  |  b as u8,
        Opcode::Seti => r[c] = a as u8,

        Opcode::Addr => r[c] = r[a]  +  r[b],
        Opcode::Mulr => r[c] = r[a]  *  r[b],
        Opcode::Banr => r[c] = r[a]  &  r[b],
        Opcode::Borr => r[c] = r[a]  |  r[b],
        Opcode::Setr => r[c] = r[a],

        Opcode::Gtri => r[c] = if r[a] >  b as u8 { 1 } else { 0 },
        Opcode::Eqri => r[c] = if r[a] == b as u8 { 1 } else { 0 },

        Opcode::Gtir => r[c] = if a as u8 >  r[b] { 1 } else { 0 },
        Opcode::Eqir => r[c] = if a as u8 == r[b] { 1 } else { 0 },

        Opcode::Gtrr => r[c] = if r[a] >  r[b] { 1 } else { 0 },
        Opcode::Eqrr => r[c] = if r[a] == r[b] { 1 } else { 0 },
    }

   r
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
        Opcode::Gtir,
        Opcode::Gtri,
        Opcode::Gtrr,
        Opcode::Eqir,
        Opcode::Eqri,
        Opcode::Eqrr,
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
    let file = fs::File::open("input-1.txt")?;
    let input = io::BufReader::new(file);

    let starter_unknown = UnknownOpcode {
        before: [3, 2, 1, 1],
        instr:  Instr { opcode: 9, a: 2, b: 1, c: 2},
        after:  [3, 2, 2, 1]
    };
    let behaves_like = guess_opcode(starter_unknown);
    println!("{:#?} behaves like {} opcodes:",
             starter_unknown,
             behaves_like.len());
    for op in behaves_like {
        println!("  {:?}", op);
    }

    let lines: Vec<String> = input
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let unknowns: Vec<UnknownOpcode> = lines
        .chunks_exact(4)
        .map(|thing| {
            assert_eq!(thing[3], "");
            assert!(thing[0].starts_with("Before: ["));
            assert!(thing[2].starts_with("After:  ["));

            UnknownOpcode::parse_from_input( &thing[0], &thing[1], &thing[2])
        })
        .collect();

    println!("Found {} unknowns to test.", unknowns.len());
    let mut count = 0;
    for unknown in unknowns.iter() {
        let ops = guess_opcode(*unknown);
        assert_ne!(ops.len(), 0);
        if ops.len() >= 3 {
            // println!("Before: {:?}", unknown.before);
            // println!("{:?}",         unknown.instr);
            // println!("After:  {:?}", unknown.after);
            // println!("{:?}", ops);
            // println!("");

            count += 1;
        }
    }
    println!("Unknowns with 3 or more potential opcodes: {}/{}",
             count,
             unknowns.len());

    Ok(())
}

fn run2() -> Result<(), failure::Error> {
    let file = fs::File::open("input.txt")?;
    let _input = io::BufReader::new(file);

    Ok(())
}
