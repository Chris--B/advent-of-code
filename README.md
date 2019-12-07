# Advent of Code

Advent of Code is an [advent calendar]() where each day involves a ~~small~~ coding puzzle. This repo contains my solutions for Advent of Code puzzles over the years.

Some of these puzzles are incomplete, missing, or wrong. Some of the code here horrifies me, and some of it I'm quite proud of. It's a bit of a zoo.

For more information about this annual event, see it's website:
- https://adventofcode.com/2019/about
- https://adventofcode.com/

#### Puzzle Links over the years
- https://adventofcode.com/2019
- https://adventofcode.com/2018
- https://adventofcode.com/2017
- https://adventofcode.com/2016
- https://adventofcode.com/2015

### Info
All solutions here use the `cargo-aoc` framework for running and benchmarking the puzzles. To get started using it, consult it's README in its repo [here](https://github.com/gobanos/cargo-aoc).

I use `cargo-aoc 0.2.0` installed locally. There may be issues if you mis-match versions of the library (pulled by Cargo during builds) and the runner (isntalled with `cargo install`).

### Building
Each year lives in isolation and must be built from its directory.
```
$ cd 2019
$ cargo test
$ cargo aoc
$ cargo aoc -d5 # Run just Day5
```

I save my input files in the repo to keep things simple. If you would like to
test my code with your input, you can overwrite them in-place without issue.
```
$ cd 2019
$ cargo aoc input -d5
$ cargo aoc -d5
```

`cargo-aoc` defaults to the current year when fetching input, so keep that in mind when running older puzzles
```
$ cd 2018
$ cargo aoc input -y 2018 d3
$ cargo aoc -d5
```

### Benchmarking

Please consult [`cargo-aoc`](https://github.com/gobanos/cargo-aoc) for details on using its benchmarking feature. This repo should cooperate with it without issue. Please let me know if it does not.
