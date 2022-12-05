use aoc_runner_derive::aoc;

#[derive(Copy, Clone, Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

#[derive(Debug)]
struct State {
    stacks: Vec<Vec<char>>,
    moves: Vec<Move>,
}

fn parse(input: &str) -> State {
    let crate_lines: Vec<&str> = input
        .lines()
        .filter(|l| l.trim().starts_with('['))
        .rev()
        .collect();

    let moves: Vec<Move> = input
        .lines()
        .filter(|line| line.trim().starts_with("move"))
        .map(|line| {
            let mut parts = line.split_whitespace();

            let _ = parts.next(); // "move"
            let count = parts.next().unwrap().parse().unwrap();

            let _ = parts.next(); // "from"
            let from = parts.next().unwrap().parse().unwrap();

            let _ = parts.next(); // "to"
            let to = parts.next().unwrap().parse().unwrap();

            Move { count, from, to }
        })
        .collect();

    let mut stacks = vec![vec![]; crate_lines.len() + 1];

    for line in crate_lines {
        for (i, c) in line.as_bytes().chunks(4).enumerate() {
            let c = c[1] as char;

            if c != ' ' {
                stacks[i].push(c);
            }
        }
    }

    State { stacks, moves }
}

// Part1 ========================================================================
#[aoc(day5, part1)]
#[inline(never)]
pub fn part1(input: &str) -> String {
    let mut state = parse(input);

    for Move { count, from, to } in state.moves {
        debug_assert!(state.stacks[from - 1].len() >= count);

        for _ in 0..count {
            let c = state.stacks[from - 1].pop().unwrap();
            state.stacks[to - 1].push(c);
        }
    }

    state
        .stacks
        .iter()
        .filter_map(|stack| stack.last())
        .collect()
}

// Part2 ========================================================================
#[aoc(day5, part2)]
#[inline(never)]
pub fn part2(input: &str) -> String {
    let mut state = parse(input);

    let mut tmp = vec![];
    for Move { count, from, to } in state.moves {
        debug_assert!(state.stacks[from - 1].len() >= count);

        for _ in 0..count {
            let c = state.stacks[from - 1].pop().unwrap();
            tmp.push(c);
        }
        tmp.reverse();
        state.stacks[to - 1].extend_from_slice(&tmp);
        tmp.clear();
    }

    state
        .stacks
        .iter()
        .filter_map(|stack| stack.last())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[rstest]
    #[case::given("CMZ", EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1)]
        p: impl FnOnce(&str) -> String,
        #[case] expected: String,
        #[case] input: &str,
    ) {
        assert_eq!(p(input), expected);
    }

    #[rstest]
    #[case::given("MCD", EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2)]
        p: impl FnOnce(&str) -> String,
        #[case] expected: String,
        #[case] input: &str,
    ) {
        let input = input;
        assert_eq!(p(input), expected);
    }
}
