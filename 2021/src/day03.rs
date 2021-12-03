use aoc_runner_derive::aoc;

// Part1 ======================================================================
#[aoc(day3, part1)]
#[inline(never)]
pub fn part1(input: &str) -> u64 {
    const BITS: usize = 12;

    // count bits at each position
    let mut count_of_ones = [0; BITS];
    let count_of_bits = input.lines().count();

    for (idx, count) in count_of_ones.iter_mut().enumerate() {
        for line in input.lines() {
            let bits = line.as_bytes();
            if bits[idx] == b'1' {
                *count += 1;
            }
        }
    }

    let mut gamma: u64 = 0;
    for count in count_of_ones {
        gamma <<= 1;
        if count >= count_of_bits / 2 {
            gamma |= 1;
        }
    }

    let mask = u64::MAX >> gamma.leading_zeros();
    let epsilon = !gamma & mask;

    gamma * epsilon
}

// Part2 ======================================================================
#[aoc(day3, part2)]
#[inline(never)]
pub fn part2(input: &str) -> u64 {
    let o2_reading;
    {
        let mut lines: Vec<_> = input.lines().collect();
        let bits_per_line = lines[0].len();

        'bit_loop: for bit_idx in 0..bits_per_line {
            let mut count_of_ones = 0;
            let count_of_bits = lines.len();

            for line in lines.iter() {
                let bits = line.as_bytes();
                if bits[bit_idx] == b'1' {
                    count_of_ones += 1;
                }
            }

            for line_idx in (0..lines.len()).rev() {
                let bits = lines[line_idx].as_bytes();

                let should_cull = if count_of_ones >= (count_of_bits - count_of_ones) {
                    bits[bit_idx] == b'0'
                } else {
                    bits[bit_idx] == b'1'
                };

                if should_cull {
                    let removed = lines[line_idx];

                    lines.swap_remove(line_idx);

                    if lines.len() == 1 {
                        break 'bit_loop;
                    }
                }
            }
        }

        assert!(lines.len() == 1);

        o2_reading = u64::from_str_radix(lines[0], 2).unwrap();
    }

    let co_reading;
    {
        let mut lines: Vec<_> = input.lines().collect();
        let bits_per_line = lines[0].len();

        for bit_idx in 0..bits_per_line {
            let mut count_of_ones = 0;
            let count_of_bits = lines.len();

            for line in lines.iter() {
                let bits = line.as_bytes();
                if bits[bit_idx] == b'1' {
                    count_of_ones += 1;
                }
            }

            for line_idx in (0..lines.len()).rev() {
                if lines.len() == 1 {
                    break;
                }

                let bits = lines[line_idx].as_bytes();

                let should_cull = if count_of_ones >= (count_of_bits - count_of_ones) {
                    bits[bit_idx] == b'1'
                } else {
                    bits[bit_idx] == b'0'
                };

                if should_cull {
                    lines.swap_remove(line_idx);
                }
            }
        }

        assert!(lines.len() == 1);
        co_reading = u64::from_str_radix(lines[0], 2).unwrap();
    }

    o2_reading * co_reading
}

#[test]
fn check_example_2() {
    let input = r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#;

    assert_eq!(part2(input), 230);
}

// Helpers ======================================================================

/// Parse given input into structured input
///
/// Because the cursed version doesn't use one of these, we can't use `aoc_generator`.
fn parse_input(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| u32::from_str_radix(line, 2).unwrap())
        .collect()
}

/// Count set and unset bits at `bit_idx`
fn count(readings: &[u32], bit_idx: u32) -> [u32; 2] {
    let mut counts = [0, 0];

    for x in readings.iter() {
        let i = (x >> bit_idx) & 0x1;
        counts[i as usize] += 1;
    }

    counts
}

/// Remove all readings where the bit at `bit_idx` does not equal `bit`
///
/// This may reorder `readings`!
fn cull_if_bit_ne(readings: &mut Vec<u32>, bit: u32, bit_idx: u32) {
    let bit_select = 0x1 << bit_idx;
    let bit_value = bit << bit_idx;

    // Consider removing each reading
    // Walk backwards so that removing doesn't disrupt iteration
    for idx in (0..readings.len()).rev() {
        if (readings[idx] & bit_select) == bit_value {
            readings.swap_remove(idx);
        }
    }
}

// Part1 ======================================================================
#[aoc(day3, part1, v2)]
#[inline(never)]
pub fn part1_v2(input: &str) -> u32 {
    let readings: Vec<_> = parse_input(input);

    let readings_max = *readings.iter().max().unwrap_or(&0);
    let bits_idx_max = 32 - readings_max.leading_zeros();

    let mut gamma: u32 = 0;
    for bit_idx in (0..bits_idx_max).rev() {
        gamma <<= 1;

        let [zeros, ones] = count(&readings, bit_idx);

        if ones >= zeros {
            gamma |= 1;
        }
    }

    let mask = u32::MAX >> gamma.leading_zeros();
    let epsilon = !gamma & mask;

    gamma * epsilon
}

#[test]
fn check_example_1_v2() {
    let input = r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#;

    assert_eq!(part1_v2(input), 198);
}

// Part2 ======================================================================
#[aoc(day3, part2, v2)]
#[inline(never)]
pub fn part2_v2(input: &str) -> u32 {
    let input: Vec<_> = parse_input(input);

    // We can compute these once, since `readings` is only ever shrunk
    let input_max = *input.iter().max().unwrap_or(&0);
    let bits_idx_max = 32 - input_max.leading_zeros();

    let oxygen_reading: u32 = {
        let mut readings = input.clone();

        for bit_idx in (0..bits_idx_max).rev() {
            // Re-compute this every loop, since we're updating the readings list
            let [zeros, ones] = count(&readings, bit_idx);

            // NOTE: This changes vs co_reading
            if ones >= zeros {
                cull_if_bit_ne(&mut readings, 0x0, bit_idx);
            } else {
                cull_if_bit_ne(&mut readings, 0x1, bit_idx);
            }

            // Once we have a unique solution, stop culling!
            if readings.len() == 1 {
                break;
            }
        }

        // We absolutely should have filtered everything but 1 by now
        debug_assert_eq!(readings.len(), 1);

        readings[0]
    };

    let co_reading: u32 = {
        let mut readings = input;

        for bit_idx in (0..bits_idx_max).rev() {
            // Re-compute this every loop, since we're updating the readings list
            let [zeros, ones] = count(&readings, bit_idx);

            // NOTE: This changes vs oxygen_reading
            if ones >= zeros {
                cull_if_bit_ne(&mut readings, 0x1, bit_idx);
            } else {
                cull_if_bit_ne(&mut readings, 0x0, bit_idx);
            }

            // Once we have a unique solution, stop culling!
            if readings.len() == 1 {
                break;
            }
        }

        // We absolutely should have filtered everything but 1 by now
        debug_assert_eq!(readings.len(), 1);

        readings[0]
    };

    oxygen_reading * co_reading
}

#[test]
fn check_example_2_v2() {
    let input = r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#;

    assert_eq!(part2_v2(input), 230);
}
