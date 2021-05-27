#!/bin/bash

ARCH=$1
DEVICE=$2

if [ "$ARCH" = 'aarch64-uefi' ]; then
    cargo build --target targets/aarch64-uefi-novusk.json --features $DEVICE
elif [ "$ARCH" = 'aarch64' ]; then
    python3 tools/target.py $DEVICE
    cargo xbuild --target=targets/aarch64-novusk.json --features $DEVICE
fi
