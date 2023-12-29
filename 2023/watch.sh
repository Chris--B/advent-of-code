#!/bin/bash

if [ "$1" ]; then
    set -xe

    export CARGO_PROFILE_RELEASE_OVERFLOW_CHECKS=true

    cargo clippy --tests
    cargo fmt
    cargo test --lib --quiet day11
    cargo aoc

    # neato target/day25.dot -Tsvg -o day25.svg;

    exit 0
fi

set -xe
cargo doc --document-private-items
cargo watch -c -s "sh ./watch.sh doit"
