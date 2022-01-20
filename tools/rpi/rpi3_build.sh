#!/bin/bash

echo "Compiling kernel..."
make all ARCH=aarch64 DEFCONFIG=False CONFIG=arch/aarch64/src/configs/aarch64-rpi3-defconfig.txt
echo "Linking..."
aarch64-linux-gnu-gcc -mcpu=cortex-a53 -fpic -ffreestanding -c arch/arm/src/boot/start/a53_start.S -o arch/arm/src/boot/start/a53_start.o
arm-none-eabi-gcc -T arch/aarch64/src/boot/start/linker.ld -o arch/aarch64/src/boot/kernel7.img -ffreestanding -O2 -nostdlib arch/arm/src/boot/start/a53_start.o arch/aarch64/src/boot/libarmkernel
printf "\n"
echo "Created kernel7.img:"
echo "arch/aarch64/src/boot/kernel7.img"
