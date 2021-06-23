#!/bin/bash

ARCH=$1

mv ../../Novusk boot/novusk
grub-mkrescue -o boot/novusk.iso ../grub/
