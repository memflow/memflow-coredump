#!/bin/bash

cargo build --release --all-features
if [[ ! -d ~/.local/lib/memflow ]]; then
    mkdir -p ~/.local/lib/memflow
fi
cp target/release/libmemflow_coredump.so ~/.local/lib/memflow
