#!/bin/bash

cd kernel/ && make kernel_main && cd ../
ld -relocatable arch/x86_64/src/boot/bzImage kernel/main.o -o tools/iso/bzImage
grub-mkrescue -o novusk.iso tools/iso/
