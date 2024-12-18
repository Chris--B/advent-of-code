use crate::prelude::*;

type SmallVec<T> = smallvec::SmallVec<[T; 12]>;

const A: usize = 0;
const B: usize = 1;
const C: usize = 2;

// Part1 ========================================================================
fn concat<T: std::fmt::Display>(out: &[T]) -> String {
    out.iter().join(",")
}

#[aoc(day17, part1)]
pub fn part1(input: &str) -> String {
    let mut iter = input.i64s();
    let a = iter.next().unwrap();
    let b = iter.next().unwrap();
    let c = iter.next().unwrap();

    let program: Vec<_> = iter.map(|n| n as u8).collect_vec();
    let out = do_sim([a, b, c], &program);

    concat(&out)
}

#[aoc(day17, part1, native)]
pub fn part1_native(_: &str) -> String {
    concat(&do_sim__2_4__1_1__7_5__1_5__4_0__5_5__0_3__3_0(64854237))
}

#[allow(non_snake_case)]
fn do_sim__2_4__1_1__7_5__1_5__4_0__5_5__0_3__3_0(mut a: i64) -> SmallVec<u8> {
    let mut out: SmallVec<u8> = smallvec::smallvec![];

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
    // println!();

    for o in &mut out {
        *o ^= 5;
    }

    out
}

// Part2 ========================================================================
// #[aoc(day17, part2)]
pub fn part2(input: &str) -> i64 {
    use indicatif::ProgressBar;
    use std::cmp::Ordering;

    let prog: SmallVec<u8> = input.i64s().skip(3).map(|o| o as u8).collect();
    let mut outs: SmallVec<u8> = smallvec![];

    let mut low: i64 = 1;
    let mut high: i64 = i64::MAX;

    let mut mid = 0;
    for _i in 0.. {
        mid = (high - low) / 2 + low;

        // println!("[{_i:>2}] low ={low}");
        // println!("     mid ={mid}");
        // println!("     high={high}");

        outs = do_sim([mid, 0, 0], &prog);
        // println!("     outs=({}) {outs:?}", outs.len());
        // println!("     prog=({}) {prog:?}", prog.len());
        // println!();

        assert!(low <= high);
        match outs.len().cmp(&prog.len()) {
            Ordering::Less => low = mid + 1,
            Ordering::Greater => high = mid - 1,
            Ordering::Equal => break,
        }
    }

    // Adjust based on outputs, not just length
    match outs.cmp(&prog) {
        Ordering::Less => high = mid + 1,
        Ordering::Greater => low = mid - 1,
        Ordering::Equal => unreachable!(),
    }

    if cfg!(test) {
        assert!(low <= 117440, "low={low}");
        assert!(high >= 117440, "high={high}");
    }

    // And now we have a smaller range.
    println!(
        "Brute forcing {high} - {low} == {} A values...",
        high - low + 1
    );

    let pb = ProgressBar::new(high as u64 - low as u64 + 1);
    assert!(high - low + 1 < 1_000_000_000);
    for a in low..=high {
        pb.inc(1);
        if prog == do_sim([a, 0, 0], &prog) {
            pb.finish();
            return a;
        }
    }

    unreachable!()
}

fn do_sim(mut regs: [i64; 3], program: &[u8]) -> SmallVec<u8> {
    let mut ip = 0;
    let mut out = smallvec![];

    while ip + 1 < program.len() {
        let opcode = program[ip];
        let lit = program[ip + 1] as i64;

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

        match opcode {
            // The adv instruction (opcode 0) performs division.
            0 => regs[A] /= 1 << combo,

            // The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the instruction's literal operand, then stores the result in register B.
            1 => regs[B] ^= lit,

            // The bst instruction (opcode 2) calculates the value of its combo operand modulo 8 (thereby keeping only its lowest 3 bits), then writes that value to the B register.
            2 => regs[B] = combo % 8,

            // The jnz instruction (opcode 3) does nothing if the A register is 0.
            3 => {
                if regs[A] != 0 {
                    ip = lit as usize;
                    continue;
                }
            }

            // The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C, then stores the result in register B.
            4 => regs[B] ^= regs[C],

            // The out instruction (opcode 5) calculates the value of its combo operand modulo 8, then outputs that value.
            5 => out.push((combo % 8) as u8),

            // The bdv instruction (opcode 6) works exactly like the adv instruction except that the result is stored in the B register.
            6 => regs[B] = regs[A] / (1 << combo),

            // The cdv instruction (opcode 7) works exactly like the adv instruction except that the result is stored in the C register.
            7 => regs[C] = regs[A] / (1 << combo),

            //
            _ => unreachable!("Illegal opcode: {opcode} (ip={ip})"),
        }

        ip += 2;
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

    const _MINI_EX_1: &str = r"
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

    const _MINI_EX_4: &str = r"
Register A: 0
Register B: 29
Register C: 0

1,7
";

    const _MINI_EX_5: &str = r"
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
        for a in 0..10_000 {
            let out_jit = do_sim__2_4__1_1__7_5__1_5__4_0__5_5__0_3__3_0(a);
            let out = do_sim([a, 0, 0], &[2, 4, 1, 1, 7, 5, 1, 5, 4, 0, 5, 5, 0, 3, 3, 0]);
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

    #[rstest]
    #[case::given_117440(117440, EXAMPLE_INPUT_P2)]
    #[timeout(Duration::from_millis(100))]
    #[trace]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        init_logging();

        let input = input.trim();
        assert_eq!(p(input), expected);
    }
}
