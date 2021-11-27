#!/bin/bash

echo "Compiling kernel..."
cargo build --lib --release --features rpi2 --target targets/arm-a-novusk.json
echo "Linking..."
arm-none-eabi-gcc -mcpu=cortex-a7 -fpic -ffreestanding -c arch/arm32/src/boot/start/a7_start.S -o arch/arm32/src/boot/start/a7_start.o
arm-none-eabi-gcc -T arch/arm32/src/boot/start/linker.ld -o target/arm-a-novusk/release/kernel7.img -ffreestanding -O2 -nostdlib arch/arm32/src/boot/start/a7_start.o target/arm-a-novusk/release/libnovusk.a
printf "\n"
echo "Created kernel7.img:"
echo "target/arm-a-novusk/release/kernel7.img"
