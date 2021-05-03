#!/bin/bash

ARCH=$1

if [ "$ARCH" = 'aarch64-uefi' ]; then
    ./disk_img target/aarch64-uefi-novusk/debug/aarch64.efi
    cp -r target/aarch64-uefi-novusk/debug/aarch64.fat src/boot/Image
elif [ "$ARCH" = 'aarch64' ]; then
    cp -r target/aarch64-novusk/debug/aarch64 src/boot/Image
else
  echo "$ARCH is invalid"
fi
