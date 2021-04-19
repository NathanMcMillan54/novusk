#!/bin/bash

echo "Cleaning main"
rm -rf novusk.iso
rm -rf tools/iso/grub/boot/bzImage
sh tools/clean/arch.sh
