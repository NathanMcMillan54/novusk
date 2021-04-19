#!/bin/bash

cp -r arch/x86_64/src/boot/bzImage tools/iso/boot
grub-mkrescue -o novusk.iso tools/iso/
