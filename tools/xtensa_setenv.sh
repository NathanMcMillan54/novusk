#!/bin/bash

# Using https://github.com/esp-rs/rust
RUSTC_PATH=$1

cd $RUSTC_PATH && pwd
export RUST_BACKTRACE=1
export XARGO_RUST_SRC=$RUSTC_PATH/library
export RUSTC=$RUSTC_PATH/build/x86_64-unknown-linux-gnu/stage2/bin/rustc
export RUSTDOC=$RUSTC_PATH/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc
rustup toolchain link esp $RUSTC_PATH/build/x86_64-unknown-linux-gnu/stage2
rustup default esp
echo "To reset rustup, run 'rustup toolchain stable'"
