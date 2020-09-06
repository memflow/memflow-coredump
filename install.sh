#!/bin/bash

cargo build --release --all-features

if [[ "$OSTYPE" == "darwin"* ]]; then
  FILENAME=target/release/libmemflow_coredump.dylib
else
  FILENAME=target/release/libmemflow_coredump.so
fi

if [ ! -z "$1" ] && [ $1 = "--system" ]; then
    if [[ ! -d /usr/lib/memflow ]]; then
        sudo mkdir /usr/lib/memflow
    fi
    sudo cp $FILENAME /usr/lib/memflow
else
    if [[ ! -d ~/.local/lib/memflow ]]; then
        mkdir -p ~/.local/lib/memflow
    fi
    cp $FILENAME ~/.local/lib/memflow
fi
