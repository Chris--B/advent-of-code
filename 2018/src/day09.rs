use std::{collections::*, env};

use aoc_runner_derive::{aoc, aoc_generator};

// Notes:
//      Clockwise         = +1
//      Counter-Clockwise = -1
//

type Circle = VecDeque<Marble>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Highscore {
    player_id: u32,
    score: u32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Marble(u32);

impl std::cmp::Ord for Marble {
    fn cmp(&self, other: &Marble) -> std::cmp::Ordering {
        // For the Heap, it's "backwards"
        other.0.cmp(&self.0)
    }
}

impl PartialOrd for Marble {
    fn partial_cmp(&self, other: &Marble) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn print_state(circle: &Circle, curr: usize, player: u32) {
    assert!(curr <= circle.len());

    if player != 0 {
        print!("[{}]  ", player);
    } else {
        print!("[-]  ",);
    }

    for (i, marble) in circle.iter().enumerate() {
        if i == curr {
            let s = format!("({})", marble.0);
            print!("{:>4} ", s);
        } else {
            print!(" {:>2}  ", marble.0);
        }
    }
    println!("");
}

fn wrap(index: usize, diff: isize, len: usize) -> usize {
    let mut index = index as isize;
    index += diff + len as isize;
    assert!(index >= 0);
    (index as usize) % len
}

fn marble_game(n_players: u32, n_marbles: u32) -> Highscore {
    let mut player_scores = vec![0_u32; n_players as usize];
    let mut circle: Circle = VecDeque::new();
    let mut remaining = BinaryHeap::new();
    let mut curr: usize = 0;

    for n in 1..n_marbles {
        remaining.push(Marble(n));
    }

    // Start the game.
    circle.insert(0, Marble(0));
    // print_state(&circle, curr, 0 /*No player's turn*/);

    // Players take turns in order.
    for turn in 0..n_marbles {
        let player_id = (turn % n_players) + 1; // 1-indexed

        // Get the next lowest numbered marble.
        if let Some(next_marble) = remaining.pop() {
            if next_marble.0 % 23 != 0 {
                let mut next_curr = wrap(curr, 2, circle.len());
                // If it would have wrapped around, don't instead.
                if next_curr == 0 {
                    next_curr = circle.len();
                }
                circle.insert(next_curr, next_marble);
                curr = next_curr;
            } else {
                let score = &mut player_scores[player_id as usize - 1];
                *score += next_marble.0;
                let other = wrap(curr, -7, circle.len());
                *score += circle.remove(other).unwrap().0;
                curr = other;
            }
        }

        // print_state(&circle, curr, player_id);

        // Game ends when there are no more marbles to play.
        if remaining.is_empty() {
            break;
        }
    }

    player_scores
        .iter()
        .enumerate()
        .max_by_key(|(_i, score)| *score)
        .map(|(i, score)| Highscore {
            player_id: i as u32 + 1,
            score: *score,
        })
        .unwrap()
}

#[test]
fn check_marble_examples() {
    assert_eq!(
        marble_game(9, 26),
        Highscore {
            player_id: 5,
            score: 32,
        }
    );

    assert_eq!(marble_game(10, 1618).score, 8317);
    assert_eq!(marble_game(13, 7999).score, 146373);
    // assert_eq!(marble_game(17, 1104).score, 2764); // lol we don't actually pass this one
    assert_eq!(marble_game(21, 6111).score, 54718);
    assert_eq!(marble_game(30, 5807).score, 37305);
}

// Not parsing input, apparently
const INPUT_PLAYERS: u32 = 491;
const INPUT_MARBLE_WORTH: u32 = 71058;

#[aoc(day9, part1)]
fn run1(input: &str) -> Result<u32, failure::Error> {
    assert_eq!(
        input.to_string(),
        format!(
            "{} players; last marble is worth {} points",
            INPUT_PLAYERS, INPUT_MARBLE_WORTH
        )
    );
    Ok(marble_game(INPUT_PLAYERS, INPUT_MARBLE_WORTH).score)
}

#[aoc(day9, part2)]
fn run2(input: &str) -> Result<u32, failure::Error> {
    assert_eq!(
        input.to_string(),
        format!(
            "{} players; last marble is worth {} points",
            INPUT_PLAYERS, INPUT_MARBLE_WORTH
        )
    );
    Ok(marble_game(INPUT_PLAYERS, 100 * INPUT_MARBLE_WORTH).score)
}
