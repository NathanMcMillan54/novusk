#!/bin/bash

ARCH=$1
ARCH_FAM=$2
BOOT=$3

if [ "$BOOT" = 'uefi' ]; then
    novus_boot target/$ARCH-novusk-uefi/debug/novusk.efi $ARCH_FAM
    mv target/$ARCH-novusk-uefi/debug/novusk.efi Novusk
elif [ "$BOOT" = 'bios' ]; then
    cargo bootimage --target targets/x86_64-novusk-bios.json
    mv target/x86_64-novusk-bios/debug/bootimage-novusk.bin Novusk
elif [ "$BOOT" = 'no' ]; then
    mv target/$ARCH-novusk/debug/novusk Novusk
fi
