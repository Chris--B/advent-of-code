#![allow(unused)]

use crate::prelude::*;

#[cfg(target_feature = "neon")]
use core::arch::aarch64::*;

// Part1 ========================================================================
fn fits(lock: [u8; 5], key: [u8; 5]) -> bool {
    std::iter::zip(lock, key).all(|(l, k)| (l + k) <= 5)
}

#[aoc(day25, part1)]
pub fn part1(input: &str) -> i64 {
    let mut locks: Vec<[u8; 5]> = vec![];
    let mut keys: Vec<[u8; 5]> = vec![];

    for thing in input.split("\n\n") {
        if thing.trim().lines().next() == Some("#####") {
            // locks
            locks.push([0; 5]);
            for line in thing.trim().lines().skip(1) {
                for (x, c) in line.chars().enumerate() {
                    let l = locks.len();
                    if c == '#' {
                        locks[l - 1][x] += 1;
                    }
                }
            }
        } else if thing.trim().lines().next() == Some(".....") {
            // keys
            keys.push([0; 5]);
            for line in thing.trim().lines().take(6) {
                for (x, c) in line.chars().enumerate() {
                    let l = keys.len();
                    if c == '#' {
                        keys[l - 1][x] += 1;
                    }
                }
            }
        }
    }

    let mut count = 0;

    for l in locks {
        for &k in &keys {
            if fits(l, k) {
                count += 1;
            }
        }
    }

    count
}

#[aoc(day25, part1, simd)]
#[cfg(target_feature = "neon")]
pub fn part1_simd(input: &str) -> i64 {
    let input = input.as_bytes();

    unsafe {
        type Lock = uint8x8_t;
        type Key = uint8x8_t;
        let zeros = vld1_dup_u8(&0);
        let ones: [u8; 8] = [1, 1, 1, 1, 1, 0, 0, 0];
        let ones = vld1_u8(ones.as_ptr());

        const KEY: usize = 1;
        const LOCK: usize = 0;
        let mut lockkeys: [Vec<uint8x8_t>; 2] = [vec![], vec![]];
        let mut idxs: [usize; 2] = [0, 0];

        let mut which = LOCK;
        let mut i = 0;
        let mut j = 0;
        // The last line is always either all '#'s or all '.'s. Either way, we can ignore it.
        while i + 8 < input.len() {
            if input[i] == b'\n' {
                i += 1;
                j = 0;
            }

            let row = vand_u8(ones, vld1_u8(input[i..][..8].as_ptr()));
            let set = vaddv_u8(row);
            if j == 0 {
                if set == 5 {
                    which = LOCK;
                } else if set == 0 {
                    which = KEY;
                }
                idxs[which] = lockkeys[which].len();
                lockkeys[which].push(zeros);
            }

            // Add our tally to the final lock/key
            let tally: &mut uint8x8_t = &mut lockkeys[which][idxs[which]];
            *tally = vadd_u8(*tally, row);

            i += 6;
            j += 1;
        }

        if cfg!(test) {
            println!("  + {} locks", lockkeys[LOCK].len());
            println!("  + {} keys", lockkeys[KEY].len());
            println!();

            println!("Locks:");
            for &lock in &lockkeys[LOCK] {
                println!("  +{lock:?}");
            }
            println!();

            println!("Keys:");
            for &key in &lockkeys[KEY] {
                println!("  +{key:?}");
            }
            println!();
        }

        let mut count = 0;

        // O(n^2) go brrr
        for &lock in &lockkeys[LOCK] {
            for &key in &lockkeys[KEY] {
                if cfg!(test) {
                    println!(
                        "lock: {lock:?}, key: {key:?}: {}",
                        vmaxv_u8(vadd_u8(lock, key))
                    );
                }
                if vmaxv_u8(vadd_u8(lock, key)) <= 7 {
                    count += 1;
                }
            }
        }

        count
    }
}

#[aoc(day25, part1, bits)]
pub fn part1_bits(input: &str) -> i64 {
    let input = input.as_bytes();

    const LOCK: usize = 1;
    const KEY: usize = 0;
    const LOCKKEY_LEN: usize = 6 * 7 - 1;

    let mut lockkeys: [[u32; 250]; 2] = [[0; 250]; 2];
    let mut lens: [usize; 2] = [0; 2];

    let mut i = 0;
    while i < input.len() {
        let mut v = 0;
        for j in 6..36 {
            v |= ((input[i + j] == b'#') as u32) << (j - 6);
        }

        let which = (input[i] == b'#') as usize;
        lockkeys[which][lens[which]] = v;
        lens[which] += 1;

        i += LOCKKEY_LEN + 2;
    }

    let mut count = 0;

    if cfg!(test) {
        for lock in 0..lens[LOCK] {
            for key in 0..lens[KEY] {
                count += ((lockkeys[LOCK][lock] & lockkeys[KEY][key]) == 0) as i64;
            }
        }
    } else {
        for lock in 0..250 {
            for key in 0..250 {
                count += ((lockkeys[LOCK][lock] & lockkeys[KEY][key]) == 0) as i64;
            }
        }
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    #[rstest]
    #[case::give_0(false, [0,5,3,4,3], [5,0,2,1,3])]
    #[case::give_1(false, [0,5,3,4,3], [4,3,4,0,2])]
    #[case::give_2(true, [0,5,3,4,3], [3,0,2,0,1])]
    #[case::give_3(false, [1,2,0,5,3], [5,0,2,1,3])]
    #[case::give_4(true, [1,2,0,5,3], [4,3,4,0,2])]
    #[case::give_5(true, [1,2,0,5,3], [3,0,2,0,1])]
    #[trace]
    fn check_fits(#[case] expected: bool, #[case] lock: [u8; 5], #[case] key: [u8; 5]) {
        assert_eq!(expected, fits(lock, key));
    }

    const EXAMPLE_INPUT: &str = r"
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
";

    #[rstest]
    #[case::given(3, EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1, part1_bits)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        init_logging();
        let input = input.trim();
        println!("{input}");
        println!();

        assert_eq!(p(input), expected);
    }

    #[cfg(target_feature = "neon")]
    #[rstest]
    #[case::given(3, EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_1_simd(
        #[notrace]
        #[values(part1_simd)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        init_logging();
        let input = input.trim();
        println!("{input}");
        println!();

        assert_eq!(p(input), expected);
    }
}
