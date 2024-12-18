#![allow(unused)]

use crate::prelude::*;

const A: usize = 0;
const B: usize = 1;
const C: usize = 2;

#[derive(Copy, Clone, Debug)]
struct Opts {
    break_after: Option<usize>,
    verbose: bool,
    jnz_verbose: bool,
}
impl Opts {
    fn none() -> Self {
        Opts {
            break_after: None,
            verbose: false,
            jnz_verbose: false,
        }
    }
}

// Part1 ========================================================================
fn concat<T: std::fmt::Display>(out: &[T]) -> String {
    out.iter().join(",")
}

#[aoc(day17, part1)]
pub fn part1(input: &str) -> String {
    let mut iter = input.i64s();
    let mut reg_a = iter.next().unwrap();
    let mut reg_b = iter.next().unwrap();
    let mut reg_c = iter.next().unwrap();

    let program: Vec<_> = iter.collect_vec();
    let mut ip = 0;
    let mut out: Vec<i64> = vec![];

    while ip + 1 < program.len() {
        let opcode = program[ip];
        let lit = program[ip + 1];

        // println!("{regs:?}, ip={ip}", regs = [reg_a, reg_b, reg_c]);
        let combo = match lit {
            0..=3 => lit,
            4 => reg_a,
            5 => reg_b,
            6 => reg_c,
            // reserved:
            7 => {
                debug_assert!(
                    [0, 2, 5, 6, 7].contains(&opcode),
                    "Reserved combo operand used",
                );
                lit
            }
            _ => unreachable!("Illegal combo operand"),
        };

        match opcode {
            // The adv instruction (opcode 0) performs division.
            // The numerator is the value in the A register.
            // The denominator is found by raising 2 to the power of the instruction's combo operand.
            // (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.)
            // The result of the division operation is truncated to an integer and then written to the A register.
            0 => {
                // println!("[{ip:>2}] adv {combo}");
                reg_a /= (1 << combo);
            }

            // The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the instruction's literal operand, then stores the result in register B.
            1 => {
                // println!("[{ip:>2}] bxl {lit}");
                reg_b ^= lit;
            }

            // The bst instruction (opcode 2) calculates the value of its combo operand modulo 8 (thereby keeping only its lowest 3 bits), then writes that value to the B register.
            2 => {
                // println!("[{ip:>2}] bst {combo}");
                reg_b = combo % 8;
            }

            // The jnz instruction (opcode 3) does nothing if the A register is 0.
            // However, if the A register is not zero, it jumps by setting the instruction pointer to the value of its literal operand;
            // if this instruction jumps, the instruction pointer is not increased by 2 after this instruction.
            3 => {
                // println!("[{ip:>2}] jnz {lit}");
                if reg_a != 0 {
                    ip = lit as usize;
                    continue;
                }
            }

            // The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C, then stores the result in register B.
            // (For legacy reasons, this instruction reads an operand but ignores it.)
            4 => {
                // println!("[{ip:>2}] bxc");
                reg_b ^= reg_c;
            }

            // The out instruction (opcode 5) calculates the value of its combo operand modulo 8, then outputs that value.
            // (If a program outputs multiple values, they are separated by commas.)
            5 => {
                // println!("[{ip:>2}] out {combo} ({})", combo % 8);
                out.push(combo % 8);
            }

            // The bdv instruction (opcode 6) works exactly like the adv instruction except that the result is stored in the B register.
            // (The numerator is still read from the A register.)
            6 => {
                // println!("[{ip:>2}] bdv {combo}");
                reg_b = reg_a / (1 << combo);
            }

            // The cdv instruction (opcode 7) works exactly like the adv instruction except that the result is stored in the C register.
            // (The numerator is still read from the A register.)
            7 => {
                // println!("[{ip:>2}] cdv {combo}");
                reg_c = reg_a / (1 << combo);
            }

            //
            _ => unreachable!("Illegal opcode: {opcode} (ip={ip})"),
        }

        ip += 2;
    }

    if cfg!(test) {
        println!("{regs:?}, ip={ip}", regs = [reg_a, reg_b, reg_c]);
        println!("out={out:?}");
    }

    concat(&out)
}

#[aoc(day17, part1, native)]
pub fn part1_native(input: &str) -> String {
    let a = input.i64s().next().unwrap();

    let mut opts = Opts::none();
    opts.verbose = true;
    concat(&do_sim__2_4__1_1__7_5__1_5__4_0__5_5__0_3__3_0(a))
}

#[allow(non_snake_case)]
fn do_sim__2_4__1_1__7_5__1_5__4_0__5_5__0_3__3_0(mut a: i64) -> SmallVec<[u8; 12]> {
    // println!("regs[A] % 64 == {a:2} 0b{a:06b}", a = a & 0b111_111);
    let mut out: SmallVec<[u8; 12]> = smallvec::smallvec![];

    loop {
        // println!("regs[A] % 64 == {a:2} 0b{a:06b}", a = a & 0b111_111);

        let b = ((a ^ 0b01) % 8) as u8;
        let c = ((a >> b) % 8) as u8;

        out.push(b ^ c);

        a >>= 3;
        if a == 0 {
            break;
        }
    }

    for o in &mut out {
        *o ^= 5;
    }

    // println!();

    out
}

// Part2 ========================================================================
fn p8(n: u8) -> i64 {
    let mut x = 1;
    for _ in 0..n {
        x *= 8;
    }
    x
}

// #[aoc(day17, part2)]
pub fn part2(input: &str) -> i64 {
    let mut iter = input.i64s().skip(3).map(|p| p as u8);
    let mut program: Vec<_> = iter.collect_vec();
    println!("program={program:?}");
    assert_eq!(program, [2, 4, 1, 1, 7, 5, 1, 5, 4, 0, 5, 5, 0, 3, 3, 0]);

    program.reverse();

    for a in 0..64 {
        let outs = do_sim__2_4__1_1__7_5__1_5__4_0__5_5__0_3__3_0(a);
        if program.starts_with(&outs) {
            println!("[{a:>4} {a:014b}] outs={outs:?}");
        }
    }

    // Check all 1-digit outs
    for a in 0..8 {
        let outs = do_sim__2_4__1_1__7_5__1_5__4_0__5_5__0_3__3_0(a);
        println!("  + a={a} ({a:03b}) -> {outs:?}");
    }

    let mut a: i64 = 4;
    let mut t: u8 = 3;
    for _ in 0..5 {
        {
            let mut candidates = [0; 8];

            // Find 3-bit b and c s.t.:
            //      b ^ c ^ 5 == t mod 8
            for b in 0..8 {
                let c = t ^ b ^ 5;

                // let b = (a ^ 1);
                let ab = b ^ 1;
                // let c = (a >> b);
                let ac = c << b;

                let a = ab | ac;
                candidates[b as usize] = a;

                // println!("b =0b{b:06b}");
                // println!("c =0b{c:06b}");
                // println!("ab=0b{ab:06b}");
                // println!("ac=0b{ac:06b}");
                // println!("a =0b{a:06b} ({a})",);
                // println!();
            }
            println!("candidates={candidates:?}");
            candidates.sort();
            a |= (candidates[0] as i64);
            a *= 8;
        }

        let outs = do_sim__2_4__1_1__7_5__1_5__4_0__5_5__0_3__3_0(a);
        println!("  + a={a} ({a:03b})");
        println!("    -> {outs:?}");
        println!("    program={program:?}");
        println!();
        t = program[outs.len()];
    }

    // for a in 0..64 * 64 * 64 {
    //     let outs = do_sim__2_4__1_1__7_5__1_5__4_0__5_5__0_3__3_0(a);
    //     if program.starts_with(&outs) {
    //         print!("[{a:>4} {a:014b}] outs={outs:?}");
    //         println!(" *");
    //     } else {
    //         // println!();
    //     }
    // }
    // println!("program={program:?}");
    // println!("outs={outs:?}");

    unreachable!();
    /*
        Import Observations:
            1. B and C are 0, A is the only initialized register
            2. Small values of a make shorter lists
            3. As A gets bigger, the list gets longer at power of 8s
                 7 -> [3]
                 8 -> [0, 4]
                63 -> [3, 3]
                64 -> [4, 0, 4]
            4. These numbers are the first 1, 2, and 3 digits of program:
                      6
                     14
                    332
            5. The full answer is probably ~14 digits (input is 16 numbers, so likely related)
                A is not program encoded/decoded as base 8 (or it is and I did it wrong)
            6. I think this is some kind of LUT that you can use to build the rest of A
                    program=[2, 4, 1, 1, 7, 5, 1, 5, 4, 0, 5, 5, 0, 3, 3, 0]
                    0 -> [4]
                    1 -> [4]
                    2 -> [6]
                    3 -> [7]
                    4 -> [0]
                    5 -> [1]
                    6 -> [2]
                    7 -> [3]
                    8 -> [0, 4]
                But unsure how.
                [4] shows up twice whih is odd.
            7. For A in 8..63, outs starts with [2] 10 times, usually with mod 8 == 6?
                    14 -> [2, 4]      14 mod 8 == 6
                    22 -> [2, 6]      22 mod 8 == 6
                    30 -> [2, 7]      30 mod 8 == 6
                    34 -> [2, 0]      34 mod 8 == 2
                    38 -> [2, 0]      38 mod 8 == 6
                    46 -> [2, 1]      46 mod 8 == 6
                    53 -> [2, 2]      53 mod 8 == 5
                    54 -> [2, 2]      54 mod 8 == 6
                    61 -> [2, 3]      61 mod 8 == 5
                    62 -> [2, 3]      62 mod 8 == 6
                Also 332 (another Interesting Value)
                    332 -> [2, 4, 1] 332 mod 8 == 4
            8. Only "interesting values" < 1e9
                [   6] outs=[2]
                [  14] outs=[2, 4]
                [ 332] outs=[2, 4, 1]
                [23948989] outs=[2, 4, 1, 1, 7, 5, 1, 5, 4]
                [23949245] outs=[2, 4, 1, 1, 7, 5, 1, 5, 4]
            8b. 23949245 - 23948989 == 256
            9. Interesting Values in base 8:
                       6 == 0o        6
                      14 == 0o       16
                     332 == 0o      514
                23948989 == 0o133267275
                23949245 == 0o133267675
            10. If we can get this to 8 checks per digit, that's VERY scalable! (8*15 == 120)
            11. The example has NO partial matches between lengths 1 and a full match!
                    [     0] outs=[0]
                    [     1] outs=[0]
                    [     2] outs=[0]
                    [     3] outs=[0]
                    [     4] outs=[0]
                    [     5] outs=[0]
                    [     6] outs=[0]
                    [     7] outs=[0]
                    [117440] outs=[0, 3, 5, 4, 3, 0]
            12. I think I need to read the opcodes more closely to understand how they pick outputs...
    */
}

