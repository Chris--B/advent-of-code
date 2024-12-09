use crate::prelude::*;

fn print_blocks(blocks: &[Option<i64>], left: usize, right: usize) {
    if cfg!(test) {
        for b in blocks {
            if let Some(b) = b {
                print!("{}", b);
            } else {
                print!(".");
            }
        }
        println!();
        for i in 0..blocks.len() {
            if i == left {
                print!("L");
            } else if i == right {
                print!("R");
            } else {
                print!(" ");
            }
        }
        println!();
        println!();
    }
}

// Part1 ========================================================================
#[aoc(day9, part1)]
pub fn part1(input: &str) -> i64 {
    debug_assert_eq!(input.lines().count(), 1);
    let disk_map = input.as_bytes();

    // Expand (this will not scale lol)
    let mut blocks: Vec<Option<i64>> = vec![];
    let mut id = 0;
    while 2 * id < input.len() {
        let file = disk_map[2 * id];
        for _ in b'0'..file {
            blocks.push(Some(id as i64));
        }

        if let Some(free) = disk_map.get(2 * id + 1) {
            for _ in b'0'..*free {
                blocks.push(None);
            }
        }

        id += 1;
    }

    let mut left = 0;
    let mut right = blocks.len() - 1;

    while left < right {
        // print_blocks(&blocks, left, right);

        // Advance left
        while left < right && blocks[left].is_some() {
            left += 1;
        }

        // Advance right
        while left < right && blocks[right].is_none() {
            right -= 1;
        }

        // Copy over free space
        while left < right && blocks[left].is_none() && blocks[right].is_some() {
            blocks.swap(left, right);
            left += 1;
            right -= 1;
        }
    }
    print_blocks(&blocks, left, right);

    blocks
        .iter()
        .enumerate()
        .map(|(pos, id)| (pos as i64) * (id.unwrap_or(0)))
        .sum()
}

// Part2 ========================================================================
#[derive(Copy, Clone, Debug)]
enum Block {
    File { id: i64, size: usize },
    Free { size: usize },
}
use Block::*;

impl Block {
    #[track_caller]
    fn file_id(self) -> i64 {
        match self {
            File { id, .. } => id,
            Free { .. } => unreachable!(),
        }
    }

    #[track_caller]
    fn file_size(self) -> usize {
        match self {
            File { size, .. } if size != 0 => size,
            Free { .. } => unreachable!(),
            _ => unreachable!("0 sized block is bad"),
        }
    }

    #[track_caller]
    fn free_size(self) -> usize {
        match self {
            File { .. } => unreachable!(),
            Free { size, .. } if size != 0 => size,
            _ => unreachable!("0 sized block is bad"),
        }
    }

    #[track_caller]
    fn any_size(self) -> usize {
        match self {
            File { size, .. } if size != 0 => size,
            Free { size, .. } if size != 0 => size,
            _ => unreachable!("0 sized block is bad"),
        }
    }
}

fn print_blocks_p2(blocks: &[Block], labels: &[(char, usize)]) {
    if cfg!(test) {
        // println!("Blocks:");
        // let num_blocks: usize = blocks.iter().map(|&b| b.any_size()).sum();
        // println!("    # meta   = {}", blocks.len());
        // println!("    # blocks = {num_blocks}");

        for &b in blocks {
            match b {
                File { id, size } => {
                    assert!(id < 10);
                    for _ in 0..size {
                        print!("{id}");
                    }
                }
                Free { size } => {
                    for _ in 0..size {
                        print!(".");
                    }
                }
            }
        }
        println!();

        if !labels.is_empty() {
            for (i, &b) in blocks.iter().enumerate() {
                let width = b.any_size();

                let mut printed = false;
                for (text, idx) in labels {
                    if i == *idx {
                        printed = true;
                        for _ in 0..width {
                            print!("{text}");
                        }
                    }
                }

                if !printed {
                    match b {
                        File { .. } => print!("b"),
                        Free { .. } => print!("."),
                    };
                    for _ in 1..width {
                        print!(" ");
                    }
                }
            }
            println!();
        }

        println!();
    }
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> i64 {
    debug_assert_eq!(input.lines().count(), 1);
    let disk_map = input.as_bytes();

    let mut blocks: Vec<Block> = vec![];
    let mut id = 0;
    while 2 * id < input.len() {
        blocks.push(File {
            id: id as i64,
            size: (disk_map[2 * id] - b'0') as usize,
        });
        if 2 * id + 1 < disk_map.len() {
            let size = (disk_map[2 * id + 1] - b'0') as usize;
            if size != 0 {
                blocks.push(Free { size });
            }
        }
        id += 1;
    }

    let mut seen_ids = vec![];

    let mut curr = blocks.len();
    while curr != 0 {
        curr -= 1;

        // Advance curr until it's at a file
        while matches!(blocks.get(curr), Some(Free { .. })) {
            curr -= 1;
        }

        // Skip already-moved files
        if seen_ids.contains(&blocks[curr].file_id()) {
            continue;
        }

        print_blocks_p2(&blocks, &[('C', curr)]);
        // println!("+ Trying to move id {}", blocks[curr].file_id());

        // Walk our free list until we find one that fits
        if let Some((target_idx, _)) = blocks.iter().enumerate().find(|(_id, &block)| match block {
            File { .. } => false,
            Free { size } => size >= blocks[curr].file_size(),
        }) {
            if target_idx > curr {
                // don't move "forward"
                continue;
            }
            // println!("Moving {curr} -> {target_idx}");
            seen_ids.push(blocks[curr].file_id());

            let new_free_size = blocks[target_idx].free_size() - blocks[curr].file_size();

            // Replace the old block with a Free
            let file = blocks[curr];
            blocks[curr] = Free {
                size: file.file_size(),
            };
            // TODO: Combine blocks[curr] with its neighbors (maybe?)

            // Insert file in the new slot
            blocks[target_idx] = file;
            if new_free_size != 0 {
                // Insert a free block after this if the file wasn't a perfect fit
                blocks.insert(
                    target_idx + 1,
                    Free {
                        size: new_free_size,
                    },
                );
                if curr > target_idx {
                    curr += 1;
                } else {
                    unreachable!();
                }
            }
        }
    }
    print_blocks_p2(&blocks, &[]);

    let mut checksum = 0;
    let mut pos = 0;
    for block in blocks {
        match block {
            File { id, size } => {
                for p in 0..size {
                    checksum += (pos + p) as i64 * id;
                }
                pos += size;
            }
            Free { size } => {
                pos += size;
                continue;
            }
        }
    }

    checksum
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"2333133121414131402";

    #[rstest]
    #[case::given(1928, EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        init_logging();

        let input = input.trim();
        assert!(
            input.chars().all(|c| c.is_ascii_digit()),
            "{input} is not a valid input"
        );
        assert_eq!(p(input), expected);
    }

    #[rstest]
    #[case::given(2858, EXAMPLE_INPUT)]
    #[trace]
    #[timeout(Duration::from_millis(750))] // so many infinite loops
    fn check_ex_part_2(
        #[notrace]
        #[values(part2)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        init_logging();

        let input = input.trim();
        assert!(
            input.chars().all(|c| c.is_ascii_digit()),
            "{input} is not a valid input"
        );
        assert_eq!(p(input), expected);
    }
}
