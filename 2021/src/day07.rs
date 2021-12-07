use aoc_runner_derive::aoc;

fn parse_input(input: &str) -> Vec<i64> {
    input
        .split(',')
        .map(|e| e.trim().parse().unwrap())
        .collect()
}

#[test]
fn check_example_1() {
    let input = "16,1,2,0,4,2,7,1,2,14";

    assert_eq!(part1(input), 37);
}

#[test]
fn check_example_2() {
    let input = "16,1,2,0,4,2,7,1,2,14";

    assert_eq!(part2(input), 168);
}

// Part1 ======================================================================
#[aoc(day7, part1)]
#[inline(never)]
pub fn part1(input: &str) -> i64 {
    let positions = parse_input(input);

    let mut cheapest = i64::MAX;

    for target in 0..(positions.len() as i64) {
        let mut cost = 0;

        for p in &positions {
            cost += (p - target).abs();
        }

        cheapest = cheapest.min(cost);
    }

    cheapest
}

// Part2 ======================================================================
#[aoc(day7, part2)]
#[inline(never)]
pub fn part2(input: &str) -> i64 {
    let positions = parse_input(input);

    let mut cheapest = i64::MAX;

    for target in 0..(positions.len() as i64) {
        let mut cost = 0;

        for p in &positions {
            let steps = (p - target).abs();
            cost += steps * (steps + 1) / 2;
        }

        cheapest = cheapest.min(cost);
    }

    cheapest
}