fn opcode_str(op: i64) -> &'static str {
    match op {
        0 => "adv",
        1 => "bxl",
        2 => "bst",
        3 => "jnz",
        4 => "bxc",
        5 => "out",
        6 => "bdv",
        7 => "cdv",
        _ => unreachable!("{op} is not a valid opcode"),
    }
}

fn do_sim(mut regs: [i64; 3], program: &[i64], opts: Opts) -> Vec<u8> {
    let mut ip = 0;
    let mut out = vec![];

    while ip + 1 < program.len() {
        let opcode = program[ip];
        let lit = program[ip + 1];

        let combo = match lit {
            0..=3 => lit,
            4 => regs[A],
            5 => regs[B],
            6 => regs[C],
            // reserved:
            7 => {
                if ![0, 2, 5, 6, 7].contains(&opcode) {
                    unreachable!("Reserved combo operand used");
                }
                lit
            }
            _ => unreachable!("Illegal combo operand"),
        };
        let combo_str = match lit {
            0 => "combo.0",
            1 => "combo.1",
            2 => "combo.2",
            3 => "combo.3",
            4 => "combo.A",
            5 => "combo.B",
            6 => "combo.C",
            7 => "combo.7",
            _ => unreachable!("Illegal combo operand"),
        };

        if opts.verbose {
            println!("{}", opcode_str(opcode));
        }
        match opcode {
            // The adv instruction (opcode 0) performs division.
            0 => {
                if opts.verbose {
                    println!("  + A = A / (1 << {combo_str}");
                }
                regs[A] /= (1 << combo);
            }

            // The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the instruction's literal operand, then stores the result in register B.
            1 => {
                if opts.verbose {
                    println!("  + B = B ^ {lit}");
                }
                regs[B] ^= lit;
            }

            // The bst instruction (opcode 2) calculates the value of its combo operand modulo 8 (thereby keeping only its lowest 3 bits), then writes that value to the B register.
            2 => {
                if opts.verbose {
                    println!("  + B = {combo_str} % 8 (={})", combo % 8);
                }
                regs[B] = combo % 8;
            }

            // The jnz instruction (opcode 3) does nothing if the A register is 0.
            3 => {
                if opts.verbose || opts.jnz_verbose {
                    println!("  + if A ({A}) != 0 goto {lit}", A = regs[A]);
                }
                if regs[A] != 0 {
                    ip = lit as usize;
                    continue;
                }
            }

            // The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C, then stores the result in register B.
            4 => {
                if opts.verbose {
                    println!("  + B = B ^ C");
                }
                regs[B] ^= regs[C];
            }

            // The out instruction (opcode 5) calculates the value of its combo operand modulo 8, then outputs that value.
            5 => {
                if opts.verbose {
                    println!("  + out {combo_str} % 8 (={})", combo % 8);
                }
                out.push((combo % 8) as u8);
                if Some(out.len()) == opts.break_after {
                    if opts.verbose {
                        println!("DEBUG: Breaking after {:?} out(s)!", opts.break_after);
                    }
                    break;
                }
            }

            // The bdv instruction (opcode 6) works exactly like the adv instruction except that the result is stored in the B register.
            6 => {
                if opts.verbose {
                    println!("  + B = A / (1 << {combo})");
                }
                regs[B] = regs[A] / (1 << combo);
            }

            // The cdv instruction (opcode 7) works exactly like the adv instruction except that the result is stored in the C register.
            7 => {
                if opts.verbose {
                    println!("  + C = A / (1 << {combo})");
                }
                regs[C] = regs[A] / (1 << combo);
            }

            //
            _ => unreachable!("Illegal opcode: {opcode} (ip={ip})"),
        }

        ip += 2;
    }

    if opts.verbose {
        println!("[{ip:>2}] {regs:?}");
        println!("out={out:?}");
    }

    out
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

    const MINI_EX_1: &str = r"
Register A: 0
Register B: 0
Register C: 9

Program: 2,6
";

    const MINI_EX_2: &str = r"
Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4
";

    const MINI_EX_3: &str = r"
Register A: 2024
Register B: 0
Register C: 0

0,1,5,4,3,0
";

    const MINI_EX_4: &str = r"
Register A: 0
Register B: 29
Register C: 0

1,7
";

    const MINI_EX_5: &str = r"
Register A: 0
Register B: 2024
Register C: 43690

4,0
";

    const EXAMPLE_INPUT_P2_ISH: &str = r"
Register A: 117440
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
";

    #[test]
    fn check_jit_sim() {
        let opts = Opts::none();
        for a in 0..10_000 {
            let out_jit = do_sim__2_4__1_1__7_5__1_5__4_0__5_5__0_3__3_0(a);
            let out = do_sim(
                [a, 0, 0],
                &[2, 4, 1, 1, 7, 5, 1, 5, 4, 0, 5, 5, 0, 3, 3, 0],
                opts,
            );
            assert_eq!(out.as_slice(), out_jit.as_slice());
        }
    }

    #[rstest]
    #[case::given(concat(&[4,6,3,5,6,3,5,2,1,0]), EXAMPLE_INPUT)]
    #[case::mini_ex_2(concat(&[0,1,2]), MINI_EX_2)]
    #[case::mini_ex_3(concat(&[4,2,5,6,7,7,7,7,3,1,0]), MINI_EX_3)]
    #[case::but_p2(concat(&[0,3,5,4,3,0]), EXAMPLE_INPUT_P2_ISH)]
    #[trace]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1)]
        p: impl FnOnce(&str) -> String,
        #[case] expected: String,
        #[case] input: &str,
    ) {
        init_logging();

        let input = input.trim();
        assert_eq!(p(input), expected);
    }

    const EXAMPLE_INPUT_P2: &str = r"
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
";

    // #[rstest]
    // #[ignore]
    // #[case::given_117440(117440, EXAMPLE_INPUT_P2)]
    // #[timeout(Duration::from_millis(100))]
    // #[trace]
    // fn check_ex_part_2(
    //     #[notrace]
    //     #[values(part2)]
    //     p: impl FnOnce(&str) -> i64,
    //     #[case] expected: i64,
    //     #[case] input: &str,
    // ) {
    //     init_logging();

    //     let input = input.trim();
    //     assert_eq!(p(input), expected);
    // }
}
