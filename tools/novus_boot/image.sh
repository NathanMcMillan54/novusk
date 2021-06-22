#!/bin/bash

ARCH=$1
ARCH_FAM=$2

mv ../../target/$ARCH-novusk/debug/novusk novusk.elf
novus_boot novusk.elf $ARCH_FAM-novus-boot.efi $ARCH_FAM
