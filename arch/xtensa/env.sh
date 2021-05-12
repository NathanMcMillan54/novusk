#!/bin/bash

CUSTOM_RUSTC=tools/rust-xtensa

export RUST_BACKTRACE=1
export XARGO_RUST_SRC=$CUSTOM_RUSTC/library
export RUSTC=$CUSTOM_RUSTC/build/x86_64-unknown-linux-gnu/stage2/bin/rustc
export RUSTDOC=$CUSTOM_RUSTC/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc
