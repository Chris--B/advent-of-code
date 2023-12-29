#!/bin/bash

if [ "$1" ]; then
    set -xe

    export CARGO_PROFILE_RELEASE_OVERFLOW_CHECKS=true

    cargo clippy --tests
    cargo fmt
    cargo test --lib day12
    cargo aoc

    # neato target/day25.dot -Tsvg -o day25.svg;

    exit 0
fi

set -xe
cargo watch -c -s "sh ./watch.sh doit"
