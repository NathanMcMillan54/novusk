#!/bin/bash

as arch/x86_64/src/boot/grub.S -o arch/x86_64/src/boot/grub.o
gcc -T arch/x86_64/src/boot/grub.ld -o target/x86_64-novusk/debug/novusk_kernel_grubb -ffreestanding -nostdlib arch/x86_64/src/boot/grub.o target/x86_64-novusk/debug/novusk_kernel -lgcc
