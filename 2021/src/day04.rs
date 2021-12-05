use aoc_runner_derive::aoc;

use smallvec::SmallVec;

#[derive(Clone)]
pub struct BingoState {
    rng: Vec<u32>,
    boards: Vec<Board>,
}

#[derive(Copy, Clone)]
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

fn parse_input(input: &str) -> BingoState {
    let mut lines = input.lines();
    let rng = lines
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let mut boards = Vec::with_capacity(100);

    // Boards are 6 lines: 1 empty and 5 lines of 5 numbers each.
    while let Some(_empty) = lines.next() {
        debug_assert_eq!(_empty, "");

        #[inline(always)]
        fn parse_line(line: &str) -> SmallVec<[u32; 5]> {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        }

        let b0 = parse_line(lines.next().unwrap());
        let b1 = parse_line(lines.next().unwrap());
        let b2 = parse_line(lines.next().unwrap());
        let b3 = parse_line(lines.next().unwrap());
        let b4 = parse_line(lines.next().unwrap());

        boards.push(Board([
            [b0[0], b0[1], b0[2], b0[3], b0[4]],
            [b1[0], b1[1], b1[2], b1[3], b1[4]],
            [b2[0], b2[1], b2[2], b2[3], b2[4]],
            [b3[0], b3[1], b3[2], b3[3], b3[4]],
            [b4[0], b4[1], b4[2], b4[3], b4[4]],
        ]));
    }

    BingoState { rng, boards }
}

// Part1 ======================================================================

#[aoc(day4, part1)]
#[inline(never)]
pub fn part1(input: &str) -> u32 {
    let BingoState { rng, mut boards } = parse_input(input);

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
pub fn part2(input: &str) -> u32 {
    let BingoState { rng, mut boards } = parse_input(input);

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

#[derive(Debug)]
struct Entry {
    board: u8,
    pos: u8,
    num: u8,
    next: u16,
}

struct State<'a> {
    drawings_text: &'a str,
    entries: Vec<Entry>,
    head_idx_per_number: [usize; 100],
    num_boards: u8,
}

fn parse_input_packed(input: &str) -> State {
    let mut lines = input.lines();
    let drawings_text: &str = lines.next().unwrap();

    let mut board = 0;
    let mut pos = 0;

    let mut entries = vec![];

    // linked lists per number
    let mut head_idx_per_number = [usize::MAX; 100];
    let mut tail_idx_per_number = [usize::MAX; 100];

    while let Some(_empty) = lines.next() {
        debug_assert_eq!(_empty, "");

        fn parse_board_row(
            num: u8,
            entry: Entry,
            head_idx_per_number: &mut [usize; 100],
            tail_idx_per_number: &mut [usize; 100],
            entries: &mut Vec<Entry>,
        ) {
            // Save the head for each number when we find one
            if head_idx_per_number[num as usize] == usize::MAX {
                head_idx_per_number[num as usize] = entries.len();
            }

            // Update the tail for `num` everytime we append
            let tail_idx = tail_idx_per_number[num as usize];
            if tail_idx < entries.len() {
                entries[tail_idx].next = entries.len() as u16;
            }
            tail_idx_per_number[num as usize] = entries.len();

            // append
            entries.push(entry);
        }

        for _ in 0..5 {
            for entry in lines.next().unwrap().split_whitespace() {
                let num: u8 = entry.parse().unwrap();
                let entry = Entry {
                    board,
                    pos,
                    num,
                    next: u16::MAX,
                };

                parse_board_row(
                    num,
                    entry,
                    &mut head_idx_per_number,
                    &mut tail_idx_per_number,
                    &mut entries,
                );
                pos += 1;
            }
        }

        debug_assert_eq!(pos, 25);

        board += 1;
        pos = 0;
    }

    State {
        entries,
        drawings_text,
        head_idx_per_number,
        num_boards: board,
    }
}

fn board_has_won(board: u32, pos: u32) -> bool {
    let col = pos % 5;
    let row_start = pos - col;

    let unfinished_row = 0b11111 & (!board >> row_start);
    let unfinished_col = 0x108421 & (!board >> col);

    (unfinished_row == 0) || (unfinished_col == 0)
}

#[aoc(day4, part1, askalski_bit_shenanigans)]
#[inline(never)]
pub fn part1_askalski_bit_shenanigans(input: &str) -> u32 {
    let State {
        entries,
        drawings_text,
        head_idx_per_number,
        num_boards,
    } = parse_input_packed(input);

    let mut boards = vec![0_u32; num_boards as usize];

    for drawing in drawings_text.split(',') {
        let num: u8 = drawing.parse().unwrap();

        // walk the boards list and mark them as played
        let idx = head_idx_per_number[num as usize];
        if idx == usize::MAX {
            continue;
        }

        let mut n = &entries[idx];
        loop {
            let board: &mut u32 = &mut boards[n.board as usize];

            debug_assert!(!board_has_won(*board, n.pos as u32));

            *board |= 1 << n.pos as u32;

            if board_has_won(*board, n.pos as u32) {
                let mut score = 0_u32;

                for p in 0..25 {
                    if *board & (0x1 << p) == 0 {
                        let entry = entries
                            .iter()
                            .find(|e| e.board == n.board && e.pos == p as u8)
                            .unwrap();
                        score += entry.num as u32;
                    }
                }

                return score * num as u32;
            }

            if n.next != u16::MAX {
                n = &entries[n.next as usize];
            } else {
                break;
            }
        }
    }

    panic!("No winners?");
}

#[aoc(day4, part2, askalski_bit_shenanigans)]
#[inline(never)]
pub fn part2_askalski_bit_shenanigans(input: &str) -> u32 {
    let State {
        entries,
        drawings_text,
        head_idx_per_number,
        num_boards,
    } = parse_input_packed(input);

    let mut boards = vec![0_u32; num_boards as usize];

    let mut last_winner = None;

    for drawing in drawings_text.split(',') {
        let num: u8 = drawing.parse().unwrap();

        // walk the boards list and mark them as played
        let idx = head_idx_per_number[num as usize];
        if idx == usize::MAX {
            continue;
        }

        let mut n = &entries[idx];
        loop {
            let board: &mut u32 = &mut boards[n.board as usize];

            // has not won yet
            if *board & (1 << 25) == 0 {
                *board |= 1 << n.pos as u32;

                if board_has_won(*board, n.pos as u32) {
                    last_winner = Some((n.board, *board, num));
                    *board |= 1 << 25;
                }
            }

            if n.next != u16::MAX {
                n = &entries[n.next as usize];
            } else {
                break;
            }
        }
    }

    let (board_idx, board, num) = last_winner.unwrap();

    let mut score = 0_u32;

    for p in 0..25 {
        if board & (0x1 << p) == 0 {
            let entry = entries
                .iter()
                .find(|e| e.board == board_idx && e.pos == p as u8)
                .unwrap();
            score += entry.num as u32;
        }
    }

    score * num as u32
}
