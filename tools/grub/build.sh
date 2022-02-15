#!/bin/bash

echo "Compiling grub multiboot files..."
nasm -felf64 tools/grub/grub_boot.asm -o tools/grub/grub_boot
nasm -felf64 tools/grub/long_mode_init.asm -o tools/grub/long_mode_init
nasm -felf64 tools/grub/multiboot_header.asm -o tools/grub/multiboot_header
echo "Linking..."
ld -n --gc-section -T arch/x86_64/src/boot/linker.ld -o tools/grub/boot/novusk tools/grub/grub_boot tools/grub/multiboot_header tools/grub/long_mode_init arch/x86_64/target/x86_64-novusk/release/libx86_64_kernel_lib.a include/novusk/kms/*.km
sh tools/grub/grub.sh
