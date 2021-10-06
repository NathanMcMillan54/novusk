#!/bin/bash

echo "Compiling grub multiboot files..."
nasm -felf64 tools/grub/grub_boot.asm -o tools/grub/grub_boot
nasm -felf64 tools/grub/long_mode_init.asm -o tools/grub/long_mode_init
nasm -felf64 tools/grub/multiboot_header.asm -o tools/grub/multiboot_header
echo "Compiling kernel..."
cargo build --lib --features bios_boot,grub --target targets/x86_64-novusk.json
echo "Linking..."
ld -n --gc-section -T arch/x86_64/src/boot/grub.ld -o tools/grub/boot/novusk tools/grub/grub_boot tools/grub/long_mode_init tools/grub/multiboot_header target/x86_64-novusk/debug/libnovusk.a
sh tools/grub/grub.sh