#!/bin/bash

rm -rf tools/grub/boot/novusk.*
mv arch/x86_64/src/boot/kernel.bin tools/grub/boot/novusk.bin
sh tools/grub/grub.sh
