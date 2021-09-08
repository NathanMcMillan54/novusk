#!/bin/bash

as -o arch/x86_64/src/boot/grub.o arch/x86_64/src/boot/grub.S
gcc -T arch/x86_64/src/boot/grub.ld -o target/x86_64-novusk/debug/novusk_kernel_grubb -ffrestanding -02 -nostdlib arch/x86_64/src/boot/grub.o target/x86_64-novusk/debug/novusk_kernel -lgcc
