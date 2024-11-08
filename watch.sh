#!/bin/bash

if [ "$#" -eq 2 ] && [ "$2" == "looping" ]; then
    day=$1
    echo $PWD
    set -xe

    export RUST_BACKTRACE=1
    export CARGO_PROFILE_RELEASE_OVERFLOW_CHECKS=true

    cargo clippy --tests
    cargo fmt
    cargo test --lib --quiet day$day
    cargo aoc --day $day

    # cargo doc --document-private-items

elif [ "$#" -eq 1 ]; then
    set -xe

    cargo watch -c -s "sh $(realpath $BASH_SOURCE) $1 looping"
else
    echo "Usage: $(realpath $BASH_SOURCE) DAY"
    echo "    Where DAY is 1..=25"
    echo ""
    echo "For example, to run day16 do:"
    echo "    $(realpath $BASH_SOURCE) 16"
fi
