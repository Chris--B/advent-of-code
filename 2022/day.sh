#!/bin/bash

set -e

d=$(TZ="America/Boston" date +%d)
dd=$(TZ="America/Boston" date +%0d)

cp -v src/day00.rs src/day$dd.rs
sed "s/dayN/day$d/" -i src/day$dd.rs
cargo aoc input
