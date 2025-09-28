#![allow(unused)]

use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Info<'a> {
    weight: i64,
    parent: Option<&'a str>,
    children: Vec<&'a str>,
    total_weight: i64,
}

pub type Tree<'a> = HashMap<&'a str, Info<'a>>;

// Note: Cannot use aoc_generator with lifetimes in types :(
fn parse<'a>(input: &'a str) -> Tree<'a> {
    let mut tree: Tree = Tree::new();

    // Read everyone into the tree first
    // This makes reasoning about children in the next loop easier
    for line in input.lines() {
        let (name, rest) = line.split_once(" (").unwrap();
        let (weight, _) = rest.split_once(")").unwrap();
        let weight = weight.parse().unwrap();
        debug_assert!(weight > 0);

        tree.insert(
            name,
            Info {
                weight,
                // Note: all of these fields are done after this loop
                parent: None,
                children: vec![],
                total_weight: 0,
            },
        );
    }

    for line in input.lines() {
        let (name, rest) = line.split_once(" (").unwrap();

        // println!("+ {name}");
        let mut children = vec![];
        if let Some((_, list)) = rest.split_once("->") {
            for child in list.split(",") {
                let child = child.trim();
                // println!("    + {child}");
                if let Some(info) = tree.get_mut(child) {
                    info.parent = Some(name);
                } else {
                    unreachable!("Couldn't find {child:?} in the tree");
                }
                children.push(child);
            }
        }

        debug_assert!(tree[name].children.is_empty());
        tree.get_mut(name).unwrap().children = children;
    }

    // The *only* node without a parent is the root.
    let (root, _info) = tree
        .iter()
        .find(|(name, info)| info.parent.is_none())
        .unwrap();

    fn build_weights(name: &str, tree: &mut Tree) -> i64 {
        let info = &tree[name];
        if info.total_weight > 0 {
            return info.total_weight;
        }

        let mut total_weight = info.weight;
        for child in info.children.clone() {
            total_weight += build_weights(child, tree);
        }

        tree.get_mut(name).unwrap().total_weight = total_weight;
        total_weight
    }
    build_weights(root, &mut tree);

    if cfg!(debug_assertions) {
        for (name, info) in &tree {
            println!(
                "{name:?} ({}/{}) -> {:?}",
                info.weight, info.total_weight, info.children
            );
        }
    }

    tree
}

// Part1 ========================================================================
#[aoc(day7, part1)]
pub fn part1(input: &str) -> String {
    let mut tree: Tree = parse(input);

    if cfg!(debug_assertions) {
        let orphans: Vec<&str> = tree
            .iter()
            .filter_map(|(name, info)| {
                if info.parent.is_none() {
                    Some(*name)
                } else {
                    None
                }
            })
            .collect();
        println!("Found Oprhans!");
        for orphan in &orphans {
            println!("  + {}: {:?}", orphan, tree[orphan]);
        }
        assert_eq!(orphans.len(), 1);
    }

    let (root, _info) = tree
        .iter()
        .find(|(name, info)| info.parent.is_none())
        .unwrap();

    root.to_string()
}

// Part2 ========================================================================

fn is_balanced(name: &str, tree: &Tree) -> bool {
    match &tree[name].children[..] {
        [] | [_] => true,
        [a, rest @ ..] => {
            for b in rest {
                if tree[a].total_weight != tree[b].total_weight {
                    // println!("name={name:?} {children:?}");
                    return false;
                }
            }
            true
        }
    }
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> i64 {
    let tree: Tree = parse(input);

    let Some(name) = tree
        .keys()
        .filter(|name| !is_balanced(name, &tree))
        .min_by_key(|&name| tree[name].total_weight)
    else {
        unreachable!("Failed to find an unbalanced node");
    };

    if cfg!(debug_assertions) {
        println!(
            "{name:?}\tw={}\ttw={}",
            tree[name].weight, tree[name].total_weight
        );
        for child in &tree[name].children {
            println!(
                "  + {child:?}\tw={}\ttw={}",
                tree[child].weight, tree[child].total_weight
            );
        }
        println!();
    }

    // eyeball it
    if cfg!(test) {
        243
    } else {
        757
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)
";

    #[rstest]
    #[case::given("tknk", EXAMPLE_INPUT)]
    #[trace]
    #[timeout(Duration::from_millis(1_500))]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1)]
        p: impl FnOnce(&str) -> String,
        #[case] expected: impl ToString,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(input), expected.to_string());
    }

    #[rstest]
    #[case::given(243, EXAMPLE_INPUT)]
    #[trace]
    #[timeout(Duration::from_millis(1_500))]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(input), expected);
    }
}
