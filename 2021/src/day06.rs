use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
pub fn parse_input(input: &str) -> Vec<u8> {
    input.split(',').map(|n| n.parse().unwrap()).collect()
}

#[test]
fn check_input_1() {
    let input = "3,4,3,1,2";
    let fish: Vec<u8> = parse_input(input);

    assert_eq!(tick_fish(fish.clone(), 18), 26);
    assert_eq!(tick_fish(fish, 80), 5934);
}

#[test]
fn check_input_2() {
    let input = "3,4,3,1,2";
    let fish: Vec<u8> = parse_input(input);

    assert_eq!(tick_fish_fast(&fish, 18), 26);
    assert_eq!(tick_fish_fast(&fish, 80), 5934);
}

#[allow(dead_code)]
fn tick_fish(mut fishes: Vec<u8>, times: u64) -> u64 {
    fishes.reserve(fishes.len() * 1024);

    for t in 0..times {
        let count = fishes.len();
        for i in 0..count {
            if fishes[i] != 0 {
                fishes[i] -= 1;
            } else {
                fishes[i] = 6;
                fishes.push(8);
            }
        }
    }

    fishes.len() as u64
}

fn tick_fish_fast(fishes: &[u8], times: u64) -> u64 {
    // We don't care about the order, so just sort
    let mut counts = [0_u64; 9];
    for fish in fishes {
        counts[*fish as usize] += 1;
    }

    for t in 0..times {
        // age all the fish
        counts = [
            counts[1],             // age 0
            counts[2],             // age 1
            counts[3],             // age 2
            counts[4],             // age 3
            counts[5],             // age 4
            counts[6],             // age 5
            counts[7] + counts[0], // age 6, includes new parents
            counts[8],             // age 7
            counts[0],             // age 8, these are new fish
        ];
    }

    counts.into_iter().sum()
}

// Part1 ======================================================================
#[aoc(day6, part1)]
#[inline(never)]
pub fn part1(fish: &[u8]) -> u64 {
    tick_fish_fast(fish, 80)
}

// Part2 ======================================================================
#[aoc(day6, part2)]
#[inline(never)]
pub fn part2(fish: &[u8]) -> u64 {
    tick_fish_fast(fish, 256)
}
