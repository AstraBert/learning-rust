#!/bin/bash

for dir in */; do
    if [ -f "${dir}/1/Cargo.toml" ]; then
        echo "Building $dir"
        (cd "${dir}/1" && cargo build) || exit 1
    fi
done