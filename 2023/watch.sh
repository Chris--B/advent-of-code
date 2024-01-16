#!/bin/bash

day=17

if [ "$1" ]; then
    set -xe

    export CARGO_PROFILE_RELEASE_OVERFLOW_CHECKS=true

    cargo clippy --tests
    cargo fmt
    # cargo test --lib --quiet day$day -- --nocapture

    mkdir -p target/day17_test/
    trash    target/day17_test/
    cargo test --release --lib day17 -- --nocapture || true

    # set +e
    # trash day17_test.mp4
    # ffmpeg -pattern_type glob -framerate 10/1 -i 'target/day17_test/*.png' -c:v libx264 -r 30 -pix_fmt yuv420p day17_test.mp4
    # open day17_test.mp4

    # cargo aoc --day $day
    # cargo doc --document-private-items

    exit 0
fi

set -xe
cargo watch -c -s "sh ./watch.sh doit"
