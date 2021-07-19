#!/bin/bash

LINKER="arch/aarch64/src/boot/linker.ld"

export LINKER

export RUSTFLAGS="-C link-arg=-T$LINKER -C target-cpu=cortex-a53"

cargo rustc --target targets/aarch64-none-novusk.json --features aarch64/rpi3 --release
rust-objcopy --strip-all -O binary target/aarch64-none-novusk/release/novusk_kernel Image
