use aoc_runner_derive::{aoc, aoc_generator};

use std::collections::HashMap;

use smallstr::SmallString;
use smallvec::SmallVec;

type String = SmallString<[u8; 64]>;
type Entries<'a> = SmallVec<[Entry<'a>; 32]>;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Entry<'a> {
    Dir(&'a str),
    File(&'a str, u32),
}
use Entry::*;

fn join_dirs(dirs: &[&str]) -> String {
    let mut path = String::new();

    if let Some(dir) = dirs.first() {
        path.push_str(dir);
    }

    for dir in dirs.iter().skip(1) {
        path.push('/');
        path.push_str(dir);
    }

    if path.starts_with('/') {
        path.remove(0);
    }
    path
}

#[aoc_generator(day7)]
fn parse_input(input: &str) -> HashMap<String, u32> {
    build_sizes_list(input)
}

fn build_sizes_list(input: &str) -> HashMap<String, u32> {
    let mut fs: HashMap<String, Entries> = HashMap::new();
    let mut dir_stack: SmallVec<[&str; 32]> = SmallVec::new();

    #[derive(Debug)]
    enum Cmd {
        Ls,
        Cd,
    }

    let mut cmd = None;

    for line in input.lines() {
        if line.starts_with('$') {
            // execute command
            match &line[2..4] {
                "cd" => {
                    cmd = Some(Cmd::Cd);

                    match &line[5..] {
                        "/" => {
                            dir_stack.clear();
                        }
                        ".." => {
                            dir_stack.pop();
                        }
                        dir => {
                            let path = join_dirs(&dir_stack);
                            let entry = fs.entry(path).or_default();
                            entry.sort();
                            if !entry.contains(&Dir(dir)) {
                                entry.push(Dir(dir));
                            }

                            dir_stack.push(dir);
                        }
                    }
                }
                "ls" => {
                    cmd = Some(Cmd::Ls);
                }
                cmd => {
                    unreachable!("Unknown command '{cmd}' from line \"{line}\"");
                }
            }
        } else {
            // processing output from previous command
            match cmd {
                None => unreachable!("Processing output, but there was no command"),
                Some(Cmd::Cd) => unreachable!("Processing output from cd, but cd doesn't output"),
                Some(Cmd::Ls) => {
                    let entry = if line.starts_with("dir") {
                        Dir(&line[4..])
                    } else {
                        let mut iter = line.split_whitespace();
                        let size: u32 = iter.next().unwrap().parse().unwrap();
                        let name = iter.next().unwrap();

                        File(name, size)
                    };

                    let path = join_dirs(&dir_stack);
                    fs.entry(path).or_default().push(entry);
                }
            }
        }
    }

    if cfg!(debug_assertions) {
        println!("{} entries", fs.len());
        for (path, entries) in &fs {
            print!("- {path}: ");

            print!("[");
            for entry in entries {
                match entry {
                    Dir(dir) => print!("{dir}/, "),
                    File(name, _) => print!("{name}, "),
                }
            }
            print!("]");

            println!();
        }
    }

    fn just_do_it<'a>(
        fs: &'a HashMap<String, Entries>,
        sizes: &'a mut HashMap<String, u32>,
        prefix: &'a str,
        entry: Entry,
    ) -> u32 {
        match entry {
            File(_name, size) => {
                // Should have been created by calling code
                *sizes.get_mut(prefix).unwrap() += size;
                size
            }
            Dir(dir) => {
                let path = join_dirs(&[prefix, dir]);

                // Create our directory if it wasn't already
                sizes.entry(path.clone()).or_insert(0);

                let mut size = 0;
                for entry in &fs[&path] {
                    // Update our Size recursively
                    size += just_do_it(fs, sizes, &path, *entry);
                }

                // Update our parent - should already exists
                *sizes.get_mut(prefix).unwrap() += size;

                size
            }
        }
    }

    // Cumulative size for each directory in fs.
    let mut sizes: HashMap<String, u32> = HashMap::new();
    sizes.insert("".into(), 0);

    for entry in &fs[""] {
        just_do_it(&fs, &mut sizes, "", *entry);
    }

    if cfg!(debug_assertions) {
        println!("{} entries", fs.len());
        for (path, size) in &sizes {
            println!("- {path}: {size}");
        }
    }

    sizes
}

// Part1 ========================================================================
#[aoc(day7, part1)]
#[inline(never)]
pub fn part1(sizes: &HashMap<String, u32>) -> u32 {
    sizes.values().filter(|size| **size <= 100_000).sum()
}

// Part2 ========================================================================
#[aoc(day7, part2)]
#[inline(never)]
pub fn part2(sizes: &HashMap<String, u32>) -> u32 {
    const TOTAL_SPACE: u32 = 70_000_000;
    const UNUSED_TARGET: u32 = 30_000_000;

    let current_unused = TOTAL_SPACE - sizes[""];
    let need_to_delete = UNUSED_TARGET - current_unused;

    sizes
        .values()
        .copied()
        .filter(|size| *size >= need_to_delete)
        .min_by_key(|size| *size)
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    const EXAMPLE_INPUT: &str = r"
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

    const NO_DIRS_INPUT: &str = r"
$ cd /
$ ls
100 a
200 b
";

    const ONE_DIR_INPUT: &str = r"
$ cd /
$ ls
100 a
200 b
dir c
$ cd c
$ ls
300 d
";

    #[rstest]
    #[case::given(94853 + 584, EXAMPLE_INPUT)]
    #[case::no_dirs(300, NO_DIRS_INPUT)]
    #[case::files_and_1_dir(900, ONE_DIR_INPUT)]
    #[trace]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1)]
        p: impl FnOnce(&HashMap<String, u32>) -> u32,
        #[case] expected: u32,
        #[case] input: &str,
    ) {
        let input = input.trim_start();
        assert_eq!(p(&parse_input(input)), expected);
    }

    #[rstest]
    #[case::given(24933642, EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_2(
        #[notrace]
        #[values(part2)]
        p: impl FnOnce(&HashMap<String, u32>) -> u32,
        #[case] expected: u32,
        #[case] input: &str,
    ) {
        let input = input.trim_start();
        assert_eq!(p(&parse_input(input)), expected);
    }
}
