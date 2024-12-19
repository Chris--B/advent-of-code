#!/bin/bash


day=$1
echo "New file for day $day"

set -ex
# No dirty state pls
test -z "$(git status --porcelain)"

cp src/day{00,$day}.rs
git add -N src/day$day.rs

cargo aoc input --day $day
gsed -i "s%// pub mod day$day;%pub mod day$day;%g" src/lib.rs
gsed -i "s/dayN/day$day/g" src/day$day.rs

cargo aoc input --day $day
cargo aoc --day $day
