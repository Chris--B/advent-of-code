#!/bin/bash

set -e

d=$(date +%d)
dd=$(date +%0d)

cp -v src/day00.rs src/day$dd.rs
sed "s/dayN/day$d/" -i src/day$dd.rs
cargo aoc input
