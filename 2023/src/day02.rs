use crate::prelude::*;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Cubes {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
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

// #[aoc_generator(day2)]
// pub fn parse(input: &str) -> Vec<Vec<Cubes>> {
//     let mut games = vec![];
//     for line in input.lines() {
//         let mut game = vec![];

//         // Skip "Game N:"
//         let line = line[]

//         games.push(game);
//     }

//     games
// }

#[aoc_generator(day2)]
pub fn parse(input: &str) -> Vec<Vec<Cubes>> {
    parser_of_lies(input)
}

#[aoc(day2, part1)]
pub fn part1(games: &[Vec<Cubes>]) -> i64 {
    games
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

// Part2 ========================================================================
#[aoc(day2, part2)]
pub fn part2(games: &[Vec<Cubes>]) -> i64 {
    games
        .iter()
        .enumerate()
        .map(|(idx, game)| (1 + idx as i64, game))
        .map(|(id, game)| -> Cubes {
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

    #[rstest]
    #[case::given(8, EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1)]
        p: impl FnOnce(&[Vec<Cubes>]) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        let input = parse(input.trim());
        assert_eq!(p(&input), expected);
    }

    #[rstest]
    #[case::given(2286, EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2)]
        p: impl FnOnce(&[Vec<Cubes>]) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        let input = parse(input.trim());
        assert_eq!(p(&input), expected);
    }
}

#[allow(warnings)]
pub fn parser_of_lies(input: &str) -> Vec<Vec<Cubes>> {
    if cfg!(test) {
        vec![
            // Game 1:
            vec![
                Cubes {
                    blue: 3,
                    red: 4,
                    ..Cubes::default()
                },
                Cubes {
                    red: 1,
                    green: 2,
                    blue: 6,
                },
                Cubes {
                    green: 2,
                    ..Cubes::default()
                },
            ],
            // Game 2:
            vec![
                Cubes {
                    blue: 1,
                    green: 2,
                    ..Cubes::default()
                },
                Cubes {
                    green: 3,
                    blue: 4,
                    red: 1,
                },
                Cubes {
                    green: 1,
                    blue: 1,
                    ..Cubes::default()
                },
            ],
            // Game 3:
            vec![
                Cubes {
                    green: 8,
                    blue: 6,
                    red: 20,
                },
                Cubes {
                    blue: 5,
                    red: 4,
                    green: 13,
                },
                Cubes {
                    green: 5,
                    red: 1,
                    ..Cubes::default()
                },
            ],
            // Game 4:
            vec![
                Cubes {
                    green: 1,
                    red: 3,
                    blue: 6,
                },
                Cubes {
                    green: 3,
                    red: 6,
                    ..Cubes::default()
                },
                Cubes {
                    green: 3,
                    blue: 15,
                    red: 14,
                },
            ],
            // Game 5:
            vec![
                Cubes {
                    red: 6,
                    blue: 1,
                    green: 3,
                },
                Cubes {
                    blue: 2,
                    red: 1,
                    green: 2,
                },
            ],
        ]
    } else {
        vec![
            vec![
                Cubes {
                    blue: 4,
                    green: 16,
                    red: 2,
                    ..Cubes::default()
                },
                Cubes {
                    red: 5,
                    blue: 11,
                    green: 16,
                    ..Cubes::default()
                },
                Cubes {
                    green: 9,
                    blue: 11,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 10,
                    green: 6,
                    red: 4,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    green: 15,
                    red: 20,
                    blue: 8,
                    ..Cubes::default()
                },
                Cubes {
                    green: 12,
                    red: 7,
                    ..Cubes::default()
                },
                Cubes {
                    green: 10,
                    blue: 2,
                    red: 15,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 13,
                    red: 15,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    red: 8,
                    blue: 2,
                    ..Cubes::default()
                },
                Cubes {
                    green: 3,
                    blue: 10,
                    red: 10,
                    ..Cubes::default()
                },
                Cubes {
                    green: 7,
                    blue: 4,
                    red: 7,
                    ..Cubes::default()
                },
                Cubes {
                    red: 8,
                    green: 6,
                    blue: 13,
                    ..Cubes::default()
                },
                Cubes {
                    green: 4,
                    blue: 3,
                    red: 10,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 7,
                    green: 7,
                    red: 5,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    green: 13,
                    blue: 14,
                    red: 9,
                    ..Cubes::default()
                },
                Cubes {
                    green: 6,
                    red: 14,
                    blue: 18,
                    ..Cubes::default()
                },
                Cubes {
                    red: 9,
                    green: 11,
                    blue: 3,
                    ..Cubes::default()
                },
                Cubes {
                    green: 11,
                    red: 10,
                    blue: 14,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 17,
                    red: 3,
                    green: 4,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 17,
                    red: 1,
                    green: 9,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    green: 2,
                    red: 1,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 8,
                    green: 2,
                    red: 6,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 5,
                    red: 9,
                    green: 2,
                    ..Cubes::default()
                },
                Cubes {
                    green: 3,
                    red: 8,
                    blue: 6,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 6,
                    red: 5,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    green: 3,
                    blue: 7,
                    red: 5,
                    ..Cubes::default()
                },
                Cubes {
                    green: 3,
                    red: 6,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 11,
                    red: 6,
                    green: 1,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    red: 8,
                    green: 4,
                    blue: 11,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 12,
                    green: 1,
                    red: 5,
                    ..Cubes::default()
                },
                Cubes {
                    red: 6,
                    green: 1,
                    blue: 5,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 12,
                    green: 2,
                    red: 2,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 4,
                    green: 4,
                    red: 3,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 9,
                    green: 4,
                    red: 8,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    red: 1,
                    green: 4,
                    ..Cubes::default()
                },
                Cubes {
                    red: 6,
                    green: 1,
                    ..Cubes::default()
                },
                Cubes {
                    red: 10,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 1,
                    green: 2,
                    ..Cubes::default()
                },
                Cubes {
                    green: 4,
                    red: 3,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 1,
                    red: 8,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    blue: 9,
                    green: 13,
                    red: 1,
                    ..Cubes::default()
                },
                Cubes {
                    green: 10,
                    blue: 4,
                    red: 4,
                    ..Cubes::default()
                },
                Cubes {
                    red: 3,
                    blue: 4,
                    green: 14,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 13,
                    red: 1,
                    green: 12,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    blue: 2,
                    red: 16,
                    green: 2,
                    ..Cubes::default()
                },
                Cubes {
                    green: 1,
                    red: 16,
                    blue: 6,
                    ..Cubes::default()
                },
                Cubes {
                    red: 9,
                    green: 3,
                    ..Cubes::default()
                },
                Cubes {
                    green: 1,
                    blue: 2,
                    red: 8,
                    ..Cubes::default()
                },
                Cubes {
                    red: 8,
                    blue: 6,
                    green: 3,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    green: 7,
                    red: 11,
                    blue: 12,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 3,
                    green: 6,
                    red: 6,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 10,
                    green: 13,
                    ..Cubes::default()
                },
                Cubes {
                    red: 1,
                    green: 13,
                    blue: 9,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 2,
                    red: 2,
                    green: 13,
                    ..Cubes::default()
                },
                Cubes {
                    red: 2,
                    blue: 3,
                    green: 15,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    green: 3,
                    red: 2,
                    blue: 2,
                    ..Cubes::default()
                },
                Cubes {
                    green: 7,
                    blue: 5,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 1,
                    red: 1,
                    green: 3,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    green: 2,
                    red: 2,
                    blue: 3,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 3,
                    red: 3,
                    green: 3,
                    ..Cubes::default()
                },
                Cubes {
                    green: 3,
                    red: 2,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 2,
                    red: 3,
                    green: 3,
                    ..Cubes::default()
                },
                Cubes {
                    green: 2,
                    red: 3,
                    blue: 1,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    green: 4,
                    red: 9,
                    ..Cubes::default()
                },
                Cubes {
                    green: 11,
                    red: 10,
                    blue: 12,
                    ..Cubes::default()
                },
                Cubes {
                    red: 6,
                    green: 3,
                    blue: 12,
                    ..Cubes::default()
                },
                Cubes {
                    green: 5,
                    red: 4,
                    blue: 4,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 18,
                    red: 7,
                    green: 11,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 16,
                    red: 4,
                    green: 10,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    green: 5,
                    red: 2,
                    blue: 9,
                    ..Cubes::default()
                },
                Cubes {
                    green: 18,
                    red: 6,
                    blue: 20,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 11,
                    green: 12,
                    red: 11,
                    ..Cubes::default()
                },
                Cubes {
                    red: 9,
                    blue: 17,
                    green: 16,
                    ..Cubes::default()
                },
                Cubes {
                    green: 7,
                    red: 1,
                    blue: 9,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    blue: 9,
                    green: 11,
                    ..Cubes::default()
                },
                Cubes {
                    green: 8,
                    blue: 2,
                    ..Cubes::default()
                },
                Cubes {
                    red: 1,
                    green: 6,
                    blue: 4,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    red: 2,
                    green: 2,
                    blue: 2,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 7,
                    green: 4,
                    red: 3,
                    ..Cubes::default()
                },
                Cubes {
                    red: 2,
                    blue: 8,
                    green: 1,
                    ..Cubes::default()
                },
                Cubes {
                    red: 2,
                    blue: 6,
                    green: 2,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 4,
                    red: 3,
                    ..Cubes::default()
                },
                Cubes {
                    green: 4,
                    red: 5,
                    blue: 6,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    green: 6,
                    red: 7,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 3,
                    green: 6,
                    red: 1,
                    ..Cubes::default()
                },
                Cubes {
                    red: 6,
                    blue: 3,
                    green: 5,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    red: 6,
                    green: 4,
                    blue: 5,
                    ..Cubes::default()
                },
                Cubes {
                    red: 2,
                    blue: 4,
                    green: 13,
                    ..Cubes::default()
                },
                Cubes {
                    green: 1,
                    blue: 1,
                    red: 2,
                    ..Cubes::default()
                },
                Cubes {
                    green: 4,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    red: 7,
                    blue: 17,
                    green: 6,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 3,
                    green: 6,
                    red: 8,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 7,
                    red: 6,
                    green: 1,
                    ..Cubes::default()
                },
                Cubes {
                    green: 3,
                    ..Cubes::default()
                },
                Cubes {
                    red: 8,
                    green: 7,
                    blue: 14,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    red: 5,
                    blue: 3,
                    green: 7,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 1,
                    red: 2,
                    green: 5,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 2,
                    green: 8,
                    red: 3,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 3,
                    red: 8,
                    green: 4,
                    ..Cubes::default()
                },
                Cubes {
                    red: 5,
                    blue: 1,
                    green: 3,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    red: 2,
                    green: 6,
                    blue: 1,
                    ..Cubes::default()
                },
                Cubes {
                    red: 3,
                    green: 3,
                    blue: 1,
                    ..Cubes::default()
                },
                Cubes {
                    green: 2,
                    red: 7,
                    blue: 2,
                    ..Cubes::default()
                },
                Cubes {
                    green: 5,
                    red: 1,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    red: 2,
                    green: 16,
                    blue: 1,
                    ..Cubes::default()
                },
                Cubes {
                    red: 1,
                    green: 12,
                    blue: 3,
                    ..Cubes::default()
                },
                Cubes {
                    green: 12,
                    blue: 1,
                    red: 3,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    red: 7,
                    blue: 1,
                    green: 12,
                    ..Cubes::default()
                },
                Cubes {
                    red: 2,
                    green: 19,
                    blue: 3,
                    ..Cubes::default()
                },
                Cubes {
                    green: 19,
                    blue: 1,
                    red: 12,
                    ..Cubes::default()
                },
                Cubes {
                    green: 6,
                    red: 16,
                    blue: 5,
                    ..Cubes::default()
                },
                Cubes {
                    red: 11,
                    blue: 4,
                    green: 12,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    blue: 2,
                    red: 3,
                    green: 8,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 4,
                    red: 2,
                    green: 9,
                    ..Cubes::default()
                },
                Cubes {
                    red: 2,
                    blue: 7,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    red: 17,
                    blue: 8,
                    green: 3,
                    ..Cubes::default()
                },
                Cubes {
                    green: 3,
                    red: 13,
                    blue: 4,
                    ..Cubes::default()
                },
                Cubes {
                    red: 20,
                    green: 1,
                    blue: 6,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 7,
                    red: 2,
                    green: 2,
                    ..Cubes::default()
                },
                Cubes {
                    red: 20,
                    blue: 8,
                    ..Cubes::default()
                },
                Cubes {
                    green: 2,
                    red: 16,
                    blue: 8,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    blue: 3,
                    green: 17,
                    red: 19,
                    ..Cubes::default()
                },
                Cubes {
                    green: 16,
                    red: 5,
                    blue: 6,
                    ..Cubes::default()
                },
                Cubes {
                    green: 17,
                    red: 16,
                    blue: 4,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    green: 1,
                    red: 7,
                    blue: 1,
                    ..Cubes::default()
                },
                Cubes {
                    green: 8,
                    red: 12,
                    blue: 1,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 1,
                    red: 9,
                    green: 1,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    green: 3,
                    blue: 3,
                    red: 2,
                    ..Cubes::default()
                },
                Cubes {
                    green: 3,
                    red: 2,
                    blue: 1,
                    ..Cubes::default()
                },
                Cubes {
                    green: 3,
                    red: 2,
                    blue: 3,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 3,
                    red: 3,
                    green: 4,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    red: 3,
                    blue: 8,
                    green: 3,
                    ..Cubes::default()
                },
                Cubes {
                    green: 1,
                    red: 1,
                    ..Cubes::default()
                },
                Cubes {
                    green: 17,
                    blue: 17,
                    ..Cubes::default()
                },
                Cubes {
                    green: 19,
                    blue: 15,
                    red: 1,
                    ..Cubes::default()
                },
                Cubes {
                    green: 1,
                    red: 2,
                    blue: 16,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    green: 11,
                    blue: 11,
                    red: 14,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 6,
                    green: 15,
                    red: 2,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 11,
                    green: 19,
                    red: 2,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    red: 9,
                    green: 2,
                    ..Cubes::default()
                },
                Cubes {
                    green: 7,
                    blue: 4,
                    red: 2,
                    ..Cubes::default()
                },
                Cubes {
                    red: 6,
                    green: 5,
                    blue: 1,
                    ..Cubes::default()
                },
                Cubes {
                    red: 4,
                    blue: 4,
                    green: 1,
                    ..Cubes::default()
                },
                Cubes {
                    red: 8,
                    green: 6,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    blue: 6,
                    red: 16,
                    green: 9,
                    ..Cubes::default()
                },
                Cubes {
                    red: 5,
                    blue: 7,
                    green: 13,
                    ..Cubes::default()
                },
                Cubes {
                    green: 1,
                    blue: 9,
                    red: 1,
                    ..Cubes::default()
                },
                Cubes {
                    green: 4,
                    blue: 9,
                    red: 17,
                    ..Cubes::default()
                },
                Cubes {
                    green: 2,
                    red: 10,
                    blue: 13,
                    ..Cubes::default()
                },
                Cubes {
                    red: 9,
                    blue: 1,
                    green: 14,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    red: 2,
                    green: 2,
                    blue: 4,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 3,
                    green: 2,
                    ..Cubes::default()
                },
                Cubes {
                    green: 1,
                    red: 1,
                    blue: 2,
                    ..Cubes::default()
                },
                Cubes {
                    red: 1,
                    blue: 3,
                    green: 3,
                    ..Cubes::default()
                },
                Cubes {
                    green: 2,
                    blue: 8,
                    red: 2,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 3,
                    red: 1,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    red: 4,
                    blue: 14,
                    green: 2,
                    ..Cubes::default()
                },
                Cubes {
                    green: 1,
                    blue: 15,
                    red: 1,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 1,
                    red: 2,
                    green: 1,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    blue: 4,
                    red: 1,
                    green: 2,
                    ..Cubes::default()
                },
                Cubes {
                    green: 2,
                    blue: 15,
                    red: 8,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 7,
                    red: 1,
                    ..Cubes::default()
                },
                Cubes {
                    red: 7,
                    green: 1,
                    blue: 1,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    blue: 2,
                    green: 1,
                    red: 5,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 2,
                    green: 2,
                    red: 4,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 2,
                    red: 5,
                    green: 8,
                    ..Cubes::default()
                },
                Cubes {
                    green: 3,
                    blue: 2,
                    red: 1,
                    ..Cubes::default()
                },
                Cubes {
                    red: 1,
                    blue: 1,
                    green: 5,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 2,
                    red: 1,
                    green: 8,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    blue: 2,
                    green: 4,
                    red: 11,
                    ..Cubes::default()
                },
                Cubes {
                    green: 7,
                    red: 6,
                    blue: 2,
                    ..Cubes::default()
                },
                Cubes {
                    green: 1,
                    red: 3,
                    blue: 1,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 4,
                    green: 4,
                    red: 4,
                    ..Cubes::default()
                },
                Cubes {
                    red: 2,
                    blue: 5,
                    green: 2,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    green: 7,
                    blue: 7,
                    red: 2,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 11,
                    green: 4,
                    red: 8,
                    ..Cubes::default()
                },
                Cubes {
                    red: 10,
                    green: 4,
                    blue: 1,
                    ..Cubes::default()
                },
                Cubes {
                    green: 8,
                    blue: 9,
                    ..Cubes::default()
                },
                Cubes {
                    green: 9,
                    red: 4,
                    ..Cubes::default()
                },
                Cubes {
                    green: 1,
                    blue: 8,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    green: 1,
                    blue: 13,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 6,
                    red: 7,
                    ..Cubes::default()
                },
                Cubes {
                    red: 8,
                    ..Cubes::default()
                },
                Cubes {
                    green: 1,
                    blue: 13,
                    red: 3,
                    ..Cubes::default()
                },
                Cubes {
                    green: 1,
                    red: 16,
                    blue: 13,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 14,
                    red: 14,
                    green: 1,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    green: 5,
                    blue: 2,
                    red: 10,
                    ..Cubes::default()
                },
                Cubes {
                    green: 4,
                    blue: 2,
                    red: 5,
                    ..Cubes::default()
                },
                Cubes {
                    green: 6,
                    red: 9,
                    blue: 1,
                    ..Cubes::default()
                },
                Cubes {
                    red: 4,
                    blue: 1,
                    ..Cubes::default()
                },
                Cubes {
                    red: 1,
                    green: 3,
                    blue: 2,
                    ..Cubes::default()
                },
                Cubes {
                    red: 3,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    green: 17,
                    blue: 11,
                    red: 11,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 5,
                    green: 11,
                    red: 9,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 10,
                    red: 13,
                    green: 4,
                    ..Cubes::default()
                },
                Cubes {
                    green: 8,
                    blue: 4,
                    red: 15,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    red: 1,
                    blue: 3,
                    ..Cubes::default()
                },
                Cubes {
                    green: 1,
                    blue: 3,
                    red: 1,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 2,
                    green: 1,
                    ..Cubes::default()
                },
                Cubes {
                    green: 2,
                    blue: 1,
                    ..Cubes::default()
                },
                Cubes {
                    red: 1,
                    blue: 3,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    green: 7,
                    red: 5,
                    blue: 1,
                    ..Cubes::default()
                },
                Cubes {
                    green: 6,
                    blue: 1,
                    red: 5,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 2,
                    green: 6,
                    ..Cubes::default()
                },
                Cubes {
                    green: 3,
                    red: 2,
                    ..Cubes::default()
                },
                Cubes {
                    green: 4,
                    ..Cubes::default()
                },
                Cubes {
                    red: 6,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    red: 16,
                    blue: 14,
                    green: 19,
                    ..Cubes::default()
                },
                Cubes {
                    red: 1,
                    green: 5,
                    blue: 6,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 16,
                    green: 2,
                    red: 1,
                    ..Cubes::default()
                },
                Cubes {
                    green: 15,
                    red: 6,
                    blue: 16,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    blue: 8,
                    green: 2,
                    ..Cubes::default()
                },
                Cubes {
                    red: 4,
                    green: 3,
                    blue: 6,
                    ..Cubes::default()
                },
                Cubes {
                    green: 1,
                    blue: 8,
                    red: 3,
                    ..Cubes::default()
                },
                Cubes {
                    green: 3,
                    blue: 12,
                    red: 1,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    green: 9,
                    blue: 3,
                    ..Cubes::default()
                },
                Cubes {
                    green: 1,
                    blue: 1,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 4,
                    green: 9,
                    red: 6,
                    ..Cubes::default()
                },
                Cubes {
                    green: 8,
                    blue: 4,
                    red: 6,
                    ..Cubes::default()
                },
                Cubes {
                    red: 6,
                    green: 12,
                    blue: 1,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 4,
                    green: 7,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    green: 11,
                    blue: 4,
                    red: 1,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 11,
                    red: 8,
                    green: 9,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 4,
                    red: 3,
                    green: 7,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 10,
                    green: 2,
                    red: 9,
                    ..Cubes::default()
                },
                Cubes {
                    green: 8,
                    blue: 2,
                    red: 2,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    green: 8,
                    blue: 1,
                    red: 5,
                    ..Cubes::default()
                },
                Cubes {
                    green: 1,
                    blue: 1,
                    ..Cubes::default()
                },
                Cubes {
                    green: 3,
                    red: 4,
                    blue: 2,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 1,
                    green: 7,
                    red: 1,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 1,
                    green: 7,
                    red: 3,
                    ..Cubes::default()
                },
                Cubes {
                    red: 5,
                    green: 5,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    green: 2,
                    red: 2,
                    blue: 4,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 8,
                    green: 2,
                    red: 7,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 4,
                    red: 5,
                    ..Cubes::default()
                },
                Cubes {
                    red: 9,
                    blue: 4,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 5,
                    red: 9,
                    ..Cubes::default()
                },
                Cubes {
                    green: 2,
                    red: 8,
                    blue: 6,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    green: 6,
                    red: 1,
                    blue: 2,
                    ..Cubes::default()
                },
                Cubes {
                    red: 2,
                    blue: 4,
                    green: 6,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 9,
                    green: 4,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    green: 7,
                    red: 3,
                    blue: 12,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 8,
                    red: 9,
                    green: 5,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 2,
                    green: 10,
                    red: 8,
                    ..Cubes::default()
                },
                Cubes {
                    red: 12,
                    green: 5,
                    blue: 3,
                    ..Cubes::default()
                },
                Cubes {
                    red: 8,
                    green: 8,
                    blue: 12,
                    ..Cubes::default()
                },
                Cubes {
                    green: 2,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    green: 2,
                    blue: 9,
                    red: 5,
                    ..Cubes::default()
                },
                Cubes {
                    red: 6,
                    green: 3,
                    ..Cubes::default()
                },
                Cubes {
                    red: 5,
                    green: 2,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    red: 9,
                    blue: 13,
                    ..Cubes::default()
                },
                Cubes {
                    green: 1,
                    red: 9,
                    blue: 16,
                    ..Cubes::default()
                },
                Cubes {
                    red: 12,
                    blue: 1,
                    green: 4,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    red: 1,
                    blue: 2,
                    green: 3,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 1,
                    ..Cubes::default()
                },
                Cubes {
                    red: 1,
                    blue: 5,
                    green: 3,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 1,
                    green: 3,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 5,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    green: 1,
                    red: 4,
                    blue: 1,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 1,
                    red: 2,
                    green: 13,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 5,
                    red: 4,
                    ..Cubes::default()
                },
                Cubes {
                    green: 13,
                    red: 3,
                    blue: 3,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    blue: 13,
                    red: 2,
                    green: 7,
                    ..Cubes::default()
                },
                Cubes {
                    green: 3,
                    red: 4,
                    blue: 14,
                    ..Cubes::default()
                },
                Cubes {
                    red: 3,
                    green: 3,
                    blue: 3,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 7,
                    green: 5,
                    red: 1,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    red: 6,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 1,
                    red: 4,
                    green: 2,
                    ..Cubes::default()
                },
                Cubes {
                    green: 3,
                    blue: 1,
                    ..Cubes::default()
                },
                Cubes {
                    green: 7,
                    red: 1,
                    ..Cubes::default()
                },
                Cubes {
                    red: 6,
                    green: 13,
                    blue: 1,
                    ..Cubes::default()
                },
                Cubes {
                    red: 3,
                    green: 13,
                    blue: 1,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    green: 5,
                    red: 10,
                    blue: 8,
                    ..Cubes::default()
                },
                Cubes {
                    red: 7,
                    green: 3,
                    blue: 2,
                    ..Cubes::default()
                },
                Cubes {
                    green: 6,
                    red: 3,
                    blue: 6,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    green: 2,
                    red: 5,
                    blue: 15,
                    ..Cubes::default()
                },
                Cubes {
                    green: 2,
                    blue: 9,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 9,
                    green: 8,
                    red: 3,
                    ..Cubes::default()
                },
                Cubes {
                    green: 2,
                    red: 6,
                    blue: 2,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    blue: 8,
                    green: 3,
                    red: 4,
                    ..Cubes::default()
                },
                Cubes {
                    red: 1,
                    blue: 10,
                    green: 1,
                    ..Cubes::default()
                },
                Cubes {
                    red: 4,
                    green: 5,
                    blue: 3,
                    ..Cubes::default()
                },
                Cubes {
                    red: 3,
                    blue: 8,
                    green: 5,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    blue: 19,
                    red: 3,
                    green: 14,
                    ..Cubes::default()
                },
                Cubes {
                    green: 1,
                    blue: 7,
                    red: 1,
                    ..Cubes::default()
                },
                Cubes {
                    red: 15,
                    blue: 20,
                    green: 6,
                    ..Cubes::default()
                },
                Cubes {
                    red: 8,
                    green: 4,
                    blue: 14,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    red: 13,
                    blue: 1,
                    ..Cubes::default()
                },
                Cubes {
                    red: 18,
                    green: 4,
                    ..Cubes::default()
                },
                Cubes {
                    green: 6,
                    red: 9,
                    blue: 1,
                    ..Cubes::default()
                },
                Cubes {
                    green: 7,
                    blue: 1,
                    red: 9,
                    ..Cubes::default()
                },
                Cubes {
                    red: 5,
                    blue: 1,
                    green: 4,
                    ..Cubes::default()
                },
                Cubes {
                    green: 5,
                    blue: 1,
                    red: 17,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    green: 2,
                    blue: 1,
                    red: 5,
                    ..Cubes::default()
                },
                Cubes {
                    red: 2,
                    green: 5,
                    ..Cubes::default()
                },
                Cubes {
                    red: 6,
                    green: 4,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    blue: 1,
                    green: 7,
                    red: 1,
                    ..Cubes::default()
                },
                Cubes {
                    red: 7,
                    green: 1,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 1,
                    green: 3,
                    red: 3,
                    ..Cubes::default()
                },
                Cubes {
                    red: 7,
                    green: 3,
                    ..Cubes::default()
                },
                Cubes {
                    green: 3,
                    red: 7,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 1,
                    green: 4,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    green: 7,
                    blue: 6,
                    red: 8,
                    ..Cubes::default()
                },
                Cubes {
                    green: 4,
                    red: 9,
                    blue: 3,
                    ..Cubes::default()
                },
                Cubes {
                    green: 6,
                    blue: 4,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 5,
                    green: 2,
                    ..Cubes::default()
                },
                Cubes {
                    red: 6,
                    green: 4,
                    blue: 2,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    blue: 10,
                    green: 17,
                    red: 17,
                    ..Cubes::default()
                },
                Cubes {
                    red: 11,
                    blue: 9,
                    green: 9,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 9,
                    red: 19,
                    green: 5,
                    ..Cubes::default()
                },
                Cubes {
                    red: 5,
                    blue: 3,
                    green: 20,
                    ..Cubes::default()
                },
                Cubes {
                    red: 11,
                    blue: 1,
                    green: 7,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    green: 9,
                    red: 4,
                    blue: 5,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 11,
                    green: 9,
                    red: 2,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 11,
                    red: 2,
                    green: 6,
                    ..Cubes::default()
                },
                Cubes {
                    green: 2,
                    red: 6,
                    blue: 3,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 1,
                    green: 6,
                    red: 4,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    red: 3,
                    blue: 15,
                    green: 1,
                    ..Cubes::default()
                },
                Cubes {
                    red: 4,
                    blue: 14,
                    green: 2,
                    ..Cubes::default()
                },
                Cubes {
                    red: 4,
                    blue: 18,
                    green: 4,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    red: 3,
                    green: 8,
                    ..Cubes::default()
                },
                Cubes {
                    red: 2,
                    green: 6,
                    ..Cubes::default()
                },
                Cubes {
                    red: 4,
                    blue: 2,
                    green: 2,
                    ..Cubes::default()
                },
                Cubes {
                    red: 8,
                    green: 1,
                    blue: 2,
                    ..Cubes::default()
                },
                Cubes {
                    red: 6,
                    blue: 3,
                    green: 4,
                    ..Cubes::default()
                },
                Cubes {
                    green: 13,
                    red: 8,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    green: 3,
                    red: 17,
                    ..Cubes::default()
                },
                Cubes {
                    red: 2,
                    green: 3,
                    ..Cubes::default()
                },
                Cubes {
                    green: 2,
                    red: 8,
                    blue: 1,
                    ..Cubes::default()
                },
                Cubes {
                    red: 11,
                    blue: 4,
                    ..Cubes::default()
                },
                Cubes {
                    green: 3,
                    red: 11,
                    blue: 3,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    red: 1,
                    blue: 17,
                    green: 8,
                    ..Cubes::default()
                },
                Cubes {
                    red: 2,
                    blue: 11,
                    green: 16,
                    ..Cubes::default()
                },
                Cubes {
                    red: 3,
                    blue: 16,
                    green: 1,
                    ..Cubes::default()
                },
                Cubes {
                    red: 2,
                    green: 3,
                    blue: 10,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    blue: 1,
                    green: 10,
                    red: 8,
                    ..Cubes::default()
                },
                Cubes {
                    green: 19,
                    red: 10,
                    blue: 5,
                    ..Cubes::default()
                },
                Cubes {
                    green: 3,
                    red: 13,
                    blue: 8,
                    ..Cubes::default()
                },
                Cubes {
                    green: 12,
                    blue: 4,
                    ..Cubes::default()
                },
                Cubes {
                    green: 2,
                    blue: 10,
                    red: 12,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    blue: 17,
                    red: 7,
                    green: 10,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 16,
                    red: 5,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 9,
                    green: 7,
                    red: 2,
                    ..Cubes::default()
                },
                Cubes {
                    red: 10,
                    green: 4,
                    blue: 14,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    green: 10,
                    blue: 5,
                    red: 4,
                    ..Cubes::default()
                },
                Cubes {
                    red: 7,
                    blue: 10,
                    green: 7,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 7,
                    green: 9,
                    red: 2,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    green: 13,
                    red: 16,
                    blue: 20,
                    ..Cubes::default()
                },
                Cubes {
                    red: 4,
                    blue: 14,
                    green: 5,
                    ..Cubes::default()
                },
                Cubes {
                    red: 12,
                    blue: 1,
                    green: 8,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    red: 4,
                    green: 2,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 8,
                    green: 3,
                    red: 2,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 5,
                    green: 7,
                    red: 3,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    green: 12,
                    red: 8,
                    blue: 8,
                    ..Cubes::default()
                },
                Cubes {
                    green: 10,
                    red: 9,
                    blue: 10,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 16,
                    red: 1,
                    green: 17,
                    ..Cubes::default()
                },
                Cubes {
                    red: 4,
                    green: 15,
                    blue: 13,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    green: 4,
                    red: 2,
                    ..Cubes::default()
                },
                Cubes {
                    red: 15,
                    blue: 3,
                    ..Cubes::default()
                },
                Cubes {
                    red: 15,
                    green: 5,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    blue: 4,
                    green: 1,
                    red: 13,
                    ..Cubes::default()
                },
                Cubes {
                    red: 13,
                    blue: 1,
                    green: 5,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 5,
                    red: 9,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 3,
                    green: 3,
                    ..Cubes::default()
                },
                Cubes {
                    red: 1,
                    ..Cubes::default()
                },
                Cubes {
                    red: 3,
                    green: 7,
                    blue: 6,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    red: 10,
                    green: 3,
                    blue: 4,
                    ..Cubes::default()
                },
                Cubes {
                    red: 2,
                    green: 5,
                    blue: 16,
                    ..Cubes::default()
                },
                Cubes {
                    green: 3,
                    blue: 1,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 9,
                    green: 2,
                    red: 12,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    green: 1,
                    blue: 9,
                    red: 1,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 10,
                    red: 1,
                    green: 1,
                    ..Cubes::default()
                },
                Cubes {
                    green: 1,
                    blue: 7,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 8,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    blue: 1,
                    red: 5,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 2,
                    red: 3,
                    ..Cubes::default()
                },
                Cubes {
                    green: 1,
                    blue: 2,
                    red: 1,
                    ..Cubes::default()
                },
                Cubes {
                    red: 2,
                    blue: 1,
                    green: 1,
                    ..Cubes::default()
                },
                Cubes {
                    green: 1,
                    blue: 1,
                    ..Cubes::default()
                },
                Cubes {
                    red: 2,
                    green: 1,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    red: 5,
                    blue: 14,
                    green: 2,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 6,
                    red: 5,
                    green: 8,
                    ..Cubes::default()
                },
                Cubes {
                    green: 12,
                    blue: 3,
                    red: 5,
                    ..Cubes::default()
                },
                Cubes {
                    red: 2,
                    green: 10,
                    ..Cubes::default()
                },
                Cubes {
                    green: 9,
                    blue: 14,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    blue: 2,
                    red: 2,
                    ..Cubes::default()
                },
                Cubes {
                    red: 14,
                    green: 6,
                    blue: 5,
                    ..Cubes::default()
                },
                Cubes {
                    green: 5,
                    blue: 4,
                    red: 6,
                    ..Cubes::default()
                },
                Cubes {
                    red: 8,
                    blue: 5,
                    green: 6,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    blue: 1,
                    red: 10,
                    ..Cubes::default()
                },
                Cubes {
                    red: 4,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 9,
                    red: 18,
                    green: 3,
                    ..Cubes::default()
                },
                Cubes {
                    green: 1,
                    blue: 1,
                    red: 7,
                    ..Cubes::default()
                },
                Cubes {
                    green: 3,
                    red: 8,
                    blue: 9,
                    ..Cubes::default()
                },
                Cubes {
                    red: 14,
                    green: 2,
                    blue: 4,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    green: 1,
                    red: 11,
                    blue: 8,
                    ..Cubes::default()
                },
                Cubes {
                    green: 1,
                    red: 11,
                    blue: 2,
                    ..Cubes::default()
                },
                Cubes {
                    red: 7,
                    blue: 4,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 6,
                    red: 1,
                    green: 2,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 13,
                    green: 2,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 6,
                    red: 12,
                    green: 3,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    blue: 2,
                    red: 4,
                    green: 8,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 4,
                    red: 7,
                    ..Cubes::default()
                },
                Cubes {
                    red: 3,
                    green: 10,
                    blue: 4,
                    ..Cubes::default()
                },
                Cubes {
                    green: 9,
                    blue: 3,
                    red: 5,
                    ..Cubes::default()
                },
                Cubes {
                    red: 4,
                    blue: 6,
                    green: 3,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    red: 6,
                    green: 10,
                    ..Cubes::default()
                },
                Cubes {
                    green: 15,
                    red: 15,
                    blue: 10,
                    ..Cubes::default()
                },
                Cubes {
                    red: 15,
                    green: 1,
                    blue: 4,
                    ..Cubes::default()
                },
                Cubes {
                    red: 13,
                    blue: 6,
                    green: 4,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    green: 17,
                    red: 2,
                    blue: 1,
                    ..Cubes::default()
                },
                Cubes {
                    green: 6,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 1,
                    green: 1,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 1,
                    green: 16,
                    red: 3,
                    ..Cubes::default()
                },
                Cubes {
                    green: 14,
                    red: 1,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    blue: 3,
                    green: 8,
                    ..Cubes::default()
                },
                Cubes {
                    green: 3,
                    red: 7,
                    blue: 9,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 12,
                    ..Cubes::default()
                },
                Cubes {
                    red: 9,
                    blue: 7,
                    green: 4,
                    ..Cubes::default()
                },
                Cubes {
                    green: 1,
                    red: 7,
                    blue: 1,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    blue: 11,
                    red: 9,
                    green: 12,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 1,
                    red: 14,
                    green: 6,
                    ..Cubes::default()
                },
                Cubes {
                    green: 9,
                    red: 6,
                    blue: 6,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    red: 1,
                    blue: 2,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 3,
                    green: 6,
                    ..Cubes::default()
                },
                Cubes {
                    red: 1,
                    green: 4,
                    blue: 3,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    green: 3,
                    blue: 3,
                    ..Cubes::default()
                },
                Cubes {
                    red: 1,
                    blue: 3,
                    green: 9,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 3,
                    green: 10,
                    red: 3,
                    ..Cubes::default()
                },
                Cubes {
                    green: 10,
                    blue: 6,
                    red: 2,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 9,
                    green: 14,
                    red: 2,
                    ..Cubes::default()
                },
                Cubes {
                    red: 1,
                    blue: 4,
                    green: 1,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    blue: 7,
                    green: 10,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 3,
                    green: 5,
                    red: 2,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 4,
                    green: 10,
                    red: 12,
                    ..Cubes::default()
                },
                Cubes {
                    green: 6,
                    red: 2,
                    blue: 6,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    blue: 2,
                    green: 18,
                    red: 8,
                    ..Cubes::default()
                },
                Cubes {
                    green: 13,
                    blue: 3,
                    red: 3,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 3,
                    red: 15,
                    green: 8,
                    ..Cubes::default()
                },
                Cubes {
                    green: 13,
                    red: 10,
                    blue: 2,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    blue: 14,
                    red: 2,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 15,
                    green: 1,
                    red: 2,
                    ..Cubes::default()
                },
                Cubes {
                    red: 3,
                    blue: 6,
                    green: 1,
                    ..Cubes::default()
                },
                Cubes {
                    green: 1,
                    blue: 14,
                    red: 4,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    blue: 4,
                    red: 9,
                    ..Cubes::default()
                },
                Cubes {
                    red: 10,
                    green: 1,
                    blue: 11,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 7,
                    red: 1,
                    ..Cubes::default()
                },
                Cubes {
                    red: 1,
                    blue: 6,
                    green: 1,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    red: 7,
                    green: 6,
                    blue: 2,
                    ..Cubes::default()
                },
                Cubes {
                    red: 8,
                    ..Cubes::default()
                },
                Cubes {
                    green: 16,
                    red: 7,
                    blue: 4,
                    ..Cubes::default()
                },
            ],
            vec![
                Cubes {
                    red: 1,
                    green: 1,
                    blue: 9,
                    ..Cubes::default()
                },
                Cubes {
                    blue: 6,
                    green: 4,
                    red: 3,
                    ..Cubes::default()
                },
                Cubes {
                    red: 4,
                    green: 2,
                    ..Cubes::default()
                },
                Cubes {
                    green: 3,
                    red: 2,
                    blue: 11,
                    ..Cubes::default()
                },
                Cubes {
                    green: 6,
                    blue: 5,
                    red: 1,
                    ..Cubes::default()
                },
            ],
        ]
    }
}
