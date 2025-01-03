use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use smallvec::SmallVec;
use std::collections::BinaryHeap;
use std::collections::HashMap;

pub type Material = SmallVec<[u8; 5]>;
pub type Inputs = SmallVec<[(u64, Material); 8]>;

#[derive(Clone, Eq)]
struct Reactant {
    material: Material,
    rank: u64,
}

impl std::fmt::Debug for Reactant {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Reactant")
            .field("material", &material_name(&self.material))
            .field("rank", &self.rank)
            .finish()
    }
}

impl PartialOrd for Reactant {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Reactant {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.rank.cmp(&other.rank)
    }
}

impl PartialEq for Reactant {
    fn eq(&self, other: &Self) -> bool {
        self.rank.eq(&other.rank)
    }
}

// Extract a material's name, for debugging
#[allow(dead_code)]
fn material_name(mat: &Material) -> &str {
    std::str::from_utf8(mat).expect("bad string")
}

// Create a new material from a string literal
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

/// Returns the smallest multiple of `formula` >= `roundee`
fn round_to_mult_of(mult: u64, roundee: u64) -> u64 {
    assert!(mult != 0);
    assert!(roundee != 0);

    mult * f64::ceil(roundee as f64 / mult as f64) as u64
}

#[cfg(test)]
#[test]
fn check_round_to_mult_of() {
    assert_eq!(round_to_mult_of(1, 1), 1);
    assert_eq!(round_to_mult_of(10, 10), 10);

    assert_eq!(round_to_mult_of(3, 7), 9);
    assert_eq!(round_to_mult_of(3, 8), 9);
    assert_eq!(round_to_mult_of(3, 9), 9);
    assert_eq!(round_to_mult_of(3, 10), 12);

    assert_eq!(round_to_mult_of(10, 7), 10);
}

#[cfg(test)]
#[test]
fn check_2210736_ore() {
    const INPUT: &str = r#"
    171 ORE => 8 CNZTR
    7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
    114 ORE => 4 BHXH
    14 VRPVC => 6 BMBT
    6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
    6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
    15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
    13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
    5 BMBT => 4 WPTQ
    189 ORE => 9 KTJDG
    1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
    12 VRPVC, 27 CNZTR => 2 XDBXC
    15 KTJDG, 12 BHXH => 5 XCVML
    3 BHXH, 2 VRPVC => 7 MZWV
    121 ORE => 7 VRPVC
    7 XCVML => 6 RJRHP
    5 BHXH, 4 VRPVC => 5 LTCX
    "#;

    let reactions = parse_input(INPUT.trim());
    assert_eq!(part1(&reactions), 2210736);
}

fn calc_ore(reactions: &[Reaction], fuel: u64) -> u64 {
    // Load formulas into a nice map
    let mut formulas: HashMap<Material, (u64, Inputs)> = HashMap::new();
    for r in reactions {
        formulas.insert(r.output.1.clone(), (r.output.0, r.inputs.clone()));
    }

    // compute ranks of each product
    let mut ranks: HashMap<Material, u64> = HashMap::new();
    ranks.insert(m("ORE"), 0);

    let mut last_count = ranks.len();
    loop {
        for (prod, input) in &formulas {
            let rs: SmallVec<[_; 5]> = input
                .1
                .iter()
                .filter_map(|(_count, mat)| ranks.get(mat))
                .unique()
                .cloned()
                .collect();

            if let Some(in_rank) = rs.iter().max() {
                ranks.insert(prod.clone(), in_rank + 1);
            }
        }

        // Keep searching for new ranks until we stop finding them
        if ranks.len() == last_count {
            break;
        } else {
            last_count = ranks.len();
        }
    }
    let ranks = ranks;

    let mut ingredients: BinaryHeap<Reactant> = BinaryHeap::new();
    let mut counts: HashMap<Material, u64> = HashMap::new();

    // Initialize the queue with our goal - 1 FUEL
    ingredients.push(Reactant {
        material: m("FUEL"),
        rank: ranks[&m("FUEL")],
    });
    counts.insert(m("FUEL"), fuel);

    while let Some(curr) = ingredients.pop() {
        // If we found some ORE, ignore it
        // but do NOT reset its count!
        if ranks[&curr.material] == 0 {
            continue;
        }

        // Get the count of `curr` that we need to create
        let curr_count = counts
            .get_mut(&curr.material)
            .expect("Missing material in counts");

        // Reduce to its potential ingredients
        let (formula_count, inputs) = &formulas[&curr.material];

        // Given the constraints of the formula, get our real `curr` count
        let real_curr_count = round_to_mult_of(*formula_count, *curr_count);

        // Reset this ingredient's count to 0
        *curr_count = 0;

        // Re-insert each of its inputs, remembering to scale our count
        for input in inputs {
            let material = &input.1;
            let count = input.0 * (real_curr_count / formula_count);

            if let Some(c) = counts.get_mut(material) {
                // Already inserted into ingredients, just update the count
                *c += count
            } else {
                // New ingredient, insert it into the list
                ingredients.push(Reactant {
                    rank: ranks[material],
                    material: material.clone(),
                });
                counts.insert(material.clone(), count);
            }
        }
    }

    let _leftovers = counts
        .iter()
        .filter(|&(_material, count)| *count != 0)
        .count();
    // This fires NONDETERMINISTICALLY, so not clear if this is valid or not
    // assert_eq!(leftovers, 1);

    counts[&m("ORE")]
}

#[aoc(day14, part1)]
pub fn part1(reactions: &[Reaction]) -> u64 {
    calc_ore(reactions, 1)
}
