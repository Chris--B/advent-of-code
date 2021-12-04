use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone)]
pub struct BingoState {
    rng: Vec<u32>,
    boards: Vec<Board>,
}

#[derive(Clone)]
pub struct Board([[u32; 5]; 5], [[bool; 5]; 5]);

impl Board {
    fn at(&self, x: u32, y: u32) -> u32 {
        self.0[x as usize][y as usize]
    }

    fn draw(&mut self, num: u32) {
        let nums: &mut [u32; 25] = unsafe { std::mem::transmute(&mut self.0) };
        let marked: &mut [bool; 25] = unsafe { std::mem::transmute(&mut self.1) };

        for (n, m) in nums.iter().zip(marked) {
            if *n == num {
                *m = true;
            }
        }
    }

    fn marked(&self, x: u32, y: u32) -> bool {
        self.1[x as usize][y as usize]
    }

    fn score(&self) -> u32 {
        // i do what I want
        let nums: [u32; 25] = unsafe { std::mem::transmute(self.0) };
        let marked: [bool; 25] = unsafe { std::mem::transmute(self.1) };

        (0..25).filter(|i| !marked[*i]).map(|i| nums[i]).sum()
    }

    fn has_won(&self) -> bool {
        // check each row
        for x in 0..5 {
            let won = (0..5).all(|y| self.marked(x, y));

            if won {
                return true;
            }
        }

        // check each row
        for y in 0..5 {
            let won = (0..5).all(|x| self.marked(x, y));

            if won {
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

    while let Some(_empty) = lines.next() {
        assert_eq!(_empty, "");

        let b0: Vec<u32> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let b1: Vec<u32> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let b2: Vec<u32> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let b3: Vec<u32> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let b4: Vec<u32> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        let board = Board(
            [
                [b0[0], b0[1], b0[2], b0[3], b0[4]],
                [b1[0], b1[1], b1[2], b1[3], b1[4]],
                [b2[0], b2[1], b2[2], b2[3], b2[4]],
                [b3[0], b3[1], b3[2], b3[3], b3[4]],
                [b4[0], b4[1], b4[2], b4[3], b4[4]],
            ],
            Default::default(),
        );

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
                dbg!(board.score());
                dbg!(n);

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
                dbg!(board.score());
                dbg!(n);

                winner = Some(board.score() * n);
            }
        }
    }

    winner.unwrap()
}
