#!/bin/bash

ARCH=$1
DEVICE=$2

if [ "$ARCH" = 'aarch64-uefi' ]; then
    cargo build --target targets/aarch64-uefi-novusk.json --features $DEVICE -Z build-std=core -Z build-std-features=compiler-builtins-mem
elif [ "$ARCH" = 'aarch64' ]; then
    python3 tools/target.py $DEVICE
    cargo xbuild --target=targets/aarch64-novusk.json --features board_$DEVICE
fi
