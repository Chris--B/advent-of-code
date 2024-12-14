#!/bin/bash

dir=${1-'.'}

echo "using $dir"
set +x

trash out.mp4
ffmpeg -framerate 10 -pattern_type glob -i $dir/'*.png' -c:v libx264 -pix_fmt yuv420p out.mp4
