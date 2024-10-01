#!/bin/bash

rm -rf tools/grub/boot/novusk.*
#mv arch/x86_64/src/boot/kernel.bin tools/grub/boot/novusk.bin
#sh tools/grub/grub.sh
cargo rustc -p x86_64@0.1.0 --lib --crate-type=staticlib --features grub --target targets/x86_64-novusk.json
x86_64-linux-gnu-as -o arch/x86_64/src/boot/grub.o arch/x86_64/src/boot/grub.S
x86_64-linux-gnu-ld -nostdlib -Tarch/x86_64/src/boot/grub.ld -o tools/grub/boot/novusk arch/x86_64/src/boot/grub.o target/x86_64-novusk/debug/liblibx86_64_novusk.a
