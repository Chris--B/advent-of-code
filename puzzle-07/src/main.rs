
use std::{
    collections::*,
    iter::FromIterator,
    env,
    fs,
    io::{
        self,
        BufRead,
    },
};

fn main() {
    let run = env::args().nth(1).unwrap_or("1".to_string());
    if run == "1" {
        match run1() {
            Ok(()) => {},
            Err(ref err) => eprintln!("{:?}", err),
        }
    } else if run == "2" {
        match run2() {
            Ok(()) => {},
            Err(ref err) => eprintln!("{:?}", err),
        }
    }
}

#[derive(Eq, PartialEq)]
struct Pair(u32, char);

impl std::cmp::Ord for Pair {
    fn cmp(&self, other: &Pair) -> std::cmp::Ordering {
        if self.0 == other.0 {
            // Smaller letters come first
            other.1.cmp(&self.1)
        } else {
            other.0.cmp(&self.0)
        }
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Pair) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn run1() -> Result<(), failure::Error> {
    let file = fs::File::open("input.txt")?;
    let input = io::BufReader::new(file);

    let re = regex::Regex::new(r"Step ([A-Z]) must be finished before step ([A-Z]) can begin").unwrap();

    let mut all_labels = HashSet::new();
    let mut next = HashMap::<char, Vec<char>>::new();
    let mut prev = HashMap::<char, Vec<char>>::new();

    let mut roots = HashSet::new();
    let mut not_roots = HashSet::new();

    input.lines()
        .map(|line| line.unwrap())
        .filter(|line| line.len() > 0)
        .map(|line| {
            let caps = re.captures(&line).unwrap();
            (caps[1].chars().nth(0).unwrap(), caps[2].chars().nth(0).unwrap())
        }).for_each(|(c, d)| {
            all_labels.insert(c);
            all_labels.insert(d);

            not_roots.insert(d);    // d is obviously not a root
            if roots.contains(&d) { // remove it if we thought it was
                roots.remove(&d);
            }
            if !not_roots.contains(&c) { // we don't think c is a notroot yet
                roots.insert(c);
            }

            // println!("    {} -> {}[label=\"\"]", c, d);
            {
                let mut e = next.entry(c).or_insert(vec![]);
                e.push(d);
            }
            {
                let mut e = prev.entry(d).or_insert(vec![]);
                e.push(c);
            }
        });

    // for (label, children) in next.iter(){
    //     print!("{} -> [", label);
    //     for child in children {
    //         print!(" {}", child);
    //     }
    //     println!(" ]");
    // }

    // for (label, children) in prev.iter(){
    //     print!("{} <- [", label);
    //     for child in children {
    //         print!(" {}", child);
    //     }
    //     println!(" ]");
    // }

    let mut gens = HashMap::new();

    fn populate_gens(label: char,
                     depth: u32,
                     gens: &mut HashMap<char, u32>,
                     next: &HashMap<char, Vec<char>>) {
        if let Some(old_gen) = gens.get(&label) {
            gens.insert(label, (*old_gen).max(depth));
        } else {
            gens.insert(label, depth);
        }
        let children = next
            .get(&label)
            .map(|v| v.as_slice())
            .unwrap_or(&[]);
        for child in children {
            populate_gens(*child, depth+1, gens, next);
        }
    }

    for root in roots {
        populate_gens(root, 0, &mut gens, &next);
    }

    let mut queue = BinaryHeap::from_iter(all_labels.iter()
        .map(|c| {
            Pair(*gens.get(c).unwrap(), *c)
        }));

    let mut ordering = String::new();
    while let Some(Pair(gen, label)) = queue.pop() {
        println!("{}: {}", gen, label);
        ordering.push(label);
    }

    println!("Sleigh Order: {} ({})", ordering, ordering.len());

    Ok(())
}

fn run2() -> Result<(), failure::Error> {
    let file = fs::File::open("input.txt")?;
    let input = io::BufReader::new(file);

    Ok(())
}
// MNQWGKRSFXZJOPCVTYEBLAHIUD
// MNQWGKRSFXZJOPCVTYEBLAHIUD
