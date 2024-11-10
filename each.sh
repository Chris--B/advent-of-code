#!/bin/bash

cd $(dirname $(realpath $BASH_SOURCE))
pwd

for y in $(find * -maxdepth 1 -type d -name "20*")
do
    pushd $y > /dev/null

    echo "Advent of Code $y https://adventofcode.com/$y/"
    $*
    popd     > /dev/null
done

for d in $(find * -maxdepth 1 -type f -name Cargo.toml | grep -v "20")
do
    pushd $y > /dev/null

    echo $(dirname $d)
    $*

    popd     > /dev/null
done
