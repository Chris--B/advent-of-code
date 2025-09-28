use crate::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Info<'a> {
    name: &'a str,
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
                name,
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
        .find(|(_name, info)| info.parent.is_none())
        .unwrap();

    fn build_weights(name: &str, tree: &mut Tree) -> i64 {
        let info = &tree[name];
        if info.total_weight > 0 {
            return info.total_weight;
        }

        let mut total_weight = info.weight;
        let mut children = info.children.clone();
        for &child in &children {
            total_weight += build_weights(child, tree);
        }

        // Since the children are now fully weighted, we can sort them to simplify later checks
        children.sort_by_key(|c| tree[c].total_weight);

        let info: &mut Info = tree.get_mut(name).unwrap();
        info.total_weight = total_weight;
        info.children = children;

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
    let tree: Tree = parse(input);
    tree.iter()
        .find(|(_name, info)| info.parent.is_none())
        .map(|(name, _info)| name.to_string())
        .unwrap()
}

// Part2 ========================================================================
fn is_balanced(name: &str, tree: &Tree) -> bool {
    overweight_child(name, tree).is_none()
}

fn overweight_child<'a>(name: &'_ str, tree: &'a Tree) -> Option<(&'a str, i64)> {
    // We keep these children sorted by total_weight.
    // Since exactly 0 or exactly 1 child is the "wrong" weight, and these are sorted,
    // it must be the first or last child that is off.
    // We will assume the last, since the weights are always *too much* (???? unverified?)
    let first = tree[name].children.first()?;
    let last = tree[name].children.last()?;

    let diff = tree[last].total_weight - tree[first].total_weight;
    if diff != 0 {
        Some((last, diff))
    } else {
        None
    }
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> i64 {
    let tree: Tree = parse(input);
    let name = tree
        .keys()
        .filter(|name| !is_balanced(name, &tree))
        .min_by_key(|&name| tree[name].total_weight)
        .expect("Failed to find an unbalanced node");

    let (bad_apple, diff) = overweight_child(name, &tree).unwrap();
    tree[bad_apple].weight - diff
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Info2 {
    weight: i64,
    parent: Option<KeyId>,
    children: Vec<KeyId>,
    total_weight: i64,
}

type Tree2<'a> = LookupMap<&'a str, Info2>;

fn parse_lum<'a>(input: &'a str) -> Tree2<'a> {
    let mut tree: Tree2 = Tree2::new();

    // Read everyone into the tree first
    // This makes reasoning about children in the next loop easier
    for line in input.lines() {
        let (name, rest) = line.split_once(" (").unwrap();
        let (weight, _) = rest.split_once(")").unwrap();
        let weight = weight.parse().unwrap();
        debug_assert!(weight > 0);

        let info: &mut Info2 = &mut tree.new_entry(name).value;
        info.weight = weight;
    }

    for (id, line) in input.lines().enumerate() {
        let id = KeyId(id);
        let (name, rest) = line.split_once(" (").unwrap();
        debug_assert_eq!(tree.entry(id).key, name);

        // println!("+ {name}");
        let mut children = vec![];
        if let Some((_, list)) = rest.split_once("->") {
            for child in list.split(",") {
                let child = child.trim();
                // println!("    + {child}");
                let child_entry = tree.insert_or_entry(child);
                child_entry.value.parent = Some(id);

                children.push(child_entry.id);
            }
        }

        debug_assert!(tree.entry(id).value.children.is_empty());
        tree.entry(id).value.children = children;
    }

    // The *only* node without a parent is the root.
    let root = tree
        .entries()
        .find(|e| e.value.parent.is_none())
        .map(|e| e.id)
        .unwrap();

    fn build_weights(id: KeyId, tree: &mut Tree2) -> i64 {
        let info: &Info2 = &tree.entry(id).value;
        if info.total_weight > 0 {
            return info.total_weight;
        }

        let mut total_weight = info.weight;
        let mut children = info.children.clone();
        for &child in &children {
            total_weight += build_weights(child, tree);
        }

        // Since the children are now fully weighted, we can sort them to simplify later checks
        children.sort_by_key(|&c| tree.entry(c).value.total_weight);

        let info: &mut Info2 = &mut tree.entry(id).value;
        info.total_weight = total_weight;
        info.children = children;

        total_weight
    }
    build_weights(root, &mut tree);

    if cfg!(debug_assertions) {
        for entry in tree.entries() {
            let name = entry.key;
            let info = &entry.value;
            let children = info.children.iter().map(|&id| tree.key(id)).collect_vec();
            println!(
                "{name:?} ({}/{}) -> {:?}",
                info.weight, info.total_weight, children
            );
        }
    }

    tree
}

#[aoc(day7, part1, lum)]
pub fn part1_lum(input: &str) -> String {
    let tree = parse_lum(input);
    let root = tree
        .entries()
        .find(|&e| e.value.parent.is_none())
        .map(|e| e.id)
        .unwrap();

    tree.key(root).to_string()
}

fn is_balanced_lum(id: KeyId, tree: &Tree2) -> bool {
    overweight_child_lum(id, tree).is_none()
}

fn overweight_child_lum(id: KeyId, tree: &Tree2) -> Option<(KeyId, i64)> {
    // We keep these children sorted by total_weight.
    // Since exactly 0 or exactly 1 child is the "wrong" weight, and these are sorted,
    // it must be the first or last child that is off.
    // We will assume the last, since the weights are always *too much* (???? unverified?)
    let first = *tree.value(id).children.first()?;
    let last = *tree.value(id).children.last()?;

    let diff = tree.value(last).total_weight - tree.value(first).total_weight;
    if diff != 0 {
        Some((last, diff))
    } else {
        None
    }
}

#[aoc(day7, part2, lum)]
pub fn part2_lum(input: &str) -> i64 {
    let tree = parse_lum(input);
    let id = tree
        .ids()
        .filter(|&id| !is_balanced_lum(id, &tree))
        .min_by_key(|&id| tree.value(id).total_weight)
        .expect("Failed to find an unbalanced node");

    let (bad_apple, diff) = overweight_child_lum(id, &tree).unwrap();
    tree.value(bad_apple).weight - diff
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
        #[values(part1, part1_lum)]
        p: impl FnOnce(&str) -> String,
        #[case] expected: impl ToString,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(input), expected.to_string());
    }

    #[rstest]
    #[case::given(60, EXAMPLE_INPUT)]
    #[trace]
    #[timeout(Duration::from_millis(1_500))]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2, part2_lum)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        let input = input.trim();
        assert_eq!(p(input), expected);
    }
}
