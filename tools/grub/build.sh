#!/bin/bash

cargo build --features bios_boot,grub --target targets/x86_64-novusk.json
as arch/x86_64/src/boot/grub.S -o arch/x86_64/src/boot/grub.o
ld -Tarch/x86_64/src/boot/grub.ld arch/x86_64/src/boot/grub.o target/x86_64-novusk/debug/novusk_kernel -o tools/grub/boot/novusk
