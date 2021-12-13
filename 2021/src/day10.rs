use aoc_runner_derive::aoc;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct SyntaxError {
    idx: usize,
    expected: char,
    found: char,
}

impl SyntaxError {
    fn score(self) -> u64 {
        match self.found {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => unreachable!(),
        }
    }
}

fn check_line(line: &str) -> Result<(), SyntaxError> {
    fn check_stack(stack: &mut Vec<u8>, idx: usize, found: u8) -> Result<(), SyntaxError> {
        let expected = match stack.pop().expect("empty stack?") {
            b'(' => b')',
            b'[' => b']',
            b'{' => b'}',
            b'<' => b'>',
            _ => unreachable!(),
        };

        if expected != found {
            Err(SyntaxError {
                idx,
                expected: expected as char,
                found: found as char,
            })
        } else {
            Ok(())
        }
    }

    let mut stack = vec![];

    for (idx, b) in line.as_bytes().iter().enumerate() {
        match b {
            b'(' => stack.push(b'('),
            b'[' => stack.push(b'['),
            b'{' => stack.push(b'{'),
            b'<' => stack.push(b'<'),

            b')' => check_stack(&mut stack, idx, b')')?,
            b']' => check_stack(&mut stack, idx, b']')?,
            b'}' => check_stack(&mut stack, idx, b'}')?,
            b'>' => check_stack(&mut stack, idx, b'>')?,

            _ => unreachable!(),
        }
    }

    Ok(())
}

fn finish_line(line: &str) -> String {
    let mut stack = vec![];

    for b in line.as_bytes().iter() {
        match b {
            b'(' => stack.push('('),
            b'[' => stack.push('['),
            b'{' => stack.push('{'),
            b'<' => stack.push('<'),

            b')' => {
                let c = stack.pop().unwrap();
                assert_eq!(c, '(');
            }
            b']' => {
                let c = stack.pop().unwrap();
                assert_eq!(c, '[');
            }
            b'}' => {
                let c = stack.pop().unwrap();
                assert_eq!(c, '{');
            }
            b'>' => {
                let c = stack.pop().unwrap();
                assert_eq!(c, '<');
            }

            _ => unreachable!(),
        }
    }

    stack.reverse();

    stack
        .drain(..)
        .map(|c| match c {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',

            _ => unreachable!(),
        })
        .collect()
}

fn score_line(s: &str) -> u64 {
    s.chars().fold(0, |score, c| {
        let p = match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => unreachable!(),
        };
        5 * score + p
    })
}

// Part1 ======================================================================
#[aoc(day10, part1)]
#[inline(never)]
pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(check_line)
        .filter_map(Result::err)
        .map(SyntaxError::score)
        .sum()
}

// Part2 ======================================================================
#[aoc(day10, part2)]
#[inline(never)]
pub fn part2(input: &str) -> u64 {
    let mut scores: Vec<u64> = input
        .lines()
        .filter_map(|line| {
            check_line(line)
                .ok()
                .map(|()| score_line(&finish_line(line)))
        })
        .collect();
    scores.sort_unstable();

    scores[scores.len() / 2]
}

#[test]
fn check_example_1_lines() {
    assert_eq!(
        check_line("(]"),
        Err(SyntaxError {
            expected: ')',
            found: ']',
            idx: 1
        })
    );

    assert_eq!(
        check_line("{([(<{}[<>[]}>{[]{[(<()>"),
        Err(SyntaxError {
            idx: 12,
            expected: ']',
            found: '}'
        })
    );
}

#[test]
fn check_example_1() {
    let input = r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"#;

    assert_eq!(part1(input), 26397);
}

#[test]
fn check_example_2_lines() {
    assert_eq!(score_line("])}>"), 294);
    assert_eq!(score_line("]]}}]}]}>"), 995444);
    assert_eq!(score_line("}}>}>))))"), 1480781);
    assert_eq!(score_line(")}>]})"), 5566);
    assert_eq!(score_line("}}]])})]"), 288957);

    assert_eq!(finish_line("[({(<(())[]>[[{[]{<()<>>"), "}}]])})]");
    assert_eq!(finish_line("[(()[<>])]({[<{<<[]>>("), ")}>]})");
    assert_eq!(finish_line("(((({<>}<{<{<>}{[]{[]{}"), "}}>}>))))");
    assert_eq!(finish_line("{<[[]]>}<{[{[{[]{()[[[]"), "]]}}]}]}>");
    assert_eq!(finish_line("<{([{{}}[<[[[<>{}]]]>[]]"), "])}>");
}

#[test]
fn check_example_2() {
    let input = r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"#;

    assert_eq!(part2(input), 288957);
}
