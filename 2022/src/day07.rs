use aoc_runner_derive::aoc;

use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Entry<'a> {
    Dir(&'a str),
    File(&'a str, u32),
}
use Entry::*;

// Part1 ========================================================================
#[aoc(day7, part1)]
#[inline(never)]
pub fn part1(input: &str) -> u32 {
    let mut fs: HashMap<String, Vec<Entry<'_>>> = HashMap::new();
    let mut dir_stack: Vec<&str> = vec![];

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
                            let path = dir_stack.join("/");
                            let entry: &mut Vec<_> = fs.entry(path).or_default();
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

                    let path = dir_stack.join("/");
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
        fs: &'a HashMap<String, Vec<Entry>>,
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
                let path = format!("{prefix}/{dir}");
                let path = path.trim_start_matches('/').to_string();

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
    sizes.insert("".to_string(), 0);

    for entry in &fs[""] {
        just_do_it(&fs, &mut sizes, "", *entry);
    }

    if cfg!(debug_assertions) {
        println!("{} entries", fs.len());
        for (path, size) in &sizes {
            println!("- {path}: {size}");
        }
    }

    sizes.values().filter(|size| **size <= 100_000).sum()
}

// Part2 ========================================================================
#[aoc(day7, part2)]
#[inline(never)]
pub fn part2(_input: &str) -> u32 {
    unimplemented!();
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

    #[rstest]
    #[case::given(94853 + 584, EXAMPLE_INPUT)]
    #[trace]
    fn check_ex_part_1(
        #[notrace]
        #[values(part1)]
        p: impl FnOnce(&str) -> u32,
        #[case] expected: u32,
        #[case] input: &str,
    ) {
        let input = input.trim_start();
        assert_eq!(p(input), expected);
    }

    // #[rstest]
    // #[case::given(999_999, EXAMPLE_INPUT)]
    // #[trace]
    // fn check_ex_part_2(
    //     #[notrace]
    //     #[values(part2)]
    // p: impl FnOnce(&str) -> u32,
    //     #[case] expected: u32,
    //     #[case] input: &str,
    // ) {
    //     let input = input.trim_start();
    //     assert_eq!(p(input), expected);
    // }
}
