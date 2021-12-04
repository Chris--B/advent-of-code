use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone)]
pub struct BingoState {
    rng: Vec<u32>,
    boards: Vec<Board>,
}

#[derive(Clone)]
pub struct Board([[u32; 5]; 5]);

impl Board {
    fn at(&self, x: u32, y: u32) -> u32 {
        self.0[x as usize][y as usize]
    }

    fn draw(&mut self, num: u32) {
        let nums: &mut [u32; 25] = unsafe { std::mem::transmute(&mut self.0) };

        for n in nums {
            if *n == num {
                *n = 0;
            }
        }
    }

    fn score(&self) -> u32 {
        let nums: &[u32; 25] = unsafe { std::mem::transmute(&self.0) };

        nums.iter().sum()
    }

    fn has_won(&self) -> bool {
        // check each row
        for x in 0..5 {
            if (0..5).all(|y| self.at(x, y) == 0) {
                return true;
            }
        }

        // check each col
        for y in 0..5 {
            if (0..5).all(|x| self.at(x, y) == 0) {
                return true;
            }
        }

        false
    }
}

#[aoc_generator(day4)]
pub fn parse_input(input: &str) -> BingoState {
    let mut lines = input.lines();
    let rng = lines
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let mut boards = vec![];

    // Boards are 6 lines: 1 empty and 5 lines of 5 numbers each.
    while let Some(_empty) = lines.next() {
        assert_eq!(_empty, "");

        fn parse_line(line: &str) -> Vec<u32> {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        }

        let b0 = parse_line(lines.next().unwrap());
        let b1 = parse_line(lines.next().unwrap());
        let b2 = parse_line(lines.next().unwrap());
        let b3 = parse_line(lines.next().unwrap());
        let b4 = parse_line(lines.next().unwrap());

        let board = Board([
            [b0[0], b0[1], b0[2], b0[3], b0[4]],
            [b1[0], b1[1], b1[2], b1[3], b1[4]],
            [b2[0], b2[1], b2[2], b2[3], b2[4]],
            [b3[0], b3[1], b3[2], b3[3], b3[4]],
            [b4[0], b4[1], b4[2], b4[3], b4[4]],
        ]);

        boards.push(board);
    }

    BingoState { rng, boards }
}

// Part1 ======================================================================

#[aoc(day4, part1)]
#[inline(never)]
pub fn part1(input: &BingoState) -> u32 {
    // dumb
    let BingoState { rng, mut boards } = input.clone();

    for n in rng {
        for board in boards.iter_mut() {
            board.draw(n);

            if board.has_won() {
                return board.score() * n;
            }
        }
    }

    panic!("No winners?");
}

// Part2 ======================================================================
#[aoc(day4, part2)]
#[inline(never)]
pub fn part2(input: &BingoState) -> u32 {
    // dumb
    let BingoState { rng, mut boards } = input.clone();

    let mut winner = None;

    for n in rng {
        for board in boards.iter_mut().filter(|b| !b.has_won()) {
            board.draw(n);

            if board.has_won() {
                winner = Some(board.score() * n);
            }
        }
    }

    winner.unwrap()
}
