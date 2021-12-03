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
