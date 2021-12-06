use aoc_runner_derive::aoc;

pub fn parse_input(input: &str) -> [u64; 9] {
    // We don't care about the order, so just sort
    let mut counts = [0_u64; 9];

    for s in input.split(',') {
        let n: u8 = s.parse().unwrap();

        counts[n as usize] += 1;
    }

    counts
}

pub fn parse_input_clever(input: &str) -> [u64; 9] {
    let bs = input.as_bytes();

    // We don't care about the order, so just sort
    let mut counts = [0_u64; 9];

    for b in bs {
        if b'0' <= *b && *b <= b'6' {
            let n = b - b'0';
            counts[n as usize] += 1;
        }
    }

    counts
}

#[test]
fn check_input() {
    let input = "3,4,3,1,2";
    let counts = parse_input(input);

    assert_eq!(sim_fish_population(counts, 18), 26);
    assert_eq!(sim_fish_population(counts, 80), 5934);
    assert_eq!(sim_fish_population(counts, 256), 26_984_457_539);
}

fn sim_fish_population(mut counts: [u64; 9], times: u64) -> u64 {
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

fn sim_fish_population_reindex(mut counts: [u64; 9], times: usize) -> u64 {
    const CYCLE: usize = 7;
    const LEN: usize = 9;

    for t in 0..times {
        counts[(t + CYCLE) % LEN] += counts[t % LEN];
    }

    counts.into_iter().sum()
}

// =============================================================================
#[aoc(day6, part1, simple_shuffle)]
#[inline(never)]
pub fn part1_simple_shuffle(input: &str) -> u64 {
    let counts = parse_input(input);
    sim_fish_population(counts, 80)
}

#[aoc(day6, part2, simple_shuffle)]
#[inline(never)]
pub fn part2_simple_shuffle(input: &str) -> u64 {
    let counts = parse_input(input);
    sim_fish_population(counts, 256)
}

// =============================================================================
#[aoc(day6, part1, simple_reindex)]
#[inline(never)]
pub fn part1_simple_reindex(input: &str) -> u64 {
    let counts = parse_input(input);
    sim_fish_population_reindex(counts, 80)
}

#[aoc(day6, part2, simple_reindex)]
#[inline(never)]
pub fn part2_simple_reindex(input: &str) -> u64 {
    let counts = parse_input(input);
    sim_fish_population_reindex(counts, 256)
}

// =============================================================================
#[aoc(day6, part1, clever_shuffle)]
#[inline(never)]
pub fn part1_clever_shuffle(input: &str) -> u64 {
    let counts = parse_input_clever(input);
    sim_fish_population(counts, 80)
}

#[aoc(day6, part2, clever_shuffle)]
#[inline(never)]
pub fn part2_clever_shuffle(input: &str) -> u64 {
    let counts = parse_input_clever(input);
    sim_fish_population(counts, 256)
}

// =============================================================================
#[aoc(day6, part1, clever_reindex)]
#[inline(never)]
pub fn part1_clever_reindex(input: &str) -> u64 {
    let counts = parse_input_clever(input);
    sim_fish_population_reindex(counts, 80)
}

#[aoc(day6, part2, clever_reindex)]
#[inline(never)]
pub fn part2_clever_reindex(input: &str) -> u64 {
    let counts = parse_input_clever(input);
    sim_fish_population_reindex(counts, 256)
}
