use crate::prelude::*;

fn parse(s: &str) -> HashMap<(i32, i32), char> {
    let mut grid = HashMap::new();

    for (y, line) in s.lines().enumerate() {
        let y = y as i32;
        for (i, c) in line.chars().enumerate() {
            let x: i32 = i as i32;
            match c {
                '0'..='9' => grid.insert((x, y), c),
                '.' => {
                    // Do nothing
                    None
                }
                _ => grid.insert((x, y), '*'),
            };
        }
    }

    grid
}

fn extract_num(s: &str, x0: i32, x1: i32, y: i32, width: i32) -> i32 {
    let i0 = (x0 + y * width) as usize;
    let i1 = (x1 + y * width) as usize;

    // dbg!("x");
    // dbg!((x0, x1, y));
    // dbg!(&s[i0..i1]);

    s[i0..i1].parse().unwrap_or_default()
}

// Part1 ========================================================================
#[aoc(day3, part1)]
pub fn part1(input: &str) -> i64 {
    let grid = parse(input);

    let (min_x, max_x) = grid
        .keys()
        .copied()
        .map(|(x, _y)| x)
        .minmax()
        .into_option()
        .unwrap();
    let (min_y, max_y) = grid
        .keys()
        .copied()
        .map(|(_x, y)| y)
        .minmax()
        .into_option()
        .unwrap();

    dbg!((min_x, max_x));
    dbg!((min_y, max_y));

    let mut nums = vec![];

    for y in min_y..=max_y {
        let mut x = min_x;
        while x <= max_x {
            if let Some(c) = grid.get(&(x, y)) {
                if c.is_ascii_digit() {
                    let mut xx = x;
                    while let Some(c) = grid.get(&(xx, y)) {
                        if !c.is_ascii_digit() {
                            break;
                        }
                        xx += 1;
                    }

                    let num: i32 = extract_num(input, x, xx, y, if cfg!(test) { 11 } else { 141 });
                    nums.push((x, y, num, xx - x));

                    x = xx;
                    continue;
                }
            }
            x += 1;
        }
    }
    assert!(!nums.is_empty());

    let mut sum = 0;

    'nums_loop: for (x, y, num, n) in nums {
        let mut to_check = HashSet::new();
        for x in x..(x + n) {
            for dy in [-1, 0, 1] {
                for dx in [-1, 0, 1] {
                    to_check.insert((x + dx, y + dy));
                }
            }
        }

        for (xx, yy) in to_check {
            if let Some(c) = grid.get(&(xx, yy)) {
                if !c.is_ascii_digit() {
                    sum += num;
                    continue 'nums_loop;
                }
            }
        }
    }

    sum as i64
}

// Part2 ========================================================================
#[aoc(day3, part2)]
pub fn part2(_input: &str) -> i64 {
    unimplemented!();
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

    #[rstest]
    #[case::given(4361, EXAMPLE_INPUT)]
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

    // #[rstest]
    // #[case::given(999_999, EXAMPLE_INPUT)]
    // #[trace]
    // fn check_ex_part_2(
    //     #[notrace]
    //     #[values(part2)]
    //     p: impl FnOnce(&str) -> i64,
    //     #[case] expected: i64,
    //     #[case] input: &str,
    // ) {
    //     let input = input.trim();
    //     assert_eq!(p(input), expected);
    // }
}
