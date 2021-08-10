#!/bin/bash

mkdir exclude
echo "Installing Raspberry Pi boot files from https://raw.githubusercontent.com/raspberrypi/firmware/..."
curl https://raw.githubusercontent.com/raspberrypi/firmware/master/boot/bootcode.bin > exclude/bootcode.bin
curl https://raw.githubusercontent.com/raspberrypi/firmware/master/boot/fixup.dat > exclude/fixup.dat
curl https://raw.githubusercontent.com/raspberrypi/firmware/master/boot/start.elf > exclude/start.elf
echo "Installed and moved bootcode.bin, fixup.dat, and start.elf to exclude/"
