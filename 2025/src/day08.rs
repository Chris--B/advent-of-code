use crate::prelude::*;

// Part1 ========================================================================
#[aoc(day8, part1)]
pub fn part1(input: &str) -> i64 {
    const ITER_COUNT: usize = if cfg!(test) { 10 } else { 1000 };
    let points: Vec<IVec3> = input
        .i64s()
        .tuples()
        .map(|(x, y, z)| IVec3::new(x as _, y as _, z as _))
        .collect_vec();

    let mut distances: HashMap<(IVec3, IVec3), i64> = HashMap::new();
    for (i, &a) in points.iter().enumerate() {
        for &b in &points[i + 1..] {
            let diff = (a - b).as_array().map(|x| x as i64);
            let dist: i64 = diff.map(|x| x * x).iter().sum();
            distances.insert((a, b), dist);
        }
    }

    let mut distances: Vec<((IVec3, IVec3), i64)> = distances
        .iter()
        .map(|(&(a, b), &d)| ((a, b), d))
        .collect_vec();
    distances.sort_by_key(|(_ab, d)| *d);

    let mut circuits = 0;
    let mut circuit_map: HashMap<IVec3, i32> = HashMap::new();

    for ((a, b), _) in distances.into_iter().take(ITER_COUNT) {
        let seen_a = circuit_map.contains_key(&a);
        let seen_b = circuit_map.contains_key(&b);

        if seen_a && seen_b {
            if circuit_map[&a] != circuit_map[&b] {
                let new = i32::min(circuit_map[&a], circuit_map[&b]);
                let old = i32::max(circuit_map[&a], circuit_map[&b]);
                for cir in circuit_map.values_mut() {
                    if *cir == old {
                        *cir = new;
                    }
                }
            }

            continue;
        }

        if seen_a && !seen_b {
            let this_circuit = circuit_map[&a];
            circuit_map.insert(b, this_circuit);
        } else if !seen_a && seen_b {
            let this_circuit = circuit_map[&b];
            circuit_map.insert(a, this_circuit);
        } else {
            let this_circuit = circuits;
            circuits += 1;
            circuit_map.insert(a, this_circuit);
            circuit_map.insert(b, this_circuit);
        }
    }

    let mut reverse_map: HashMap<i32, Vec<IVec3>> = HashMap::new();
    for (&a, &cir) in &circuit_map {
        reverse_map.entry(cir).or_default().push(a);
    }

    if cfg!(test) {
        println!("{circuits} Circuits");
        for cir in 0..circuits {
            if reverse_map.contains_key(&cir) {
                println!(
                    "+[{cir:>2}] ({}) {:?}",
                    reverse_map[&cir].len(),
                    reverse_map[&cir]
                );
            }
        }
    }

    let mut reverse_map = reverse_map.into_iter().collect_vec();
    reverse_map.sort_by_key(|(_, points)| core::cmp::Reverse(points.len()));

    reverse_map[..3]
        .iter()
        .map(|(_, points)| points.len() as i64)
        .product()
}

// Part2 ========================================================================
#[aoc(day8, part2)]
pub fn part2(input: &str) -> i64 {
    let points: Vec<IVec3> = input
        .i64s()
        .tuples()
        .map(|(x, y, z)| IVec3::new(x as _, y as _, z as _))
        .collect_vec();

    let mut distances: HashMap<(IVec3, IVec3), i64> = HashMap::new();

    for (i, &a) in points.iter().enumerate() {
        for &b in &points[i + 1..] {
            let diff = (a - b).as_array().map(|x| x as i64);
            let dist: i64 = diff.map(|x| x * x).iter().sum();
            distances.insert((a, b), dist);
        }
    }

    let mut distances: Vec<((IVec3, IVec3), i64)> = distances
        .iter()
        .map(|(&(a, b), &d)| ((a, b), d))
        .collect_vec();
    distances.sort_by_key(|(_ab, d)| *d);

    let mut circuits = 0;
    let mut circuit_map: HashMap<IVec3, i32> = HashMap::new();

    for ((a, b), _) in distances.into_iter() {
        let seen_a = circuit_map.contains_key(&a);
        let seen_b = circuit_map.contains_key(&b);

        if seen_a && seen_b {
            if circuit_map[&a] != circuit_map[&b] {
                let new = i32::min(circuit_map[&a], circuit_map[&b]);
                let old = i32::max(circuit_map[&a], circuit_map[&b]);
                for cir in circuit_map.values_mut() {
                    if *cir == old {
                        *cir = new;
                    }
                }
            }

            continue;
        } else if seen_a && !seen_b {
            let this_circuit = circuit_map[&a];
            circuit_map.insert(b, this_circuit);
        } else if !seen_a && seen_b {
            let this_circuit = circuit_map[&b];
            circuit_map.insert(a, this_circuit);
        } else {
            let this_circuit = circuits;
            circuits += 1;

            circuit_map.insert(a, this_circuit);
            circuit_map.insert(b, this_circuit);
        }

        let counts = circuit_map.values().tally();
        if counts[&0] == points.len() {
            return a.x as i64 * b.x as i64;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

    #[rstest]
    #[case::given(40, EXAMPLE_INPUT)]
    #[trace]
    #[timeout(Duration::from_millis(2))]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        init_logging();

        let input = input.trim();
        assert_eq!(p(input), expected);
    }

    #[rstest]
    #[case::given(25272, EXAMPLE_INPUT)]
    #[trace]
    #[timeout(Duration::from_millis(2))]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2)]
        p: impl FnOnce(&str) -> i64,
        #[case] expected: i64,
        #[case] input: &str,
    ) {
        init_logging();

        let input = input.trim();
        assert_eq!(p(input), expected);
    }
}
