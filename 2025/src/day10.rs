#![allow(unused)]

use crate::prelude::*;

// Notes:
//  Longest lights: 10
//  Biggest button: 9
#[derive(Clone)]
struct Machine {
    id: usize,
    target: Vec<u8>,
    lights: Vec<u8>,
    buttons: Vec<Vec<usize>>,
    presses: usize,
}

impl Machine {
    fn press(&mut self, btn: usize) {
        if cfg!(test) {
            println!("[id={:>3}] Pressing btn #{btn}", self.id);
        }
        for &light in &self.buttons[btn] {
            self.lights[light] ^= 1;
        }
        self.presses += 1;
    }

    fn press_from_bits(&mut self, mut bits: u16) -> usize {
        let mut presses = 0;

        for btn in 0..self.buttons.len() {
            if bits == 0 {
                break;
            }
            if bits & 1 != 0 {
                self.press(btn);
                presses += 1;
            }
            bits >>= 1;
        }

        presses
    }

    fn ready(&self) -> bool {
        self.target == self.lights
    }

    fn dump_target(&self) {
        if cfg!(test) {
            print!(
                "Machine({id}) {r} [",
                id = self.id,
                r = if self.ready() { "+" } else { "-" }
            );
            for &l in &self.target {
                let c = ".#".as_bytes()[l as usize] as char;
                print!("{c}");
            }
            println!("] ");
        }
    }

    fn dump_state(&self) {
        if cfg!(test) {
            print!(
                "Machine({id}) {r} [",
                id = self.id,
                r = if self.ready() { "+" } else { "-" }
            );
            for &l in &self.lights {
                let c = ".#".as_bytes()[l as usize] as char;
                print!("{c}");
            }

            print!("] ");
            if self.presses == 1 {
                print!("{} press", self.presses);
            } else {
                print!("{} presses", self.presses);
            }
            println!();
        }
    }
}

// Part1 ========================================================================
fn min_presses_for_machine(machine: Machine) -> i64 {
    let n = (1 << (machine.buttons.len())) - 1;

    machine.dump_target();
    (0..n)
        .filter_map(|bits| {
            let mut m = machine.clone();
            let presses = m.press_from_bits(bits);
            if m.ready() {
                m.dump_state();
                // println!(
                //     "[{id:>3}] bits=0b{bits:010b} ({bits}) in {} presses",
                //     presses,
                //     id = machine.id,
                // );
                // println!();
                Some(presses)
            } else {
                None
            }
        })
        .min()
        .expect("No solution?") as i64
}

#[aoc(day10, part1)]
pub fn part1(input: &str) -> i64 {
    let mut ans = 0;

    for (id, line) in input.lines().enumerate() {
        // "the joltage requirements are irrelevant and can be safely ignored"
        let (line, _) = line.split_once('{').unwrap();
        let (target, buttons) = line.split_once(' ').unwrap();

        let target: Vec<u8> = target[1..]
            .trim_end_matches(']')
            .as_bytes()
            .iter()
            .map(|&b| match b {
                b'#' => 1,
                b'.' => 0,
                _ => unreachable!("Invalid light indicator {b}"),
            })
            .collect_vec();

        let buttons: Vec<Vec<usize>> = buttons[1..]
            .split('(')
            .map(|ids| ids.i64s().map(|id| id as usize).collect_vec())
            .collect_vec();

        // if cfg!(test) {
        //     println!("Machine {id}:");
        //     println!("  lights goal");
        //     println!("  + {target:?}");
        //     println!("  buttons");
        //     for btn in &buttons {
        //         println!("  + {btn:?}");
        //     }
        //     println!();
        // }

        let mut machine = Machine {
            id,
            lights: vec![0; target.len()],
            target,
            buttons,
            presses: 0,
        };
        // machine.dump_state();

        ans += min_presses_for_machine(machine);
    }

    ans
}

// Part2 ========================================================================
#[aoc(day10, part2)]
pub fn part2(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

    #[rstest]
    #[case::given(7, EXAMPLE_INPUT)]
    #[case::given_1(2, "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}")]
    #[case::given_2(3, "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}")]
    #[case::given_3(2, "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}")]
    #[trace]
    #[timeout(Duration::from_millis(100))]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        init_logging();

        let input = input.trim();
        assert_eq!(p(input), expected);
    }

    #[rstest]
    #[case::given(999_999, EXAMPLE_INPUT)]
    #[ignore]
    #[trace]
    #[timeout(Duration::from_millis(100))]
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
