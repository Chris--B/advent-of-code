#!/bin/bash

set +e

# Build everything
echo "Building!"
for y in $(find * -maxdepth 1 -type d -name "20*")
do
    pushd $y > /dev/null

    echo "    Advent of Code $y https://adventofcode.com/$y/"
    cargo update
    cargo build --release
    echo ""

    popd     > /dev/null
done

# Run everything
echo "Running!"
for y in $(find * -maxdepth 1 -type d -name "20*")
do
    pushd $y > /dev/null

    echo "    Advent of Code $y https://adventofcode.com/$y/"
    RUST_BACKTRACE=1 timeout 3s cargo run --release > target/output_$y.txt
    ret=$?
    if [[ "$ret" -eq "124" ]]; then
        echo "[!!!] AOC $y timed out"
    fi
    echo "    Run logs in $y/target/output_$y.txt"
    echo ""

    popd     > /dev/null
done
