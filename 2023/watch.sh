#!/bin/bash

day=17

if [ "$1" ]; then
    set -xe

    export CARGO_PROFILE_RELEASE_OVERFLOW_CHECKS=true

    cargo clippy --tests
    cargo fmt
    # cargo test --lib --quiet day$day -- --nocapture

    cargo test --release --lib day17 -- --nocapture || true

    # set +e
    # trash day17_test.mp4
    # ffmpeg -pattern_type glob -framerate 10/1 -i 'target/day17_test_141/*.png' -c:v libx264 -r 30 -pix_fmt yuv420p day17_test.mp4
    # open day17_test.mp4

    # set +xe
    # for test in  "day17_test_10x10" "day17_test_11x11" "day17_test_12x12" "day17_test_13x13" "day17_test_2x2" "day17_test_3x3" "day17_test_4x4" "day17_test_5x5" "day17_test_6x6" "day17_test_7x7" "day17_test_8x8" "day17_test_9x9"; do
    #     trash $test.mp4
    #     ffmpeg -pattern_type glob -framerate 10/1 -i "target/$test/*.bmp" -c:v libx264 -r 30 -pix_fmt yuv420p $test.mp4
    # done

    # cargo aoc --day $day
    # cargo doc --document-private-items

    exit 0
fi

set -xe
cargo watch -c -s "sh ./watch.sh doit"
