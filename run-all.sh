#!/bin/bash

set +e

# Build everything
echo "Building!"
for d in $(find * -type d -name "20*" -maxdepth 1)
do
    pushd $d > /dev/null

    echo "    Advent of Code $d https://adventofcode.com/$d/"
    cargo build --release
    echo ""

    popd     > /dev/null
done

# Run everything
echo "Running!"
for d in $(find * -type d -name "20*" -maxdepth 1)
do
    pushd $d > /dev/null

    echo "    Advent of Code $d https://adventofcode.com/$d/"
    RUST_BACKTRACE=1 timeout 3s cargo run --release > target/output_$d.txt
    ret=$?
    if [[ "$ret" -eq "124" ]]; then
        echo "[!!!] AOC $d timed out"
    fi
    echo "    Run logs in $d/target/output_$d.txt"
    echo ""

    popd     > /dev/null
done