use crate::prelude::*;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Cubes {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl Cubes {
    pub fn new() -> Self {
        Self::default()
    }
}

// Part1 ========================================================================
fn is_possible(game: &[Cubes], c: &Cubes) -> bool {
    for draw in game {
        if draw.red > c.red || draw.green > c.green || draw.blue > c.blue {
            return false;
        }
    }

    true
}

pub fn parse(input: &str) -> Vec<Vec<Cubes>> {
    let mut games = vec![];

    for line in input.trim().lines() {
        // Example line:
        //      Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green

        // Skip "Game "
        let mut line = &line[5..];
        // skip digits e.g "12"
        while line.chars().next().map(|c| c.is_numeric()).unwrap() {
            line = &line[1..];
        }
        // skip ":"
        line = &line[1..];

        // Start parsing ';'-delineated draws
        let mut game = vec![];
        for draw_str in line.split(';') {
            let mut cube = Cubes::new();

            for field_str in draw_str.trim().split(',') {
                let mut parts = field_str.split_whitespace();

                let count: u32 = parts.next().unwrap().parse().unwrap();
                match parts.next().unwrap() {
                    "red" => cube.red = count,
                    "green" => cube.green = count,
                    "blue" => cube.blue = count,
                    other => panic!("Unrecognized field: {other}"),
                }
            }

            game.push(cube);
        }

        games.push(game);
    }

    games
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> i64 {
    parse(input)
        .iter()
        .enumerate()
        .map(|(idx, game)| (1 + idx as i64, game))
        .map(|(id, game)| -> i64 {
            if is_possible(
                game,
                &Cubes {
                    red: 12,
                    green: 13,
                    blue: 14,
                },
            ) {
                id
            } else {
                0
            }
        })
        .sum()
}

#[aoc(day2, part1, keep_tally)]
pub fn part1_keep_tally(games: &str) -> i64 {
    let given = Cubes {
        red: 12,
        green: 13,
        blue: 14,
    };

    let mut sum = 0;

    'game_loop: for (idx, game_line) in games.trim().lines().enumerate() {
        let id = 1 + idx as i64;

        let (_prefix, draws) = game_line.split_once(':').unwrap();
        for entry in draws.split([',', ';']) {
            let (count, field) = entry.trim().split_once(' ').unwrap();
            let count: u32 = count.parse().unwrap();

            match field.trim() {
                "red" => {
                    if count > given.red {
                        continue 'game_loop;
                    }
                }
                "green" => {
                    if count > given.green {
                        continue 'game_loop;
                    }
                }
                "blue" => {
                    if count > given.blue {
                        continue 'game_loop;
                    }
                }
                other => unreachable!("{other}"),
            }
        }

        sum += id;
    }

    sum
}

// Part2 ========================================================================
#[aoc(day2, part2)]
pub fn part2(input: &str) -> i64 {
    parse(input)
        .iter()
        .map(|game| -> Cubes {
            let mut power = Cubes::default();
            for draw in game {
                power.red = power.red.max(draw.red);
                power.green = power.green.max(draw.green);
                power.blue = power.blue.max(draw.blue);
            }
            power
        })
        .map(|c| (c.red * c.green * c.blue) as i64)
        .sum()
}

#[aoc(day2, part2, keep_tally)]
pub fn part2_keep_tally(games: &str) -> i64 {
    let mut sum = 0;

    for game_line in games.trim().lines() {
        let mut min = Cubes::new();

        let (_prefix, draws) = game_line.split_once(':').unwrap();
        for entry in draws.split([',', ';']) {
            let (count, field) = entry.trim().split_once(' ').unwrap();
            let count: u32 = count.parse().unwrap();

            match field.trim() {
                "red" => min.red = count.max(min.red),
                "green" => min.green = count.max(min.green),
                "blue" => min.blue = count.max(min.blue),
                other => unreachable!("{other}"),
            }
        }

        sum += (min.red * min.green * min.blue) as i64;
    }

    sum
}

#[aoc(day2, part2, manual_parsing)]
pub fn part2_manual_parsing(input: &str) -> i64 {
    let input = input.trim().as_bytes();

    let mut rgb = [0; 3];
    let mut sum = 0;

    let mut pos = 1;
    let mut game = 1;

    while pos < input.len() {
        // Sample line:
        //      "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
        if input[pos] == b'a' {
            // Skip over "Game N:" prefix, accounting for 1, 2, or 3 digit Game IDs
            pos += 7 + (game > 9) as usize + (game > 99) as usize;
            game += 1;

            // Save and reset the game state, since we found a new game
            sum += rgb[0] * rgb[1] * rgb[2];
            rgb = [0; 3];
        } else {
            // We expect a cube count next, so parse it inline here
            let mut v: u32 = (input[pos] - b'0') as u32;
            pos += 1;

            while input[pos] != b' ' {
                v = 10 * v + (input[pos] - b'0') as u32;
                pos += 1;
            }
            // skip ' '
            pos += 1;

            // Use some bit magic to compute the rgb index, so we can avoid branching here
            let col: usize = ((input[pos + 1] as usize) >> 3) & 3;
            rgb[col] = rgb[col].max(v);

            pos += 5 + col;
        }
    }

    // Since we finalize a game when starting a new one, we'll never finalize the final game.
    // Do so now.
    sum += rgb[0] * rgb[1] * rgb[2];

    sum as i64
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    #[test]
    fn check_parser() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let expected = &[
            Cubes {
                red: 4,
                green: 0,
                blue: 3,
            },
            Cubes {
                red: 1,
                green: 2,
                blue: 6,
            },
            Cubes {
                red: 0,
                green: 2,
                blue: 0,
            },
        ];

        let result = parse(line);
        assert_eq!(result.len(), 1);
        assert_eq!(
            result[0], expected,
            "Parsed game wrong!\n\"{line}\" as\n{result:#?}"
        );
    }

    #[rstest]
    #[case::given(8, EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1, part1_keep_tally)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        assert_eq!(p(input), expected);
    }

    #[rstest]
    #[case::given(2286, EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2, part2_keep_tally, part2_manual_parsing)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        assert_eq!(p(input), expected);
    }
}
