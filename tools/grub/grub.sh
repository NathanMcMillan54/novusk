#!/bin/bash

ARCH=$1

mv ../../target/$ARCH-novusk/debug/novusk boot/
grub-mkrescue -o boot/novusk.iso ../grub/
