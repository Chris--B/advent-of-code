#![allow(unused)]

use crate::prelude::*;

fn btn_to_str(btn: &[u8; 10]) -> String {
    let btn = btn
        .iter()
        .enumerate()
        .filter_map(|(i, n)| if *n != 0 { Some(i.to_string()) } else { None })
        .collect_vec();
    format!("({})", btn.join(","))
}

#[aoc(day10, part1)]
pub fn part1(input: &str) -> i64 {
    let input = input.as_bytes();

    // Dedicated function to remove it from the parsing nonsense
    fn solve(buttons: &[u16], goal: u16) -> i64 {
        let mut min_presses = i64::MAX;

        for mut bits in (1 << (buttons.len() - 1))..(1 << buttons.len()) {
            let mut state = 0;
            let mut presses = 0;

            'buttons: for btn in buttons {
                if bits == 0 {
                    break;
                }

                if bits & 1 != 0 {
                    state ^= btn;
                    presses += 1;

                    if state == goal {
                        min_presses = i64::min(min_presses, presses);
                        break 'buttons;
                    }
                }
                bits >>= 1;
            }
        }

        debug_assert!(min_presses != i64::MAX);
        min_presses
    }

    let mut total_presses = 0;
    let mut line_start = 0;
    for i in memchr_iter(b'\n', input).chain([input.len()]) {
        // "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"
        let mut line: &[u8] = &input[line_start..i];

        // "[.##.]"
        let mut goal = 0_u16;
        {
            let k = memchr(b']', line).unwrap_or_default();
            for kk in 1..k {
                if line[kk] == b'#' {
                    goal |= 1 << (kk - 1);
                }
            }
            line = &line[k..];
        }

        // " (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"
        let mut buttons = [0_u16; 30];
        let mut btn_idx = 0;
        {
            let mut s = 0;
            // let mut e = 0;
            for j in memchr2_iter(b'(', b')', line) {
                if line[j] == b'(' {
                    s = j;
                } else {
                    for &b in &line[s..j] {
                        if let b'0'..=b'9' = b {
                            buttons[btn_idx] |= 1 << (b - b'0');
                        }
                    }
                    btn_idx += 1;
                    // e = j;
                }
            }
            // line = &line[e + 1..];
        }

        // Solve
        total_presses += solve(&buttons[..btn_idx], goal);

        line_start = i + 1;
    }

    total_presses
}

// Part2 ========================================================================

// Dedicated function to remove it from the parsing nonsense
fn solve(buttons: &[[u8; 10]], goal: &[u16]) -> i64 {
    use microlp::ComparisonOp::*;
    use microlp::{OptimizationDirection, Problem, Variable};

    if cfg!(test) {
        println!("Buttons:");
        for btn in buttons {
            println!("  + {}", btn_to_str(btn));
        }

        let goal = goal.iter().map(|n| n.to_string()).collect_vec();
        println!("Goal: {{{goal}}}", goal = goal.join(","));
    }

    let mut problem = Problem::new(OptimizationDirection::Minimize);

    let n = goal.len();
    let mut vars: Vec<Variable> = vec![];
    for _ in 0..buttons.len() {
        vars.push(problem.add_integer_var(1., (0, 1000)));
    }

    if cfg!(test) {
        println!("Have {} vars", vars.len());
    }

    for (i, &eq_to) in goal.iter().enumerate() {
        if cfg!(test) {
            println!("  + [{i}] Constraining equal to {eq_to}");
            for button in buttons {
                println!("  + {}", btn_to_str(button));
            }
        }

        let mut expr: Vec<(Variable, f64)> = vec![];
        for (j, button) in buttons.iter().enumerate() {
            expr.push((vars[j], button[i] as f64));
        }

        problem.add_constraint(expr, Eq, eq_to as f64);
    }

    let solution = problem.solve().expect("No solution?");

    if cfg!(test) {
        println!("Solution:");
        println!("  + objective = {}", solution.objective());
        for (i, v) in vars.iter().enumerate() {
            println!("  [{i}] {}", solution[*v]);
        }
    }

    // Sanity check that our solution even works
    if cfg!(debug_assertions) {
        let mut sums = [0; 10];
        for (i, btn) in buttons.iter().enumerate() {
            let n = solution[vars[i]].round() as u16;
            for (j, b) in btn.iter().enumerate() {
                sums[j] += n * (*b as u16);
            }
        }
        assert_eq!(&sums[..goal.len()], goal);
    }

    solution.objective().round() as i64
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> i64 {
    let input = input.as_bytes();

    let mut total_presses = 0;
    let mut line_start = 0;
    for i in memchr_iter(b'\n', input).chain([input.len()]) {
        // "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"
        let mut line: &[u8] = &input[line_start..i];

        // skip "[.##.]"

        // " (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"
        let mut buttons = [[0u8; 10]; 30];
        let mut button_idx = 0;
        {
            let mut s = 0;
            let mut e = 0;
            for j in memchr2_iter(b'(', b')', line) {
                if line[j] == b'(' {
                    s = j;
                } else {
                    for &b in &line[s..j] {
                        if let b'0'..=b'9' = b {
                            buttons[button_idx][(b - b'0') as usize] = 1;
                        }
                    }
                    button_idx += 1;
                    e = j;
                }
            }

            line = &line[e + 1..];
        }

        // " {3,5,4,7}"
        let mut goal = [0_u16; 10];
        let mut goal_idx = 0;
        {
            for n in just_str(line).i64s() {
                goal[goal_idx] = n as u16;
                goal_idx += 1;
            }
        }

        // Solve
        total_presses += solve(&buttons[..button_idx], &goal[..goal_idx]);

        line_start = i + 1;
    }

    total_presses
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
    #[case::given(2+3+2, EXAMPLE_INPUT)]
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
    #[case::given(10+12+11, EXAMPLE_INPUT)]
    #[case::given_1(10, "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}")]
    #[case::given_2(12, "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}")]
    #[case::given_3(11, "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}")]
    #[case::rando_online(
        332,
        "[#....##.#.] (5,6,8) (0,2,5,6,7,9) (1,3,5,6,7,9) (1,2,7,9) (1,2,4,5,6,7,8,9) (2,8) (0,2,3,5,6,7,8,9) (0,4,5,9) (4,5,8) (4,5) (0,2,3,4,5,7,8,9) (0) (1,2,3,4,5,6,7,8) {49,233,86,233,57,297,253,271,72,271}"
    )]
    #[case::but_why(
        97,
        "[###.#####] (0,5) (0,2,3,4,5,7,8) (0,2,3,4,6,7,8) (0,1,2,3,5,7,8) (0,1,3,4,7,8) (4,6,7,8) (1,2,5,6,8) (0,6,8) (3,4,5,6) {72,32,43,50,51,56,40,62,74}"
    )]
    #[case::broke(
        74,
        "[.#.##] (0,1,3) (3,4) (0,1,2,3) (0,1,2,4) (1) (1,2,4) (1,4) {29,62,39,28,57}"
    )]
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
