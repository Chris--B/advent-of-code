use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use smallvec::SmallVec;
use std::collections::HashMap;

pub type Material = SmallVec<[u8; 5]>;
pub type Inputs = SmallVec<[(u64, Material); 8]>;

fn material_name(mat: &Material) -> &str {
    std::str::from_utf8(&mat).expect("bad string")
}

fn m(s: &str) -> Material {
    s.as_bytes().iter().copied().collect()
}

#[derive(Clone, Debug, Hash)]
pub struct Reaction {
    inputs: Inputs,
    output: (u64, Material),
}

impl std::str::FromStr for Reaction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut eq_iter = s.trim().split("=>");

        let inputs: Inputs = eq_iter
            .next()
            .unwrap()
            .split(',')
            .map(|thing| {
                let mut iter = thing.trim().split(' ');
                let count: u64 = iter.next().unwrap().parse().unwrap();
                let material: Material = iter.next().unwrap().as_bytes().iter().copied().collect();
                (count, material)
            })
            .collect();

        let output = eq_iter
            .next()
            .map(|thing| {
                let mut iter = thing.trim().split(' ');
                let count: u64 = iter.next().unwrap().parse().unwrap();
                let material: Material = iter
                    .next()
                    .unwrap()
                    .trim()
                    .as_bytes()
                    .iter()
                    .copied()
                    .collect();
                (count, material)
            })
            .unwrap();

        Ok(Reaction { inputs, output })
    }
}

#[aoc_generator(day14)]
pub fn parse_input(input: &str) -> Vec<Reaction> {
    input
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect()
}

#[cfg(test)]
#[test]
fn check_part1_31_ore() {
    const INPUT: &str = r#"
    10 ORE => 10 A
    1 ORE => 1 B
    7 A, 1 B => 1 C
    7 A, 1 C => 1 D
    7 A, 1 D => 1 E
    7 A, 1 E => 1 FUEL
    "#;

    let reactions = parse_input(INPUT.trim());
    assert_eq!(part1(&reactions), 31);
}

#[aoc(day14, part1)]
pub fn part1(reactions: &[Reaction]) -> i64 {
    // Load formulas into a nice map
    let mut formulas: HashMap<(u64, Material), Inputs> = HashMap::new();
    for r in reactions {
        dbg!(material_name(&r.output.1));
        formulas.insert(r.output.clone(), r.inputs.clone());
    }
    dbg!(formulas.len());

    // compute ranks of each product
    let mut ranks: HashMap<Material, u64> = HashMap::new();
    ranks.insert(m("ORE"), 0);

    let mut last_count = ranks.len();
    loop {
        dbg!(ranks.len());

        for (prod, input) in &formulas {
            let rs: SmallVec<[_; 5]> = input
                .iter()
                .filter_map(|(_count, mat)| ranks.get(mat))
                .unique()
                .cloned()
                .collect();

            if let Some(in_rank) = rs.iter().max() {
                dbg!(material_name(&prod.1), &rs);
                ranks.insert(prod.1.clone(), in_rank + 1);
            }
        }
        println!();

        // Keep searching for new ranks until we stop finding them
        if ranks.len() == last_count {
            break;
        } else {
            last_count = ranks.len();
        }
    }

    for (m, rank) in ranks {
        println!("{}: {}", material_name(&m), rank);
    }

    // TODO: Loop over products, reducing until just ore
    // We want a queue for ingrediants
    //  while anything is rank > 0
    //      pick highest rank thing left
    //      reduce it once
    //      replace with reduced
    //  }

    0
}
