use crate::prelude::*;

fn parse(s: &str) -> HashMap<(i64, i64), char> {
    let mut grid = HashMap::new();

    for (y, line) in s.lines().enumerate() {
        let y = y as i64;
        for (i, c) in line.chars().enumerate() {
            let x: i64 = i as i64;
            match c {
                '0'..='9' => {
                    grid.insert((x, y), c);
                }
                '.' => { /* Do nothing */ }
                _ => {
                    grid.insert((x, y), '*');
                }
            };
        }
    }

    grid
}

fn extract_num(s: &str, x0: i64, x1: i64, y: i64, width: i64) -> i64 {
    let i0 = (x0 + y * width) as usize;
    let i1 = (x1 + y * width) as usize;

    s[i0..i1].parse().unwrap()
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

    let mut nums = vec![];

    // Walk the grid one row (x-axis) at a time
    // We don't use the expected forloop here so we an jump ahead and parse numbers
    for y in min_y..=max_y {
        let mut x = min_x;
        while x <= max_x {
            if let Some(c) = grid.get(&(x, y)) {
                // If we found a digit we should record this and try to parse a number. Everything else is ignored.
                if c.is_ascii_digit() {
                    // Walk x forward until we run out of digits.
                    // The grid is a hashmap and doesn't have a notion of "out of bounds"
                    let mut xx = x;
                    'cur_num: while let Some(c) = grid.get(&(xx, y)) {
                        if !c.is_ascii_digit() {
                            // We found something that's not a digit, so we're out of this.
                            break 'cur_num;
                        }
                        xx += 1;
                    }

                    // NOTE: Do NOT use max_x-min_x, because this uses the bounds of all non-empty cells.
                    // The example and real input both have empty columns on the far edge, and so those will be incorrect
                    // for indexing into the string.
                    //
                    // Add 1 to width here to account for '\n'
                    let width = input.lines().next().unwrap().len() as i64 + 1;
                    let num: i64 = extract_num(input, x, xx, y, width);
                    nums.push((x, y, num, xx - x));

                    x = xx;
                    continue;
                }
            }
            x += 1;
        }
    }

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

    sum
}

// Part2 ========================================================================
#[aoc(day3, part2)]
pub fn part2(input: &str) -> i64 {
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

    let mut nums = vec![];

    // Walk the grid one row (x-axis) at a time
    // We don't use the expected forloop here so we an jump ahead and parse numbers
    for y in min_y..=max_y {
        let mut x = min_x;
        while x <= max_x {
            if let Some(c) = grid.get(&(x, y)) {
                // If we found a digit we should record this and try to parse a number. Everything else is ignored.
                if c.is_ascii_digit() {
                    // Walk x forward until we run out of digits.
                    // The grid is a hashmap and doesn't have a notion of "out of bounds"
                    let mut xx = x;
                    'cur_num: while let Some(c) = grid.get(&(xx, y)) {
                        if !c.is_ascii_digit() {
                            // We found something that's not a digit, so we're out of this.
                            break 'cur_num;
                        }
                        xx += 1;
                    }

                    // NOTE: Do NOT use max_x-min_x, because this uses the bounds of all non-empty cells.
                    // The example and real input both have empty columns on the far edge, and so those will be incorrect
                    // for indexing into the string.
                    //
                    // Add 1 to width here to account for '\n'
                    let width = input.lines().next().unwrap().len() as i64 + 1;
                    let num: i64 = extract_num(input, x, xx, y, width);
                    nums.push((x, y, num, xx - x));

                    x = xx;
                    continue;
                }
            }
            x += 1;
        }
    }
    assert!(!nums.is_empty());

    let mut next_to_nums = HashMap::new();

    'nums_loop: for (x, y, num, n) in nums.iter().copied() {
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
                    next_to_nums.entry((xx, yy)).or_insert(vec![]).push(num);
                    continue 'nums_loop;
                }
            }
        }
    }

    next_to_nums
        .iter()
        .filter_map(|(xy, nums)| -> Option<i64> {
            if nums.len() == 2 {
                Some(next_to_nums[xy].iter().copied().product())
            } else {
                None
            }
        })
        .sum()
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
    #[case::adam_ex(300, "100*200")]
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

    #[rstest]
    #[case::given(467835, EXAMPLE_INPUT)]
    #[case::adam_ex(100*200, "100*200")]
    #[trace]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(input), expected);
    }
}
