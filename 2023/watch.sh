#!/bin/bash

if [ "$1" ]; then
    set -xe

    export CARGO_PROFILE_RELEASE_OVERFLOW_CHECKS=true

    cargo clippy --tests
    cargo fmt
    cargo test --lib --quiet
    cargo aoc

    exit 0
fi

set -xe
cargo watch -c -s "sh ./watch.sh doit"
