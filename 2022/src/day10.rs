use crate::prelude::*;

struct Cpu {
    /// Cycle count
    reg_cc: u32,

    /// X register
    reg_x: i64,

    signals: [i64; 6],

    crt: [u8; 240],
}

impl Cpu {
    fn new() -> Self {
        Self {
            reg_cc: 0,
            reg_x: 1,
            signals: [0_i64; 6],
            crt: [0; 240],
        }
    }

    fn tick(&mut self) {
        let drawing = (self.reg_cc as i64) % 40;
        let sprite_min = self.reg_x - 1;
        let sprite_max = self.reg_x + 1;

        if sprite_min <= drawing && drawing <= sprite_max {
            self.crt[self.reg_cc as usize] = 1;
        }

        self.reg_cc += 1;

        if self.reg_cc >= 20 {
            let cc = self.reg_cc - 20;
            if cc % 40 == 0 {
                self.signals[(cc as usize / 40)] = self.signal_strength();
            }
        }
    }

    fn signal_strength(&self) -> i64 {
        self.reg_x * (self.reg_cc as i64)
    }

    fn noop(&mut self) {
        // Start nth cycle
        {
            self.tick();
            // Do nothing
        }
    }

    fn addx(&mut self, v: i64) {
        // Start nth cycle
        {
            self.tick();
            // Do nothing
        }

        // Start n+1 th cycle
        {
            self.tick();

            // Cycle is complete and X is updated
            self.reg_x += v;
            // println!("[{}] X={} (addx {v})", self.reg_cc, self.reg_x);
        }
    }
}

// Part1 ========================================================================
#[aoc(day10, part1)]
pub fn part1(input: &str) -> i64 {
    let mut cpu = Cpu::new();

    for line in input.lines() {
        match &line[..4] {
            "addx" => cpu.addx(line[5..].parse().unwrap()),
            "noop" => cpu.noop(),
            _ => unreachable!("Unrecognized instruction: {}", line),
        }

        if cpu.reg_cc >= 220 {
            break;
        }
    }

    let test_expecttd = [
        /* ( 20, X=21) == */ 420, /* ( 60, X=19) == */ 1140,
        /* (100, X=18) == */ 1800, /* (140, X=21) == */ 2940,
        /* (180, X=16) == */ 2880, /* (220, X=18) == */ 3960,
    ];
    debug_assert_eq!(cpu.signals, test_expecttd);

    cpu.signals.iter().sum()
}

// Part2 ========================================================================
#[aoc(day10, part2)]
pub fn part2(input: &str) -> SmallString<[u8; 256]> {
    let mut cpu = Cpu::new();

    for line in input.lines() {
        match &line[..4] {
            "addx" => cpu.addx(line[5..].parse().unwrap()),
            "noop" => cpu.noop(),
            _ => unreachable!("Unrecognized instruction: {}", line),
        }
    }

    let mut output = SmallString::new();

    for crt_line in cpu.crt.chunks(40) {
        output.push('\n');

        for lit in crt_line {
            let c = if *lit != 0 { '#' } else { '.' };
            output.push(c);
        }
    }
    debug_assert!(output.len() < 256);

    output
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = include_str!("../input/2022/day10_example.txt");

    #[rstest]
    #[case::given(13_140, EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(input), expected);
    }

    const EXAMPLE_OUTPUT: &str = r"
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";

    #[rstest]
    #[case::given(EXAMPLE_OUTPUT, EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2)]
        p: impl FnOnce(&str) -> SmallString<[u8; 256]>,
        #[case] expected: &str,
        #[case] input: &str,
    ) {
        let input = input.trim();
        let output = p(input);
        println!("output:\n{output}",);
        assert_eq!(output, expected);
    }
}
